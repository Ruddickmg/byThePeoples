use crate::connection;
use actix_web::{web, Error, HttpResponse};
use juniper::http::GraphQLRequest;
use juniper::http::{graphiql, GraphQLResponse};

pub async fn graph_iql() -> HttpResponse {
    let graphql_endpoint = connection::graphql();
    let html = graphiql::graphiql_source(&graphql_endpoint);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graph_ql(
    st: web::Data<crate::AppData>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let response = web::block(move || {
        let res: GraphQLResponse = data.execute(&st.schema, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    println!("res: {}", response);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(format!("/{}", connection::GRAPHQL_ENDPOINT)).route(web::post().to(graph_ql)),
    )
    .service(web::resource("/graphiql").route(web::get().to(graph_iql)));
}
