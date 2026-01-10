// src/config.rs

use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_host: String,
    pub bind_port: u16,

    pub openai_api_key: String,
    pub openai_model: String,

    pub http_connect_timeout_secs: u64,
    pub http_timeout_secs: u64,

    pub max_payload_bytes: usize,
    pub max_text_field_bytes: usize,
    pub max_file_bytes: usize,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let bind_host = env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let bind_port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);

        let openai_api_key = env::var("OPENAI_API_KEY").unwrap_or_default();

        // compatibilidad con tus variables anteriores (OPENAI_MODEL o AI_MODEL)
        let openai_model = env::var("OPENAI_MODEL")
            .or_else(|_| env::var("AI_MODEL"))
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let http_connect_timeout_secs = env::var("HTTP_CONNECT_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(10);

        let http_timeout_secs = env::var("HTTP_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(90);

        let max_payload_bytes = env::var("MAX_PAYLOAD_BYTES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(25 * 1024 * 1024);

        let max_text_field_bytes = env::var("MAX_TEXT_FIELD_BYTES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(512_000);

        let max_file_bytes = env::var("MAX_FILE_BYTES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(10 * 1024 * 1024);

        Self {
            bind_host,
            bind_port,
            openai_api_key,
            openai_model,
            http_connect_timeout_secs,
            http_timeout_secs,
            max_payload_bytes,
            max_text_field_bytes,
            max_file_bytes,
        }
    }
}
