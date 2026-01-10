// src/extract.rs

use std::fs;
use std::io::Read;

// Import crítico: habilita Docx::open(...)
use dotext::MsDoc;

pub fn extract_text_from_file(filepath: &str, extension: &str) -> String {
    match extension {
        "txt" | "md" | "csv" => fs::read_to_string(filepath).unwrap_or_default(),

        "docx" => {
            let mut content = String::new();
            match dotext::Docx::open(filepath) {
                Ok(mut doc) => {
                    let _ = doc.read_to_string(&mut content);
                    if content.trim().is_empty() {
                        "No se pudo leer DOCX o está vacío".to_string()
                    } else {
                        content
                    }
                }
                Err(_) => "No se pudo abrir el DOCX".to_string(),
            }
        }

        "pdf" => pdf_extract::extract_text(filepath)
            .unwrap_or_else(|_| "Error leyendo PDF (posiblemente encriptado o imagen)".to_string()),

        _ => "Formato no soportado".to_string(),
    }
}

pub fn ext_from_filename(filename: &str) -> String {
    filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
}

