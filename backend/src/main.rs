mod db;
use db::*;
use actix_web::{web, get, post, put, delete, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    body: String,
}

#[get("/api/posts")]
async fn posts() -> impl Responder {
    let result = get_posts();
    HttpResponse::Ok().json(result)
}

#[get("/api/unpublished")]
async fn unpublished() -> impl Responder {
    let result = get_unpublished();
    HttpResponse::Ok().json(result)
}

#[post("/api/create")]
async fn create(post: web::Json<CreatePost>) -> impl Responder {
    let result = create_post(post.0.title.as_ref(), post.0.body.as_ref());
    HttpResponse::Ok().json(result)
}

#[put("/api/publish/{id}")]
async fn publish(path: web::Path<String>) -> impl Responder {
    let result = publish_post(path.to_string());
    HttpResponse::Ok().json(result)
}

#[delete("/api/delete/{id}")]
async fn delete(path: web::Path<String>) -> impl Responder {
    let result = delete_post(path.to_string());
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(unpublished)
            .service(create)
            .service(publish)
            .service(delete)
    })
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}
