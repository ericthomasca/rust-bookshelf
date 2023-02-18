#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate dotenv;

mod db;
use db::*;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/posts")]
async fn get_all_posts() -> impl Responder {
    let posts = get_posts();
    HttpResponse::Ok().json(posts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_posts))
            .bind(("0.0.0.0", 8088))?
            .run()
            .await
}
