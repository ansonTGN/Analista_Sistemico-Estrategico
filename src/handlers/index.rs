// src/handlers/index.rs

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use tera::{Context, Tera};
use crate::i18n;

#[derive(Deserialize)]
pub struct LangQuery {
    lang: Option<String>,
}

#[get("/")]
pub async fn landing(tera: web::Data<Tera>) -> impl Responder {
    // Pantalla de selección de idioma (Landing)
    let context = Context::new();
    let rendered = tera.render("landing.html", &context).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/app")]
pub async fn app_page(tera: web::Data<Tera>, query: web::Query<LangQuery>) -> impl Responder {
    // 1. Obtener idioma de la query string (?lang=es) o usar default
    let lang = query.lang.clone().unwrap_or_else(|| "es".to_string());
    
    // 2. Preparar contexto con traducciones
    let mut context = Context::new();
    context.insert("lang", &lang);
    context.insert("t", &i18n::get_translations(&lang));

    // 3. Renderizar template principal
    let rendered = tera.render("index.html", &context).unwrap_or_else(|e| {
        format!("Error template: {}", e)
    });

    // 4. Devolver respuesta (IMPORTANTE: Esto debe estar DENTRO de las llaves de la función)
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
