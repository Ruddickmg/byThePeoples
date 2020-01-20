pub mod schema;

use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use schema::{create_schema, Schema};

async fn graph_iql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graph_ql(
    st: web::Data<super::AppData>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st.schema, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

pub fn graph_schema() -> Arc<Schema> {
    std::sync::Arc::new(create_schema())
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(graph_ql)))
        .service(web::resource("/graphiql").route(web::get().to(graph_iql)));
}
