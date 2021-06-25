use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use actix_web_prom::PrometheusMetrics;
use std::env;

async fn greet() -> impl Responder {
    let cluster = env::var("CLUSTER").unwrap_or("unknown".to_string());
    let ring = env::var("RING").unwrap_or("unknown".to_string());
    let tag = env::var("TAG").unwrap_or("unknown".to_string());

    format!("Bonjour from the '{}' ring in the '{}' cluster with tag '{}'!", &ring, &cluster, &tag)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse::<u16>().unwrap();

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .route("/", web::get().to(greet))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
