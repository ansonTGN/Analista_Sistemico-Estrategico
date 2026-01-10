// src/handlers/motors.rs

use crate::config::AppConfig;
use crate::extract::{ext_from_filename, extract_text_from_file};
use crate::multipart::{read_text_field, save_file_to_tmp};
use crate::openai::{call_responses_api, extract_output_text, InputMessage, ResponsesRequest};
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
    
    // 1. Check API Key
    if cfg.openai_api_key.trim().len() < 5 {
        let err_html = "<div class='alert-box' style='border-left-color:red; background:#fee2e2;'>
            <h2>Error de Configuración</h2>
            <p>Falta la variable OPENAI_API_KEY en el servidor.</p>
        </div>";
        let mut ctx = Context::new();
        ctx.insert("report", err_html);
        let rendered = tera.render("report.html", &ctx).unwrap_or_else(|e| e.to_string());
        return HttpResponse::Ok().content_type("text/html").body(rendered);
    }

    let _ = fs::create_dir_all("/tmp");

    // 2. Data Containers
    let mut target_name = String::new();
    let mut relationship = String::new();
    let mut m_context = String::new();
    let mut m_observations = String::new();
    let mut m_goal = String::new();

    // Default Signals
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

    // 3. Process Multipart
    while let Ok(Some(field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let name = cd.get_name().unwrap_or("").to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        if let Some(fname) = filename {
            if fname.is_empty() { continue; }

            let (tmp_path, _written) = match save_file_to_tmp(field, cfg.max_file_bytes).await {
                Ok(v) => v,
                Err(msg) => return HttpResponse::PayloadTooLarge().body(msg),
            };

            let ext = ext_from_filename(&fname);
            let extracted = extract_text_from_file(&tmp_path, &ext);
            let _ = fs::remove_file(&tmp_path);

            match name.as_str() {
                "m_context_file" => m_context.push_str(&format!("\n\n--- [ADJUNTO: {}] ---\n{}\n", fname, extracted)),
                "m_observations_file" => m_observations.push_str(&format!("\n\n--- [ADJUNTO: {}] ---\n{}\n", fname, extracted)),
                _ => {}
            }
        } else {
            let text_val = match read_text_field(field, cfg.max_text_field_bytes).await {
                Ok(v) => v,
                Err(msg) => return HttpResponse::PayloadTooLarge().body(msg),
            };

            match name.as_str() {
                "target_name" => target_name = text_val,
                "relationship" => relationship = text_val, // Captura cualquier valor del nuevo select
                "m_context" => m_context.push_str(&text_val),
                "m_observations" => m_observations.push_str(&text_val),
                "m_goal" => m_goal.push_str(&text_val),
                
                "sig_security" => sig_security = text_val,
                "sig_belonging" => sig_belonging = text_val,
                "sig_status" => sig_status = text_val,
                "sig_autonomy" => sig_autonomy = text_val,
                "sig_mastery" => sig_mastery = text_val,
                "sig_justice" => sig_justice = text_val,
                "sig_purpose" => sig_purpose = text_val,
                "sig_control" => sig_control = text_val,
                "sig_curiosity" => sig_curiosity = text_val,
                "sig_comfort" => sig_comfort = text_val,
                _ => {}
            }
        }
    }

    // 4. Build Prompt
    let user_prompt = format!(
        "=== EXPEDIENTE DE ANÁLISIS (DATOS DE ENTRADA) ===\n\n\
         [1. SUJETO Y VÍNCULO]\nIdentificador: {}\nRelación: {}\n\n\
         [2. CONTEXTO OPERATIVO]\n{}\n\n\
         [3. OBSERVACIONES CONDUCTUALES]\n{}\n\n\
         [4. OBJETIVO TÁCTICO]\n{}\n\n\
         [5. SEÑALES (1-5)]\nSeg: {}, Pert: {}, Stat: {}, Aut: {}, Mas: {}, Just: {}, Prop: {}, Pow: {}, Cur: {}, Comf: {}",
        target_name.trim(), relationship.trim(), m_context.trim(), m_observations.trim(), m_goal.trim(),
        sig_security, sig_belonging, sig_status, sig_autonomy, sig_mastery,
        sig_justice, sig_purpose, sig_control, sig_curiosity, sig_comfort
    );

    // 5. OpenAI Call
    let request_body = ResponsesRequest {
        model: cfg.openai_model.clone(),
        input: vec![
            InputMessage { role: "system".to_string(), content: MOTORS_SYSTEM_PROMPT.to_string() },
            InputMessage { role: "user".to_string(), content: user_prompt },
        ],
        temperature: 0.25,
        store: false,
    };

    let resp = match call_responses_api(&client, &cfg.openai_api_key, &request_body).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error API: {}", e)),
    };

    let content = extract_output_text(&resp).unwrap_or_else(|| "Error IA: Respuesta vacía".to_string());
    let content = ensure_article(content);
    let content = sanitize_ai_html(&content);

    // 6. JSON Export Data
    let case_data = json!({
        "mode": "motors",
        "target_name": target_name,
        "relationship": relationship,
        "m_context": m_context,
        "m_observations": m_observations,
        "m_goal": m_goal,
        "signals": {
            "sig_security": sig_security,
            "sig_belonging": sig_belonging,
            "sig_status": sig_status,
            "sig_autonomy": sig_autonomy,
            "sig_mastery": sig_mastery,
            "sig_justice": sig_justice,
            "sig_purpose": sig_purpose,
            "sig_control": sig_control,
            "sig_curiosity": sig_curiosity,
            "sig_comfort": sig_comfort
        }
    });

    // 7. Render Response
    let mut ctx = Context::new();
    ctx.insert("report", &content);
    ctx.insert("case_json", &case_data.to_string());

    let rendered = tera.render("report.html", &ctx).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
