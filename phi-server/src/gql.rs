use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};

pub mod model;

pub async fn index(schema: web::Data<model::PokerSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_ws(
    schema: web::Data<model::PokerSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/gql").subscription_endpoint("/gql"),
        )))
}
