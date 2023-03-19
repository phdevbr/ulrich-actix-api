use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let now = Local::now();
    println!("{:#?} - API sucessfully started at 0.0.0.0:80", now);
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
