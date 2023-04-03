use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use pokeday_server::routes; 

#[get("/")]
async fn hello() -> impl Responder {

    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {

    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(routes::auth::create_account)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 9093))?
    .run()
        .await
}
