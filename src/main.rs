use std::fs::File;
use std::io::{ Read};
use actix_web::{get, App, HttpServer, Responder};
use actix_web::http::header::CONTENT_TYPE;

#[get("/")]
async fn index() -> impl Responder {
    let path = "data.json";

    let mut input = File::open(path).unwrap();

    let mut buf = String::new();

    input.read_to_string(&mut buf).unwrap();

    buf.customize().insert_header((CONTENT_TYPE, "application/json"))
    

}



#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("0.0.0.0", 8060))?
    .run()
    .await
}
