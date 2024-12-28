//Web Microservice For caluclating multiple types of mathematical expressions
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// "This is a Calulator Microservice" page
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a Calculator Microservice")
}

// Add two numbers
#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

// Subtract two numbers
#[get("/sub/{a}/{b}")]
async fn sub(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::sub(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

// Multiply two numbers
#[get("/mul/{a}/{b}")]
async fn mul(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::mul(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

// Divide two numbers
#[get("/div/{a}/{b}")]
async fn div(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::div(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(sub)
            .service(mul)
            .service(div)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
