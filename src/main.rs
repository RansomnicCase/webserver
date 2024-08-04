use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use log::info;

async fn hello() -> impl Responder {
    "simple web server in Rust"
}
async fn about() -> impl Responder {
    "this is the about page"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("Rust_log", "info");
    env_logger::init();
    let port = 8080;
    info!("server is running on port {}", port);

    //println!("server is running on port 8080"); replaced this by using env logger

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/about", web::get().to(about))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
