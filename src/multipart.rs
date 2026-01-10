// src/multipart.rs

use actix_multipart::Field;
use futures::StreamExt;
use std::fs;
use std::io::Write;
use uuid::Uuid;

pub async fn read_text_field(mut field: Field, max_bytes: usize) -> Result<String, String> {
    let mut value = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| format!("Error leyendo chunk: {e}"))?;
        if value.len() + data.len() > max_bytes {
            return Err("Campo de texto demasiado grande (límite excedido).".to_string());
        }
        value.extend_from_slice(&data);
    }
    Ok(String::from_utf8(value).unwrap_or_default())
}

/// Guarda el fichero a /tmp y devuelve (tmp_path, bytes_written).
pub async fn save_file_to_tmp(mut field: Field, max_bytes: usize) -> Result<(String, usize), String> {
    let _ = fs::create_dir_all("/tmp");

    let tmp_path = format!("/tmp/{}", Uuid::new_v4());
    let mut f = fs::File::create(&tmp_path).map_err(|e| format!("No se pudo crear tmp: {e}"))?;

    let mut written: usize = 0;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| format!("Error leyendo chunk: {e}"))?;
        written += data.len();
        if written > max_bytes {
            let _ = fs::remove_file(&tmp_path);
            return Err("Archivo demasiado grande (límite excedido).".to_string());
        }
        f.write_all(&data)
            .map_err(|e| format!("Error escribiendo tmp: {e}"))?;
    }

    Ok((tmp_path, written))
}

