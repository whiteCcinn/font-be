extern crate font_be;
extern crate diesel;
extern crate serde;

use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use self::font_be::*;
use self::models::*;
use self::diesel::prelude::*;

use serde::Serialize;

#[derive(Serialize)]
struct SearchResult {
    id: i32,
    name: String,
}

#[get("/search/{word}")]
async fn search(req: HttpRequest, word: web::Path<String>) -> impl Responder {
    println!("REQ: {:?}", req);
    println!("{:?}", word);

    use font_be::schema::fonts::dsl::*;
    let connection = establish_connection();

    let results = fonts.filter(name.eq(word.as_str())).load::<Fonts>(&connection)
        .expect("Error loading Fonts");

    println!("Displaying {} Fonts", results.len());

    let mut vec: Vec<SearchResult> = Vec::new();

    for item in results {
        vec.push(SearchResult { id: item.id, name: item.name });
    }

    return web::Json(vec);
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