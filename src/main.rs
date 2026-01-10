// src/main.rs

mod config;
mod extract;
mod multipart;
mod openai;
mod prompts;
mod sanitize;

mod handlers;

use actix_web::{web, App, HttpServer};
use config::AppConfig;
use dotenv::dotenv;
use reqwest::Client;
use std::time::Duration;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cfg = AppConfig::from_env();

    // Evita el "use of moved value: cfg"
    let bind_host = cfg.bind_host.clone();
    let bind_port = cfg.bind_port;

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error cargando templates: {e}");
            std::process::exit(1);
        }
    };

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(cfg.http_connect_timeout_secs))
        .timeout(Duration::from_secs(cfg.http_timeout_secs))
        .build()
        .expect("No se pudo construir reqwest Client");

    println!("SISTEMA INICIADO EN: http://{}:{}", bind_host, bind_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(cfg.max_payload_bytes))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(handlers::index::index)
            .service(handlers::analyze::analyze)
            .service(handlers::motors::motors)
    })
    .bind((bind_host.as_str(), bind_port))?
    .run()
    .await
}


