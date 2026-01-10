// src/handlers/analyze.rs

use crate::config::AppConfig;
use crate::extract::{ext_from_filename, extract_text_from_file};
use crate::multipart::{read_text_field, save_file_to_tmp};
use crate::openai::{call_responses_api, extract_output_text, transcribe_audio, InputMessage, ResponsesRequest};
use crate::prompts::SYSTEM_PROMPT;
use crate::sanitize::{ensure_article, sanitize_ai_html};

use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use std::fs;
use tera::{Context, Tera};

#[post("/analyze")]
pub async fn analyze(
    mut payload: Multipart,
    tera: web::Data<Tera>,
    client: web::Data<reqwest::Client>,
    cfg: web::Data<AppConfig>,
) -> impl Responder {
    
    let _ = fs::create_dir_all("/tmp");

    let mut situation_text = String::new();
    let mut cv_text = String::new();
    let mut questions_text = String::new();
    let mut lang = "es".to_string(); // Default fallback

    while let Ok(Some(field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let name = cd.get_name().unwrap_or("").to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        if let Some(fname) = filename {
            if fname.is_empty() { continue; }
            
            // Guardar a disco temporal
            let (tmp_path, _) = match save_file_to_tmp(field, cfg.max_file_bytes).await {
                Ok(v) => v,
                Err(msg) => return HttpResponse::PayloadTooLarge().body(msg),
            };

            // Detectar si es audio o documento
            let ext = ext_from_filename(&fname);
            let is_audio = ["mp3", "wav", "m4a", "webm", "ogg"].contains(&ext.as_str());

            let extracted_content = if is_audio {
                // LLAMADA A WHISPER
                match transcribe_audio(&client, &cfg.openai_api_key, &tmp_path).await {
                    Ok(txt) => format!("[TRANSCRIPCIÓN AUDIO {}]: {}\n", fname, txt),
                    Err(e) => format!("[ERROR AUDIO {}]: {}\n", fname, e),
                }
            } else {
                extract_text_from_file(&tmp_path, &ext)
            };

            match name.as_str() {
                "situation_file" | "situation_audio" => situation_text.push_str(&format!("\n{}\n", extracted_content)),
                "cv_file" => cv_text.push_str(&format!("\n{}\n", extracted_content)),
                _ => {}
            }
            let _ = fs::remove_file(&tmp_path);

        } else {
            let val = read_text_field(field, cfg.max_text_field_bytes).await.unwrap_or_default();
            match name.as_str() {
                "situation" => situation_text.push_str(&val),
                "cv" => cv_text.push_str(&val),
                "extra_questions" => questions_text.push_str(&val),
                "lang" => lang = val,
                _ => {}
            }
        }
    }

    // Prompt con Instrucción de Idioma
    let user_prompt = format!(
        "OUTPUT LANGUAGE: {}\n\n=== DATA ===\n[SITUATION]\n{}\n\n[PROFILE]\n{}\n\n[QUESTIONS]\n{}\n",
        lang, situation_text, cv_text, questions_text
    );

    let request_body = ResponsesRequest {
        model: cfg.openai_model.clone(),
        input: vec![
            InputMessage { role: "system".to_string(), content: SYSTEM_PROMPT.to_string() },
            InputMessage { role: "user".to_string(), content: user_prompt },
        ],
        temperature: 0.6,
        store: false,
    };

    let resp = match call_responses_api(&client, &cfg.openai_api_key, &request_body).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let content = extract_output_text(&resp).unwrap_or_else(|| "<p>Error AI</p>".to_string());
    let content = sanitize_ai_html(&ensure_article(content));

    let mut ctx = Context::new();
    ctx.insert("report", &content);
    let rendered = tera.render("report.html", &ctx).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

