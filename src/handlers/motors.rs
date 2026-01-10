// src/handlers/motors.rs

use crate::config::AppConfig;
use crate::extract::{ext_from_filename, extract_text_from_file};
use crate::multipart::{read_text_field, save_file_to_tmp};
use crate::openai::{call_responses_api, extract_output_text, transcribe_audio, InputMessage, ResponsesRequest};
use crate::prompts::MOTORS_SYSTEM_PROMPT;
use crate::sanitize::{ensure_article, sanitize_ai_html};

use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use serde_json::json;
use std::fs;
use tera::{Context, Tera};

#[post("/motors")]
pub async fn motors(
    mut payload: Multipart,
    tera: web::Data<Tera>,
    client: web::Data<reqwest::Client>,
    cfg: web::Data<AppConfig>,
) -> impl Responder {
    
    let _ = fs::create_dir_all("/tmp");

    let mut target_name = String::new();
    let mut relationship = String::new();
    let mut m_context = String::new();
    let mut m_observations = String::new();
    let mut m_goal = String::new();
    let mut lang = "es".to_string();

    // The 10 Fundamental Signals (Initialized to neutral "3")
    let mut sig_security = "3".to_string();
    let mut sig_belonging = "3".to_string();
    let mut sig_status = "3".to_string();
    let mut sig_autonomy = "3".to_string();
    let mut sig_mastery = "3".to_string();
    let mut sig_justice = "3".to_string();
    let mut sig_purpose = "3".to_string();
    let mut sig_control = "3".to_string();
    let mut sig_curiosity = "3".to_string();
    let mut sig_comfort = "3".to_string();

    while let Ok(Some(field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let name = cd.get_name().unwrap_or("").to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        if let Some(fname) = filename {
            if fname.is_empty() { continue; }

            let (tmp_path, _) = match save_file_to_tmp(field, cfg.max_file_bytes).await {
                Ok(v) => v,
                Err(msg) => return HttpResponse::PayloadTooLarge().body(msg),
            };

            let ext = ext_from_filename(&fname);
            let is_audio = ["mp3", "wav", "m4a", "webm", "ogg"].contains(&ext.as_str());

            let content = if is_audio {
                match transcribe_audio(&client, &cfg.openai_api_key, &tmp_path).await {
                    Ok(t) => format!("\n[AUDIO TRANSCRIPT]: {}\n", t),
                    Err(e) => format!("\n[AUDIO ERROR]: {}\n", e),
                }
            } else {
                extract_text_from_file(&tmp_path, &ext)
            };

            match name.as_str() {
                "m_context_file" | "m_context_audio" => m_context.push_str(&content),
                "m_observations_file" | "m_observations_audio" => m_observations.push_str(&content),
                _ => {}
            }
            let _ = fs::remove_file(&tmp_path);

        } else {
            let val = read_text_field(field, cfg.max_text_field_bytes).await.unwrap_or_default();
            match name.as_str() {
                "target_name" => target_name = val,
                "relationship" => relationship = val,
                "m_context" => m_context.push_str(&val),
                "m_observations" => m_observations.push_str(&val),
                "m_goal" => m_goal.push_str(&val),
                "lang" => lang = val,
                
                // Capture all 10 signals
                "sig_security" => sig_security = val,
                "sig_belonging" => sig_belonging = val,
                "sig_status" => sig_status = val,
                "sig_autonomy" => sig_autonomy = val,
                "sig_mastery" => sig_mastery = val,
                "sig_justice" => sig_justice = val,
                "sig_purpose" => sig_purpose = val,
                "sig_control" => sig_control = val,
                "sig_curiosity" => sig_curiosity = val,
                "sig_comfort" => sig_comfort = val,
                _ => {}
            }
        }
    }

    let user_prompt = format!(
        "OUTPUT LANGUAGE: {}\n\n=== CASE FILE ===\n\
         [TARGET]: {} ({})\n\
         [CONTEXT]: {}\n\
         [OBSERVATIONS]: {}\n\
         [GOAL]: {}\n\
         [RAPID SIGNALS 1-5]:\n\
         - Security: {}\n\
         - Belonging: {}\n\
         - Status: {}\n\
         - Autonomy: {}\n\
         - Mastery: {}\n\
         - Justice: {}\n\
         - Purpose: {}\n\
         - Control: {}\n\
         - Curiosity: {}\n\
         - Comfort: {}",
        lang, target_name, relationship, m_context, m_observations, m_goal, 
        sig_security, sig_belonging, sig_status, sig_autonomy, sig_mastery,
        sig_justice, sig_purpose, sig_control, sig_curiosity, sig_comfort
    );

    let request_body = ResponsesRequest {
        model: cfg.openai_model.clone(),
        input: vec![
            InputMessage { role: "system".to_string(), content: MOTORS_SYSTEM_PROMPT.to_string() },
            InputMessage { role: "user".to_string(), content: user_prompt },
        ],
        temperature: 0.3,
        store: false,
    };

    let resp = match call_responses_api(&client, &cfg.openai_api_key, &request_body).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let content = sanitize_ai_html(&ensure_article(extract_output_text(&resp).unwrap_or_default()));
    
    // JSON para guardar caso completo
    let case_data = json!({ 
        "target": target_name, 
        "relation": relationship,
        "context": m_context,
        "signals": {
            "sec": sig_security, "bel": sig_belonging, "sta": sig_status, "aut": sig_autonomy, "mas": sig_mastery,
            "jus": sig_justice, "pur": sig_purpose, "con": sig_control, "cur": sig_curiosity, "com": sig_comfort
        }
    });

    let mut ctx = Context::new();
    ctx.insert("report", &content);
    ctx.insert("case_json", &case_data.to_string());
    
    let rendered = tera.render("report.html", &ctx).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
