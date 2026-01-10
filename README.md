# Human Motors & Systemic Analyst

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square&logo=rust)
![Actix-Web](https://img.shields.io/badge/Actix--Web-v4-green?style=flat-square)
![OpenAI](https://img.shields.io/badge/AI-OpenAI%20API-blue?style=flat-square&logo=openai)
![License](https://img.shields.io/badge/License-MIT%2FApache-lightgrey?style=flat-square)

**[ 🇺🇸 English ](#-english) | [ 🇪🇸 Español ](#-español) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

---

<a name="-english"></a>
## 🇺🇸 English

### Overview
**Human Motors & Systemic Analyst** is a web application built in **Rust** designed to generate **operational reports** from unstructured data (context, facts, behavioral observations, and goals). The system transforms raw information into verifiable hypotheses, explicit assumptions, and actionable plans, avoiding closed narratives.

It operates in two distinct modes:
1.  **Human Motors:** Identifies probable motivators, observable signals, and status/justice conflicts. It proposes verification through interviews and low-risk micro-experiments.
2.  **Systemic Analysis:** Structures the environment using PESTEL, strict analytical discipline (separating facts from inferences), and systems thinking (actors, loops, levers).

### Scientific Basis
In leadership and operations, judgment errors often stem from **misattributions** (overestimating traits vs. context), **biases under uncertainty**, and **social dynamics**. This project operationalizes academic literature to turn a "case" into a disciplined process:

*   **Psychological Safety (Edmondson):** Facilitates learning and reduces defensive decision-making.
*   **Self-Determination Theory (SDT):** Understands behavior as a function of psychological needs (autonomy, competence, relatedness).
*   **Social Identity & Leadership:** How group belonging and prototypicality condition influence and cooperation.
*   **Organizational Justice:** Procedural and distributive justice as drivers of trust and conflict.
*   **Debiasing:** Moving away from narrative certainty towards rival hypotheses and incremental verification.

### Technical Stack
*   **Server:** Rust + Actix-Web (async).
*   **Templates:** Tera.
*   **LLM Client:** OpenAI Responses API via `reqwest`.
*   **Ingestion:** `pdf-extract` (PDF), `dotext` (DOCX), and text/markdown parsers.
*   **Sanitization:** `ammonia` (strict HTML allowlist).

### Installation & Usage

**Prerequisites:**
*   Rust (2021 edition)
*   `libpoppler-glib-dev` (for PDF support on Linux)

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

**Docker:**
```bash
docker build -t human-motors .
docker run --rm -p 8080:8080 -e OPENAI_API_KEY="sk-..." human-motors
```

---

<a name="-español"></a>
## 🇪🇸 Español

### Introducción
Aplicación web en **Rust** para generar **informes operativos** a partir de información no estructurada. El sistema transforma el contexto y las observaciones en:
- **Hipótesis verificables** (no “relatos cerrados”),
- **Contra-hipótesis** y supuestos explícitos,
- **Indicadores y preguntas** para confirmar/refutar,
- **Plan de actuación** de corto ciclo (7–14 días).

### Base Científica (Psicología Operativa)
El objetivo no es “etiquetar” a las personas, sino reducir la ambigüedad operativa. El software operacionaliza evidencia de la literatura para mitigar errores de juicio:

1.  **Seguridad Psicológica:** Facilita el intercambio de información y reduce decisiones defensivas (elegir "lo seguro para mí" vs "lo óptimo para la organización").
2.  **Motivación (SDT):** Marco robusto para entender conductas como función de necesidades psicológicas y tipos de regulación.
3.  **Identidad Social:** Explica cómo la pertenencia y las normas de grupo condicionan la cooperación.
4.  **Justicia Organizacional:** Las percepciones de justicia procedimental influyen críticamente en la confianza.
5.  **Debiasing:** Salida basada en hipótesis rivales y evidencia, no en certeza narrativa.

### Funcionalidades

#### 1. Motores Humanos
*   Ranking de hipótesis de motivadores (evidencia vs contra-evidencia).
*   Señales observables, activadores y fricciones.
*   **Guía de verificación:** Preguntas de entrevista y micro-experimentos de bajo riesgo.
*   **Plan ético:** Alineación, incentivos y justicia procedimental.

#### 2. Análisis Sistémico
*   **PESTEL:** Presiones del entorno.
*   **Disciplina analítica:** Separación estricta de Hechos vs Hipótesis vs Inferencias.
*   **Modelo sistémico:** Actores, recursos, bucles de retroalimentación y palancas.

### Arquitectura Técnica

**Stack:**
*   **Backend:** Rust + Actix-Web.
*   **Renderizado:** Tera (`templates/`).
*   **IA:** Cliente HTTP asíncrono hacia OpenAI Responses API.
*   **Seguridad:** Sanitización HTML con `ammonia`, subida de archivos segura (`multipart/form-data`) y gestión de límites de memoria.

**Flujo de Datos:**
1.  Recepción Multipart (Texto + Archivos).
2.  Extracción y normalización (PDF/DOCX/TXT).
3.  Construcción de Prompt (Sistema vs Usuario).
4.  Inferencia (LLM).
5.  Post-proceso y Renderizado HTML.

### Ejecución y Despliegue

**Requisitos (Linux/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libpoppler-glib-dev libglib2.0-dev
```

**Ejecución Local:**
Crea un archivo `.env` (ver sección de configuración arriba) y ejecuta:
```bash
cargo run
```

**Docker:**
```bash
docker build -t motores-humanos .
docker run --rm -p 8080:8080 --env-file .env motores-humanos
```

---

<a name="-català"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### Introducció
Aplicació web en **Rust** per generar **informes operatius** a partir d'informació no estructurada. El sistema transforma el context i les observacions en hipòtesis verificables, contra-hipòtesis i plans d'actuació de cicle curt, evitant els "relats tancats".

### Base Científica
En lideratge i entorns operatius, molts errors de judici provenen d'atribucions errònies i biaixos sota incertesa. Aquest projecte transforma un "cas" en un procés d'anàlisi disciplinat basat en:

1.  **Seguretat Psicològica:** Per reduir la presa de decisions defensives.
2.  **Teoria de l'Autodeterminació (SDT):** Motivació basada en autonomia, competència i relació.
3.  **Justícia Organitzacional:** Impacte de la justícia procedimental en la confiança i el conflicte.
4.  **Debiasing:** Ús d'hipòtesis rivals i verificació incremental.

### Funcionalitats

#### 1. Motors Humans
*   Rànquing d'hipòtesis de motivadors.
*   Senyals observables i friccions.
*   Guia de verificació (entrevistes i micro-experiments).

#### 2. Anàlisi Sistèmica
*   PESTEL i pressions de l'entorn.
*   Distinció estricta entre Fets, Hipòtesis i Inferències.
*   Full de ruta operatiu basat en palanques sistèmiques.

### Execució

**Requisits:**
Veure la secció tècnica anterior (Rust, Cargo, llibreries `poppler`).

**Docker:**
```bash
docker build -t motors-humans .
docker run --rm -p 8080:8080 --env-file .env motors-humans
```

---

## 📚 Bibliography / Bibliografía / Bibliografia

> *Note: This bibliography serves as the conceptual backbone of the project; the software implements a flow of hypotheses and verification based on these works.*

1.  **Edmondson, A. C.** (2023). *Psychological Safety Comes of Age: Observed Themes in an Established Literature*. **Annual Review of Organizational Psychology and Organizational Behavior**.
2.  **Van den Broeck, A., et al.** (2021). *Beyond intrinsic and extrinsic motivation: A meta-analysis on self-determination theory’s multidimensional conceptualization of work motivation*. **Organizational Psychology Review**.
3.  **Steffens, N. K., Haslam, S. A., et al.** (2021). *Advancing the social identity theory of leadership: A meta-analytic review of leader group prototypicality*.
4.  **Ashforth, B. E.** (2024). *The Future: What We'd Change in “Social Identity Theory and Organizations”*. **SAGE / Annual Review**.
5.  **Graso, M.** (2020). *Organizational justice enactment: An agent-focused review*. **Human Relations**.
6.  **Colquitt, J. A., et al.** (2013). *Justice at the Millennium, a Decade Later: A Meta-Analytic Test of Social Exchange and Affect-Based Perspectives*.
7.  **Rau, D.** (2025). *A review of cognitive biases in strategic decision making (2000–2023)*. **Long Range Planning**.
8.  **Pavićević, S.** (2025). *Debiasing the Literature on Executive Decision-Making Biases*. **Academy of Management Annals**.
9.  **Dharanikota, H., et al.** (2024). *Debiasing Judgements Using a Distributed Cognition Approach*.
10. **Artinger, F. M., & Marx-Fleck, S.** (2025). *Coping with uncertainty: The interaction of psychological safety and authentic leadership in their effects on defensive decision making*. **Journal of Business Research**.
11. **Wang, Z., et al.** (2022). *Development and Validation of a Motivation Scale for Status*. **Frontiers in Psychology**.
12. **Kahneman, D., & Tversky, A.** (1979). *Prospect Theory: An Analysis of Decision under Risk*. **Econometrica**.
13. **Ross, L.** (1977). *The Intuitive Psychologist and His Shortcomings*.

---

## License

This project is licensed under the [MIT License](LICENSE).



