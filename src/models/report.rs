use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub url: String,
    pub country: Option<String>,
    pub emission_factor: f64,
    pub metrics: Metrics,
    pub ecoindex_score: f64,
    pub grade: Grade,
    pub production_ready: bool,
    pub heaviest_resources: Vec<ResourceInfo>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub page_size_kb: f64,
    pub nb_requests: usize,
    pub dom_size: usize,
    pub estimated_carbon_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub url: String,
    pub size_kb: f64,
    pub resource_type: String,
    pub recommendation: String,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Grade {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 80.0 => Grade::A,
            s if s >= 65.0 => Grade::B,
            s if s >= 50.0 => Grade::C,
            s if s >= 35.0 => Grade::D,
            s if s >= 20.0 => Grade::E,
            _ => Grade::F,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::E => "E",
            Grade::F => "F",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Grade::A => "EXCELLENT - Empreinte minimale",
            Grade::B => "BON - Quelques optimisations possibles",
            Grade::C => "MOYEN - Impact notable",
            Grade::D => "MEDIOCRE - Forte empreinte carbone",
            Grade::E => "MAUVAIS - Impact environnemental critique",
            Grade::F => "CRITIQUE - Catastrophe ecologique numerique",
        }
    }

    pub fn production_message(&self) -> (&str, bool) {
        match self {
            Grade::A => ("Autorise pour la production.", true),
            Grade::B => ("Autorise pour la production.", true),
            Grade::C => ("Optimisez avant la mise en production.", false),
            Grade::D => ("Deconseille sans refonte.", false),
            Grade::E => ("Interdiction de deploiement.", false),
            Grade::F => ("Refonte totale requise.", false),
        }
    }
}