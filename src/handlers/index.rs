// src/handlers/index.rs

use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("index.html", &context).unwrap_or_else(|e| {
        format!(
            "<html><body><h1>Error template</h1><pre>{}</pre></body></html>",
            e
        )
    });
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
