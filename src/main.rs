use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[get("/search/{fonts}")]
async fn search(req: HttpRequest, fonts: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", fonts)
}

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(search)
            .service(
                web::resource("/")
                    .default_service(
                        web::route().to(|| HttpResponse::MethodNotAllowed()),
                    )
                    .route(web::get().to(index_async)),
            )
            .service(web::resource("/test").to(|| async { "Test\r\n" }))
    })
        .bind("127.0.0.1:5678")?
        .workers(2)
        .run()
        .await
}