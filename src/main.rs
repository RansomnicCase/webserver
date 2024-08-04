use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "simple web server in Rust"
}
async fn about() -> impl Responder {
    "this is the about page"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running on port 8080");
    let port = 8080;
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/about", web::get().to(about))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
