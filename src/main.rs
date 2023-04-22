use std::time::Duration;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
struct User {
    name: String,
}

#[get("/users/{user_handle}")]
async fn index(path: web::Path<String>, pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let user_handle = path.into_inner();

    let query_result =
        sqlx::query_as::<_, User>("SELECT users.name FROM users WHERE users.handle = $1")
            .bind(user_handle)
            .fetch_optional(pool.get_ref())
            .await;

    if let Some(user) = query_result.ok().flatten() {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Hello, <b>{}</b>!", user.name));
    }

    HttpResponse::Ok().json(json!({ "method": req.method().as_str(), "path": req.uri().path() }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres:postgres@localhost:5440/benchmark";
    let connection_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::new(3, 0))
        .connect(database_url)
        .await
        .unwrap();
    let host = "127.0.0.1";
    let port = 8000;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(index)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
