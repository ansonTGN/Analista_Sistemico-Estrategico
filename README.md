# 🏛️ Analista Sistémico & Estratégico AI

![Rust](https://img.shields.io/badge/Backend-Rust-black?style=for-the-badge&logo=rust)
![Docker](https://img.shields.io/badge/Deployment-Docker-blue?style=for-the-badge&logo=docker)
![OpenAI](https://img.shields.io/badge/Intelligence-GPT--4o-green?style=for-the-badge&logo=openai)
![Status](https://img.shields.io/badge/Status-Production%20Ready-success?style=for-the-badge)

> **Motor de consultoría de élite basado en Ingeniería de Sistemas y Psicología del Poder.**

Este proyecto implementa una plataforma web de alto rendimiento escrita en **Rust**, diseñada para realizar auditorías estratégicas automáticas. Fusiona el pensamiento sistémico de **Donella Meadows** con la arquitectura de mentalidad de **Angel Urbina**, proporcionando diagnósticos profundos sobre la viabilidad de un individuo dentro de un entorno complejo.

---

## 📖 Marco Teórico y Metodología

A diferencia de los analizadores de CV tradicionales, este sistema no busca "palabras clave". Utiliza un **Prompt de Ingeniería Avanzada** para evaluar la arquitectura mental y estructural:

1.  **Ingeniería de Sistemas (D. Meadows):**
    *   Análisis de Stocks y Flujos (Recursos vs. Drenajes).
    *   Identificación de Bucles de Retroalimentación (Círculos viciosos/virtuosos).
    *   Detección de Arquetipos Sistémicos (Ej. "Desplazamiento de la carga").

2.  **Psicología del Poder (A. Urbina):**
    *   **Masa vs. Élite:** Evaluación de horizontes temporales.
    *   **Locus de Control:** Análisis lingüístico de responsabilidad (Interna vs. Externa).
    *   **Naturaleza del Poder:** Distinción entre Poder Delegado (cargos) y Poder Personal (resultados).

---

## 🚀 Características Técnicas

*   **⚡ Backend en Rust (Actix-Web):** Rendimiento nativo, seguridad de memoria y concurrencia asíncrona.
*   **📱 Diseño Omnicanal (Mobile First):** Interfaz oscura ("Dark Mode") inspirada en terminales financieros, totalmente responsiva.
*   **imparcialidad de Archivos:** Procesamiento nativo de múltiples formatos mediante extracción de texto en el servidor:
    *   `PDF` (vía `pdf-extract` / `libpoppler`).
    *   `DOCX` (vía `dotext`).
    *   `TXT` / `MD`.
*   **🤖 Integración OpenAI (GPT-4o):** Orquestación de prompts complejos con temperatura ajustada para análisis críticos.
*   **📄 Salida Profesional:**
    *   Generación de informes PDF formato A4 perfectos (`html2pdf`).
    *   Lectura de voz (Text-to-Speech) integrada en el navegador.
*   **🐳 Docker Ready:** Configuración *Multi-stage build* optimizada para despliegue en la nube (Render/AWS).

---

## 🛠️ Instalación y Desarrollo Local

### Prerrequisitos
*   **Rust & Cargo:** [Instalar Rust](https://www.rust-lang.org/tools/install)
*   **Librerías del Sistema (Linux):** Necesarias para el procesamiento de PDFs.
    ```bash
    sudo apt-get update
    sudo apt-get install -y pkg-config libpoppler-glib-dev libglib2.0-dev
    ```

### Pasos
1.  **Clonar el repositorio:**
    ```bash
    git clone https://github.com/tu-usuario/analista-sistemico.git
    cd analista-sistemico
    ```

2.  **Configurar Variables de Entorno:**
    Crea un archivo `.env` en la raíz del proyecto:
    ```env
    OPENAI_API_KEY=sk-tu-clave-api-aqui...
    # Opcional: Puerto (por defecto 8080)
    PORT=8080
    ```

3.  **Ejecutar:**
    ```bash
    cargo run
    ```
    El servidor estará disponible en: `http://127.0.0.1:8080`

---

## ☁️ Despliegue en Producción (Render)

Este proyecto está configurado para desplegarse automáticamente mediante **Docker**.

1.  Sube el código a tu repositorio de GitHub/GitLab.
2.  Crea un nuevo **Web Service** en [Render.com](https://render.com).
3.  Selecciona el repositorio. Render detectará automáticamente el `Dockerfile`.
4.  **Configuración de Entorno en Render:**
    *   Añade la variable: `OPENAI_API_KEY`.
    *   El `PORT` es gestionado automáticamente por el código Rust.
5.  Despliega. La construcción tardará unos minutos mientras compila las dependencias de C++ (`poppler`) y Rust.

---

## 📂 Estructura del Proyecto

```text
/analista_sistemico
├── Dockerfile          # Configuración Multi-stage para producción
├── Cargo.toml          # Dependencias (Actix, Tera, Reqwest, Pdf-extract)
├── src/
│   └── main.rs         # Lógica del servidor, orquestación de IA y manejo de archivos
└── templates/
    ├── index.html      # Formulario de entrada (Diseño Dark/Mobile)
    └── report.html     # Plantilla de informe (Formato A4/PDF)