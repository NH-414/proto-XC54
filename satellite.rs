use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "port {4042} xxprototype -- ninja 45 "
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
