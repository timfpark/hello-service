use actix_web::{web, App, HttpServer, Responder};
// use std::env;

async fn greet() -> impl Responder {
    // let cluster = env::var("CLUSTER").unwrap_or("unknown".to_string());
    // let ring = env::var("RING").unwrap_or("unknown".to_string());

    format!("Hello from the main ring in the azure cluster!") // , &ring, &cluster)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let port_string = env::var("PORT").unwrap_or("8080".to_string());
    // let port = port_string.parse::<u16>().unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
