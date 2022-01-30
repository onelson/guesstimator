use crate::gql::model::PokerSchema;
use crate::poker::DeckType;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use std::convert::Infallible;
use std::path::PathBuf;
use uuid::Uuid;
use warp::{http::Response as HttpResponse, Filter};

mod gql;
mod poker;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let admin_key = Uuid::new_v4();
    let static_dir: PathBuf = std::env::var("PHI_STATIC_DIR")
        .ok()
        .map(Into::into)
        .unwrap_or_else(|| PathBuf::from("."));

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

    println!("\nUsing Static Dir: {:?}\n", &static_dir);
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
        HttpResponse::builder()
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
        .or(warp::fs::dir(static_dir));
    warp::serve(routes.with(log))
        .run(([0, 0, 0, 0], 7878))
        .await;
}
