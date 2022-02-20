use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

pub mod model;

async fn index(schema: web::Data<model::PokerSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_ws(
    schema: web::Data<model::PokerSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/gql").subscription_endpoint("/gql"),
        )))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/gql").guard(guard::Post()).to(index))
        .service(
            web::resource("/gql")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(
            web::resource("/gql-playground")
                .guard(guard::Get())
                .to(index_playground),
        );
}
