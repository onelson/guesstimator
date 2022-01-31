use crate::gql::model::PokerSchema;
use crate::poker::DeckType;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use std::convert::Infallible;
use uuid::Uuid;
use warp::{http::Response, Filter};

mod gql;
mod poker;

#[cfg(feature = "baked")]
mod spa {
    use include_dir::{include_dir, Dir};
    use std::convert::Infallible;
    use warp::path::Tail;
    use warp::{http::Response, Filter, Reply};

    static SPA_FILES: Dir = include_dir!("$PHI_STATIC_DIR");
    pub fn handler() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
        println!("Using baked assets");
        warp::path::tail().map(|tail: Tail| {
            let file = SPA_FILES
                .get_file(tail.as_str())
                .or_else(|| SPA_FILES.get_file("index.html"))
                .expect("get baked file");
            let mime = mime_guess::from_path(file.path()).first_or_octet_stream();
            Response::builder()
                .header("content-type", mime.as_ref())
                .body(file.contents())
        })
    }
}

#[cfg(not(feature = "baked"))]
mod spa {
    use std::path::PathBuf;
    use warp::{Filter, Rejection, Reply};

    pub fn handler() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let static_dir: PathBuf = std::env::var("PHI_STATIC_DIR")
            .ok()
            .map(Into::into)
            .unwrap_or_else(|| PathBuf::from("."));

        println!("Using Static Dir: {:?}", &static_dir);

        warp::fs::dir(static_dir)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let admin_key = Uuid::new_v4();

    let deck_type = std::env::var("PHI_DECK_TYPE")
        .map_or_else(
            |_| Ok(DeckType::Fibonacci),
            |s| match s.as_str() {
                "fib" | "" => Ok(DeckType::Fibonacci),
                "days" => Ok(DeckType::Days),
                _ => Err(format!("Invalid deck type: `{}`. Use `fib` or `days`.", s)),
            },
        )
        .expect("PHI_DECK_TYPE");

    println!("\nAdmin Key: {}\n", &admin_key);

    let schema = Schema::build(
        gql::model::Query,
        gql::model::Mutation,
        gql::model::Subscription,
    )
    .data(poker::PlaySession::new(admin_key, deck_type))
    .finish();

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (PokerSchema, async_graphql::Request)| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/gql-playground").subscription_endpoint("/gql"),
            ))
    });

    let log = warp::log("phi_server");

    let routes = warp::path("gql")
        .and(graphql_subscription(schema).or(graphql_post))
        .or(warp::path("gql-playground").and(graphql_playground))
        // FIXME: look at baking the assets into the binary.
        .or(spa::handler());
    warp::serve(routes.with(log))
        .run(([0, 0, 0, 0], 7878))
        .await;
}
