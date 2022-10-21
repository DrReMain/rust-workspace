use actix_web::{App, guard, HttpResponse, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "localhost:3000"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("localhost") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "127.0.0.1:3000"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("127.0.0.1") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}