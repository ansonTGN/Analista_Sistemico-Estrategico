// src/i18n.rs
use std::collections::HashMap;

pub fn get_translations(lang: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    
    match lang {
        "en" => {
            map.insert("title", "Strategic Analyst");
            map.insert("subtitle", "OPERATIONAL DIAGNOSIS & HUMAN MOTORS");
            map.insert("tab_system", "Systemic Analysis");
            map.insert("tab_motors", "Human Motors");
            map.insert("btn_text", "Write");
            map.insert("btn_file", "File");
            map.insert("btn_audio", "Audio/Mic");
            map.insert("lbl_situation", "I. Operational Context");
            map.insert("ph_situation", "Describe the problem, the conflict, or the scenario...");
            map.insert("lbl_cv", "II. Agent Profile (CV/Bio)");
            map.insert("ph_cv", "Trajectory, background, previous roles...");
            map.insert("lbl_focus", "III. Strategic Focus");
            map.insert("ph_focus", "Specific doubts? What do you want to solve?");
            map.insert("btn_analyze", "START ANALYSIS");
            
            map.insert("lbl_target", "Target Name/Role");
            map.insert("lbl_relation", "Relationship");
            map.insert("lbl_m_context", "II. Context (Pressure, Culture)");
            map.insert("ph_m_context", "What is at stake? Unwritten rules? Incentives?");
            map.insert("lbl_m_obs", "III. Observations (Behavior)");
            map.insert("ph_m_obs", "Under stress they do X... When corrected they do Y...");
            map.insert("lbl_m_goal", "IV. Tactical Goal");
            map.insert("ph_m_goal", "Align team, negotiate raise, de-escalate conflict...");
            map.insert("lbl_signals", "V. Rapid Signals (1-5)");
            map.insert("btn_motors", "ANALYZE MOTORS");

            map.insert("rec_start", "🔴 Rec");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESSING INTELLIGENCE...");
        },
        "cat" => {
            map.insert("title", "Analista Estratègic");
            map.insert("subtitle", "DIAGNÒSTIC OPERATIU I MOTORS HUMANS");
            map.insert("tab_system", "Anàlisi Sistèmica");
            map.insert("tab_motors", "Motors Humans");
            map.insert("btn_text", "Escriure");
            map.insert("btn_file", "Arxiu");
            map.insert("btn_audio", "Àudio/Mic");
            map.insert("lbl_situation", "I. Context Operatiu");
            map.insert("ph_situation", "Descriviu el problema, el conflicte o l'escenari...");
            map.insert("lbl_cv", "II. Perfil de l'Agent (CV)");
            map.insert("ph_cv", "Trajectòria, antecedents...");
            map.insert("lbl_focus", "III. Focus Estratègic");
            map.insert("ph_focus", "Dubtes concrets? Què voleu resoldre?");
            map.insert("btn_analyze", "INICIAR ANÀLISI");
            
            map.insert("lbl_target", "Nom/Rol del Subjecte");
            map.insert("lbl_relation", "Relació");
            map.insert("lbl_m_context", "II. Context (Pressió, Cultura)");
            map.insert("ph_m_context", "Què hi ha en joc? Regles no escrites?");
            map.insert("lbl_m_obs", "III. Observacions (Conducta)");
            map.insert("ph_m_obs", "Sota estrès fa X... Quan se'l corregeix fa Y...");
            map.insert("lbl_m_goal", "IV. Objectiu Tàctic");
            map.insert("ph_m_goal", "Alinear equip, negociar augment, desescalar...");
            map.insert("lbl_signals", "V. Senyals Ràpids (1-5)");
            map.insert("btn_motors", "ANALITZAR MOTORS");

            map.insert("rec_start", "🔴 Gravar");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESSANT INTEL·LIGÈNCIA...");
        },
        _ => { // ES (Default)
            map.insert("title", "Analista Estratégico");
            map.insert("subtitle", "DIAGNÓSTICO OPERATIVO Y MOTORES HUMANOS");
            map.insert("tab_system", "Análisis Sistémico");
            map.insert("tab_motors", "Motores Humanos");
            map.insert("btn_text", "Escribir");
            map.insert("btn_file", "Archivo");
            map.insert("btn_audio", "Audio/Mic");
            map.insert("lbl_situation", "I. Contexto Operativo");
            map.insert("ph_situation", "Describa el problema, el conflicto o el escenario...");
            map.insert("lbl_cv", "II. Perfil del Agente (CV)");
            map.insert("ph_cv", "Trayectoria, antecedentes...");
            map.insert("lbl_focus", "III. Foco Estratégico");
            map.insert("ph_focus", "¿Dudas concretas? ¿Qué quiere resolver?");
            map.insert("btn_analyze", "INICIAR ANÁLISIS");
            
            map.insert("lbl_target", "Nombre/Rol del Sujeto");
            map.insert("lbl_relation", "Relación");
            map.insert("lbl_m_context", "II. Contexto (Presión, Cultura)");
            map.insert("ph_m_context", "¿Qué está en juego? ¿Reglas no escritas?");
            map.insert("lbl_m_obs", "III. Observaciones (Conducta)");
            map.insert("ph_m_obs", "Bajo estrés hace X... Cuando se le corrige hace Y...");
            map.insert("lbl_m_goal", "IV. Objetivo Táctico");
            map.insert("ph_m_goal", "Alinear equipo, negociar subida, desescalar...");
            map.insert("lbl_signals", "V. Señales Rápidas (1-5)");
            map.insert("btn_motors", "ANALIZAR MOTORES");

            map.insert("rec_start", "🔴 Grabar");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESANDO INTELIGENCIA...");
        }
    };
    map
}