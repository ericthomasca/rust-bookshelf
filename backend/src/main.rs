mod db;
use db::*;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// #[derive(Debug, Serialize, Deserialize)]
// struct CreatePost {
//     title: String,
//     body: String,
// }

#[get("/api/posts")]
async fn get_all_posts() -> impl Responder {
    let posts = get_posts();
    HttpResponse::Ok().json(posts)
}

// #[post("/api/posts/add")]
// async fn create(post: web::Json<CreatePost>, req: HttpRequest) -> impl Responder {
//     println!("request: {:?}", req);
//     println!("model: {:?}", post);

//     let result = create_post(post.0.title.as_ref(), post.0.body.as_ref());

//     HttpResponse::Ok().json(result)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_posts)
    )
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}
