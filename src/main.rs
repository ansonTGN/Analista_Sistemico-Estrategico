use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use reqwest::Client;
use dotenv::dotenv;
use std::env;
use std::io::Write;
use std::io::Read; // Necesario para dotext
use std::fs;
use dotext::MsDoc;

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Default)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Default)]
struct Choice {
    message: Message,
}

const SYSTEM_PROMPT: &str = r#"
# ROL
Eres el Arquitecto Estratégico Personal del usuario (Consultor Sistémico de Élite).
Marco: Ingeniería de Sistemas (Meadows) + Psicología del Poder (Urbina).

# OBJETIVO
Analiza los datos adjuntos (Contexto y CV). Háblale al usuario de "TÚ". Sé directo, empoderador pero brutalmente honesto.

# FORMATO DE SALIDA (HTML RESPONSIVO)
Genera SOLO el contenido HTML dentro de etiquetas <article>.
Estructura obligatoria:

<article>
    <div class="executive-summary">
        <h2>Dictamen Estratégico</h2>
        <p class="highlight">[Veredicto en 2 frases]</p>
    </div>

    <section class="grid-2">
        <div class="card">
            <h3>Tu Arquitectura Mental</h3>
            <ul>
                <li><strong>Horizonte:</strong> [Masa/Élite]</li>
                <li><strong>Locus:</strong> [Interno/Externo]</li>
                <li><strong>Poder:</strong> [Personal/Delegado]</li>
            </ul>
        </div>
        <div class="card">
            <h3>El Campo de Batalla</h3>
            <ul>
                <li><strong>Trampa:</strong> [Arquetipo sistémico]</li>
                <li><strong>Fricción:</strong> [Choque Perfil vs Sistema]</li>
            </ul>
        </div>
    </section>

    <section class="deep-dive">
        <h3>Análisis de Viabilidad</h3>
        <p>[Análisis denso y profundo]</p>
    </section>

    <section class="roadmap">
        <h3>Tu Plan de Maniobra</h3>
        <div class="step">
            <span class="step-num">I</span>
            <div class="step-content"><strong>Inmediato:</strong> <p>[Acción]</p></div>
        </div>
        <div class="step">
            <span class="step-num">II</span>
            <div class="step-content"><strong>Estructural:</strong> <p>[Acción]</p></div>
        </div>
         <div class="step">
            <span class="step-num">III</span>
            <div class="step-content"><strong>Mentalidad:</strong> <p>[Acción]</p></div>
        </div>
    </section>
</article>
"#;

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

fn extract_text_from_file(filepath: &str, extension: &str) -> String {
    match extension {
        "txt" | "md" | "csv" => fs::read_to_string(filepath).unwrap_or_default(),
        "docx" => {
            let mut content = String::new();
            if let Ok(mut doc) = dotext::Docx::open(filepath) {
                let _ = doc.read_to_string(&mut content);
            }
            if content.is_empty() { "No se pudo leer DOCX o está vacío".to_string() } else { content }
        },
        "pdf" => {
            pdf_extract::extract_text(filepath)
                .unwrap_or_else(|_| "Error leyendo PDF (Posiblemente encriptado o imagen)".to_string())
        },
        _ => "Formato no soportado".to_string(),
    }
}

#[post("/analyze")]
async fn analyze(
    mut payload: Multipart,
    tera: web::Data<Tera>,
    client: web::Data<Client>,
) -> impl Responder {
    
    // Obtener API Key de entorno (Render inyecta esto)
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
    
    let mut situation_text = String::new();
    let mut cv_text = String::new();
    let mut questions_text = String::new();

    // Crear carpeta temporal si no existe (importante para Docker)
    let _ = fs::create_dir_all("/tmp");

    while let Ok(Some(mut field)) = payload.try_next().await {
        
        // 1. Extraer metadatos y liberar el borrow
        let (name, filename) = {
            let cd = field.content_disposition();
            let n = cd.get_name().unwrap_or("").to_string();
            let f = cd.get_filename().map(|s| s.to_string());
            (n, f)
        }; 

        // 2. Procesar contenido
        if let Some(fname) = filename {
            if !fname.is_empty() {
                // Es un archivo
                let uuid = uuid::Uuid::new_v4();
                let temp_path = format!("/tmp/{}", uuid);
                
                // Guardar archivo
                if let Ok(mut f) = fs::File::create(&temp_path) {
                     while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            let _ = f.write_all(&data);
                        }
                    }
                }

                // Extraer texto
                let ext = fname.split('.').last().unwrap_or("").to_lowercase();
                let extracted = extract_text_from_file(&temp_path, &ext);
                
                match name.as_str() {
                    "situation_file" => situation_text.push_str(&format!("\n[ARCHIVO SITUACIÓN]:\n{}", extracted)),
                    "cv_file" => cv_text.push_str(&format!("\n[ARCHIVO CV]:\n{}", extracted)),
                    _ => {}
                }
                
                let _ = fs::remove_file(temp_path);
            }
        } else {
            // Es texto plano del formulario
            let mut value = Vec::new();
            while let Some(chunk) = field.next().await {
                if let Ok(data) = chunk {
                    value.extend_from_slice(&data);
                }
            }
            let text_val = String::from_utf8(value).unwrap_or_default();

            match name.as_str() {
                "situation" => situation_text.push_str(&text_val),
                "cv" => cv_text.push_str(&text_val),
                "extra_questions" => questions_text.push_str(&text_val),
                _ => {}
            }
        }
    }

    // Comprobación de seguridad API Key
    if api_key.len() < 5 {
        let err_html = "<div class='executive-summary' style='border-left-color:red'><h2>Error de Configuración</h2><p>Falta la variable OPENAI_API_KEY en Render.</p></div>";
        let mut ctx = Context::new();
        ctx.insert("report", err_html);
        return HttpResponse::Ok().content_type("text/html").body(tera.render("report.html", &ctx).unwrap());
    }

    let user_prompt = format!(
        "--- SITUACIÓN ---\n{}\n\n--- PERFIL (CV) ---\n{}\n\n--- DUDAS ---\n{}",
        situation_text, cv_text, questions_text
    );

    let request_body = OpenAIRequest {
        model: "gpt-4o".to_string(), 
        messages: vec![
            Message { role: "system".to_string(), content: SYSTEM_PROMPT.to_string() },
            Message { role: "user".to_string(), content: user_prompt },
        ],
        temperature: 0.6, 
    };

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let json: OpenAIResponse = resp.json().await.unwrap_or_default();
            let content = json.choices.first().map(|c| c.message.content.clone()).unwrap_or("<p>Error: La IA no devolvió contenido.</p>".into());
            
            let mut context = Context::new();
            context.insert("report", &content);
            let rendered = tera.render("report.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error conectando con OpenAI"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error cargando templates: {}", e);
            ::std::process::exit(1);
        }
    };
    
    let client = Client::new();
    let _ = fs::create_dir_all("/tmp");

    // LÓGICA DE PUERTO PARA RENDER
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str.parse().expect("PORT debe ser un número");

    println!("SISTEMA INICIADO EN PUERTO: {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(analyze)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}