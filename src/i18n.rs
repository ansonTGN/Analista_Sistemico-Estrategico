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
            map.insert("btn_analyze", "START ANALYSIS");
            map.insert("btn_motors", "ANALYZE MOTORS");
            map.insert("rec_start", "🔴 Rec");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESSING INTELLIGENCE...");
            map.insert("btn_help", "GUIDE / SCIENCE"); // Nuevo

            // SYSTEMIC
            map.insert("lbl_situation", "I. Operational Context");
            map.insert("ph_situation", "Describe the problem, the conflict, or the scenario...");
            map.insert("lbl_cv", "II. Agent Profile (CV/Bio)");
            map.insert("ph_cv", "Trajectory, background, previous roles...");
            map.insert("lbl_focus", "III. Strategic Focus");
            map.insert("ph_focus", "Specific doubts? What do you want to solve?");

            // MOTORS
            map.insert("lbl_target", "Target Name/Rol");
            map.insert("lbl_relation", "Relationship");
            map.insert("lbl_m_context", "II. Context (Pressure, Culture)");
            map.insert("ph_m_context", "What is at stake? Unwritten rules? Incentives?");
            map.insert("lbl_m_obs", "III. Observations (Behavior)");
            map.insert("ph_m_obs", "Under stress they do X... When corrected they do Y...");
            map.insert("lbl_m_goal", "IV. Tactical Goal");
            map.insert("ph_m_goal", "Align team, negotiate raise, de-escalate conflict...");
            
            // SIGNALS & LEGEND
            map.insert("lbl_signals", "V. Rapid Signals (Drivers)");
            map.insert("scale_desc", "Rate the intensity of the need/drive:"); // Nuevo
            map.insert("scale_low", "1 = Irrelevant / Aversion"); // Nuevo
            map.insert("scale_high", "5 = Critical / Obsession"); // Nuevo

            map.insert("rel_boss", "Superior / Boss");
            map.insert("rel_team", "Direct Report / Team");
            map.insert("rel_peer", "Peer / Colleague");
            map.insert("rel_client", "Client / Customer");
            map.insert("rel_provider", "Supplier / Partner");
            map.insert("rel_adversary", "Adversary / Rival");
            map.insert("rel_regulator", "Regulator / Authority");
            map.insert("rel_mentor", "Mentor / Advisor");
            map.insert("rel_other", "Other / Complex");

            map.insert("sig_security", "Security/Safety");
            map.insert("sig_belonging", "Belonging (Tribe)");
            map.insert("sig_status", "Status/Prestige");
            map.insert("sig_autonomy", "Autonomy/Freedom");
            map.insert("sig_mastery", "Mastery/Competence");
            map.insert("sig_justice", "Fairness/Justice");
            map.insert("sig_purpose", "Purpose/Meaning");
            map.insert("sig_control", "Power/Control");
            map.insert("sig_curiosity", "Curios./Novelty");
            map.insert("sig_comfort", "Comfort/Energy");
        },
        "cat" => {
            map.insert("title", "Analista Estratègic");
            map.insert("subtitle", "DIAGNÒSTIC OPERATIU I MOTORS HUMANS");
            map.insert("tab_system", "Anàlisi Sistèmica");
            map.insert("tab_motors", "Motors Humans");
            map.insert("btn_text", "Escriure");
            map.insert("btn_file", "Arxiu");
            map.insert("btn_audio", "Àudio/Mic");
            map.insert("btn_analyze", "INICIAR ANÀLISI");
            map.insert("btn_motors", "ANALITZAR MOTORS");
            map.insert("rec_start", "🔴 Gravar");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESSANT INTEL·LIGÈNCIA...");
            map.insert("btn_help", "GUIA / CIÈNCIA");

            map.insert("lbl_situation", "I. Context Operatiu");
            map.insert("ph_situation", "Descriviu el problema, el conflicte o l'escenari...");
            map.insert("lbl_cv", "II. Perfil de l'Agent (CV)");
            map.insert("ph_cv", "Trajectòria, antecedents...");
            map.insert("lbl_focus", "III. Focus Estratègic");
            map.insert("ph_focus", "Dubtes concrets? Què voleu resoldre?");

            map.insert("lbl_target", "Nom/Rol del Subjecte");
            map.insert("lbl_relation", "Relació");
            map.insert("lbl_m_context", "II. Context (Pressió, Cultura)");
            map.insert("ph_m_context", "Què hi ha en joc? Regles no escrites?");
            map.insert("lbl_m_obs", "III. Observacions (Conducta)");
            map.insert("ph_m_obs", "Sota estrès fa X... Quan se'l corregeix fa Y...");
            map.insert("lbl_m_goal", "IV. Objectiu Tàctic");
            map.insert("ph_m_goal", "Alinear equip, negociar augment, desescalar...");
            
            map.insert("lbl_signals", "V. Senyals Ràpids (Drivers)");
            map.insert("scale_desc", "Valoreu la intensitat de la necessitat/motor:");
            map.insert("scale_low", "1 = Irrellevant / Aversió");
            map.insert("scale_high", "5 = Crític / Obsessió");

            map.insert("rel_boss", "Superior / Cap");
            map.insert("rel_team", "Reporte Directe / Equip");
            map.insert("rel_peer", "Par / Col·laborador");
            map.insert("rel_client", "Client / Stakeholder");
            map.insert("rel_provider", "Proveïdor / Soci");
            map.insert("rel_adversary", "Adversari / Rival");
            map.insert("rel_regulator", "Regulador / Autoritat");
            map.insert("rel_mentor", "Mentor / Conseller");
            map.insert("rel_other", "Altre / Complex");

            map.insert("sig_security", "Seguretat/Certesa");
            map.insert("sig_belonging", "Pertinença (Tribu)");
            map.insert("sig_status", "Estatus/Prestigi");
            map.insert("sig_autonomy", "Autonomia/Llibertat");
            map.insert("sig_mastery", "Mestratge/Competència");
            map.insert("sig_justice", "Justícia/Equitat");
            map.insert("sig_purpose", "Propòsit/Sentit");
            map.insert("sig_control", "Poder/Control");
            map.insert("sig_curiosity", "Curiositat/Novetat");
            map.insert("sig_comfort", "Confort/Estalvi");
        },
        _ => { // ES (Default)
            map.insert("title", "Analista Estratégico");
            map.insert("subtitle", "DIAGNÓSTICO OPERATIVO Y MOTORES HUMANOS");
            map.insert("tab_system", "Análisis Sistémico");
            map.insert("tab_motors", "Motores Humanos");
            map.insert("btn_text", "Escribir");
            map.insert("btn_file", "Archivo");
            map.insert("btn_audio", "Audio/Mic");
            map.insert("btn_analyze", "INICIAR ANÁLISIS");
            map.insert("btn_motors", "ANALIZAR MOTORS");
            map.insert("rec_start", "🔴 Grabar");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESANDO INTELIGENCIA...");
            map.insert("btn_help", "GUÍA / CIENCIA");

            map.insert("lbl_situation", "I. Contexto Operativo");
            map.insert("ph_situation", "Describa el problema, el conflicto o el escenario...");
            map.insert("lbl_cv", "II. Perfil del Agente (CV)");
            map.insert("ph_cv", "Trayectoria, antecedentes...");
            map.insert("lbl_focus", "III. Foco Estratégico");
            map.insert("ph_focus", "¿Dudas concretas? ¿Qué quiere resolver?");

            map.insert("lbl_target", "Nombre/Rol del Sujeto");
            map.insert("lbl_relation", "Relación");
            map.insert("lbl_m_context", "II. Contexto (Presión, Cultura)");
            map.insert("ph_m_context", "¿Qué está en juego? ¿Reglas no escritas?");
            map.insert("lbl_m_obs", "III. Observaciones (Conducta)");
            map.insert("ph_m_obs", "Bajo estrés hace X... Cuando se le corrige hace Y...");
            map.insert("lbl_m_goal", "IV. Objetivo Táctico");
            map.insert("ph_m_goal", "Alinear equipo, negociar subida, desescalar...");
            
            map.insert("lbl_signals", "V. Señales Rápidas (Drivers)");
            map.insert("scale_desc", "Valore la intensidad de la necesidad/motor:");
            map.insert("scale_low", "1 = Irrelevante / Aversión");
            map.insert("scale_high", "5 = Crítico / Obsesión");

            map.insert("rel_boss", "Superior / Jefe");
            map.insert("rel_team", "Reporte Directo / Equipo");
            map.insert("rel_peer", "Par / Colaborador");
            map.insert("rel_client", "Cliente / Stakeholder");
            map.insert("rel_provider", "Proveedor / Socio");
            map.insert("rel_adversary", "Adversario / Rival");
            map.insert("rel_regulator", "Regulador / Autoridad");
            map.insert("rel_mentor", "Mentor / Consejero");
            map.insert("rel_other", "Otro / Complejo");

            map.insert("sig_security", "Seguridad/Certeza");
            map.insert("sig_belonging", "Pertenencia (Tribu)");
            map.insert("sig_status", "Estatus/Prestigio");
            map.insert("sig_autonomy", "Autonomía/Libertad");
            map.insert("sig_mastery", "Maestría/Competencia");
            map.insert("sig_justice", "Justicia/Equidad");
            map.insert("sig_purpose", "Propósito/Sentido");
            map.insert("sig_control", "Poder/Control");
            map.insert("sig_curiosity", "Curiosidad/Novedad");
            map.insert("sig_comfort", "Confort/Ahorro");
        }
    };
    map
}