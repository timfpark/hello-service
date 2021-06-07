use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::env;

// use std::env;

async fn greet() -> impl Responder {
    let cluster = env::var("CLUSTER").unwrap_or("unknown".to_string());
    let ring = env::var("RING").unwrap_or("unknown".to_string());

    format!("Hello from the '{}' ring in the '{}' cluster!", &ring, &cluster)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse::<u16>().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(greet))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
