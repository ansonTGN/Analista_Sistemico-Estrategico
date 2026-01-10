# Human Motors & Systemic Analyst

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square&logo=rust)
![Actix-Web](https://img.shields.io/badge/Actix--Web-v4-green?style=flat-square)
![OpenAI Whisper](https://img.shields.io/badge/AI-Whisper%20%2B%20GPT--4o-blue?style=flat-square&logo=openai)
![License](https://img.shields.io/badge/License-MIT%2FApache-lightgrey?style=flat-square)

**[ 🇺🇸 English ](#-english) | [ 🇪🇸 Español ](#-español) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

> **Author / Autor:** Angel A. Urbina  
> **Version:** Core v2.2 (2026)

---

<a name="-english"></a>
## 🇺🇸 English

### Overview
**Human Motors & Systemic Analyst** is an advanced operational intelligence suite built in **Rust**. It transforms unstructured data—**text, documents, and now Audio/Voice**—into verifiable hypotheses, systemic power maps, and high-precision behavioral profiles.

The system is designed to eliminate "narrative noise" and focus on **operational leverage**: What buttons to push, what risks to avoid, and how the system (organization) reacts to pressure.

### 🚀 New in v2.2
*   **🎙️ Voice-to-Intel (Whisper Integration):** You can now **record voice notes** directly from the browser or upload audio files (`.mp3`, `.wav`, `.m4a`). The system automatically transcribes and analyzes the audio content.
*   **🌍 Multi-language Core:** Full native support for **English, Spanish, and Catalan**.
*   **Landing Interface:** New entry point for language selection and system initialization.

### Modes of Operation
1.  **Human Motors (HUMINT):**
    *   Identifies psychological drivers (SDT: Autonomy, Competence, Relatedness).
    *   Detects **Status vs. Security** conflicts.
    *   Generates a "User Manual" for the target: *Do's & Don'ts*.
2.  **Systemic Analysis:**
    *   **PESTEL** scanning for environmental pressure.
    *   Separates **Facts** from **Inferences** (Intelligence Discipline).
    *   Identifies feedback loops and systemic bottlenecks.

### Technical Stack
*   **Server:** Rust + Actix-Web (Async/Tokio).
*   **AI Engine:** OpenAI `gpt-4o` (Analysis) + `whisper-1` (Audio Transcription).
*   **Audio Handling:** `reqwest` (multipart streams) + `tokio-util`.
*   **Frontend:** Tera Templates + HTML5 MediaRecorder API.
*   **Ingestion:** PDF, DOCX, TXT, MD, MP3, WAV, M4A.

### Installation & Usage

**Prerequisites:**
*   Rust (Edition 2021)
*   `libpoppler-glib-dev` (Linux) / `poppler` (macOS)
*   OpenAI API Key

**Configuration (`.env`):**
```env
OPENAI_API_KEY=sk-...
OPENAI_MODEL=gpt-4o-mini
PORT=8080
BIND_HOST=0.0.0.0
```

**Run Local:**
```bash
cargo run
```

---

<a name="-español"></a>
## 🇪🇸 Español

### Introducción
**Human Motors & Systemic Analyst** es una suite de inteligencia operativa desarrollada en **Rust**. Su objetivo es transformar información no estructurada —**texto, documentos y ahora Audio/Voz**— en hipótesis verificables, mapas de poder y perfiles conductuales precisos.

El software operacionaliza la psicología organizacional para reducir la incertidumbre en la toma de decisiones críticas.

### 🚀 Novedades v2.2
*   **🎙️ Inteligencia de Voz (Whisper):** Capacidad para **grabar notas de voz** en tiempo real o subir archivos de audio. El sistema transcribe el contenido usando el modelo Whisper de OpenAI y lo integra automáticamente en el análisis.
*   **🌍 Soporte Multilingüe:** Interfaz completa disponible en **Español, Catalán e Inglés**.
*   **Nueva Landing Page:** Pantalla de bienvenida adaptativa para selección de idioma y acceso al sistema.

### Funcionalidades Clave
1.  **Motores Humanos:**
    *   Ranking de motivadores (Teoría de la Autodeterminación).
    *   Detección de fricciones de **Justicia Organizacional**.
    *   Protocolos de interacción: *Qué decir y qué callar*.
2.  **Análisis Sistémico:**
    *   **PESTEL** y contexto operativo.
    *   Disciplina de inteligencia: Separación estricta de Hechos vs. Inferencias.
    *   Pre-Mortem y análisis de bucles de retroalimentación.

### Stack Técnico
*   **Backend:** Rust + Actix-Web.
*   **IA:** OpenAI `gpt-4o` + `whisper-1`.
*   **Audio:** Procesamiento de flujos multipart con `tokio` y `reqwest`.
*   **Frontend:** Diseño "Mobile-First" fluido con soporte para grabación de audio HTML5.

### Ejecución

**Requisitos (Linux/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libpoppler-glib-dev libglib2.0-dev libssl-dev
```

**Ejecución:**
```bash
cargo run
```
Accede a `http://localhost:8080`.

---

<a name="-català"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### Introducció
**Human Motors & Systemic Analyst** és una eina d'intel·ligència operativa avançada feta amb **Rust**. Transforma dades no estructurades —**text, documents i ara Àudio/Veu**— en hipòtesis verificables, mapes de poder sistèmics i perfils conductuals d'alta precisió.

Dissenyada per eliminar el "soroll narratiu" i trobar la palanca operativa real.

### 🚀 Novetats v2.2
*   **🎙️ Intel·ligència de Veu (Whisper):** Podeu **gravar notes de veu** directament des del navegador o pujar fitxers d'àudio. El sistema transcriu i analitza el contingut automàticament.
*   **🌍 Nucli Multilingüe:** Suport natiu per a **Català, Castellà i Anglès**.
*   **Interfície d'Entrada:** Nova pantalla d'inici per a la selecció d'idioma.

### Mòduls
1.  **Motors Humans:** Identificació de *drivers* psicològics, conflictes d'estatus i necessitats de justícia procedimental.
2.  **Anàlisi Sistèmica:** Ús de PESTEL i pensament sistèmic (bucles de reforç) per dissenyar estratègies robustes.

### Execució
Veure els requisits a la secció tècnica anterior.
```bash
cargo run
```

---

## 📚 Scientific Basis / Base Científica

> *The software operationalizes concepts from:* / *El software operacionaliza conceptos de:*

1.  **Edmondson, A. C.** - *Psychological Safety*.
2.  **Ryan, R. M. & Deci, E. L.** - *Self-Determination Theory (SDT)*.
3.  **Heuer, R. J.** - *Psychology of Intelligence Analysis (CIA)*.
4.  **Meadows, D.** - *Thinking in Systems*.
5.  **Kahneman, D.** - *Thinking, Fast and Slow (System 1 vs System 2)*.

---

## License

This project is licensed under the [MIT License](LICENSE).

**© 2026 Angel A. Urbina. All Rights Reserved.**




