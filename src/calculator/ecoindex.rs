use crate::models::report::{Grade, Metrics, ResourceInfo, ScanResult};
use crate::utils::country_data;
use chrono::Local;

/// Calcule le score EcoIndex (0 à 100)
/// Formule simplifiée basée sur les 3 métriques officielles
pub fn calculate_ecoindex(page_size_kb: f64, nb_requests: usize, dom_size: usize) -> f64 {
    // Normalisation des métriques (valeurs de référence EcoIndex)
    let size_norm = (page_size_kb / 2000.0).min(1.0);
    let req_norm = (nb_requests as f64 / 100.0).min(1.0);
    let dom_norm = (dom_size as f64 / 1500.0).min(1.0);

    // Formule pondérée (taille 40%, requêtes 35%, DOM 25%)
    let weighted = (size_norm * 0.40) + (req_norm * 0.35) + (dom_norm * 0.25);
    
    // Score de 0 à 100
    let score = (100.0 - (weighted * 100.0)).max(0.0).min(100.0);
    
    // Arrondi à 2 décimales
    (score * 100.0).round() / 100.0
}

/// Estime l'empreinte carbone en grammes de CO2eq
pub fn estimate_carbon(page_size_kb: f64, emission_factor: f64) -> f64 {
    // Hypothèse : 1 kWh permet de transférer ~1 Go de données
    // 1 Ko = 0.000001 Go = 0.000001 kWh
    let energy_kwh = page_size_kb * 0.000001;
    energy_kwh * emission_factor
}

/// Détermine le type de ressource à partir de l'URL
pub fn get_resource_type(url: &str) -> &str {
    let lower = url.to_lowercase();
    if lower.ends_with(".js") || lower.contains(".js?") {
        "JavaScript"
    } else if lower.ends_with(".css") || lower.contains(".css?") {
        "CSS"
    } else if lower.ends_with(".png") || lower.ends_with(".jpg") 
        || lower.ends_with(".jpeg") || lower.ends_with(".gif")
        || lower.ends_with(".webp") || lower.ends_with(".svg") {
        "Image"
    } else if lower.ends_with(".woff") || lower.ends_with(".woff2") 
        || lower.ends_with(".ttf") || lower.ends_with(".eot") {
        "Police"
    } else if lower.ends_with(".html") || lower.ends_with(".htm") {
        "HTML"
    } else {
        "Autre"
    }
}

/// Génère une recommandation selon le type et la taille
pub fn get_recommendation(resource_type: &str, size_kb: f64) -> String {
    match resource_type {
        "JavaScript" => {
            if size_kb > 500.0 {
                "Activer la minification et la compression Brotli/Gzip. Envisager le code splitting.".to_string()
            } else {
                "Verifier la minification et la mise en cache.".to_string()
            }
        }
        "CSS" => {
            if size_kb > 200.0 {
                "Supprimer les regles inutilisees (PurgeCSS) et compresser.".to_string()
            } else {
                "Minifier le fichier CSS.".to_string()
            }
        }
        "Image" => {
            if size_kb > 300.0 {
                "Convertir en format WebP/AVIF, redimensionner et compresser.".to_string()
            } else {
                "Optimiser la compression et utiliser le lazy loading.".to_string()
            }
        }
        "Police" => {
            "Utiliser font-display: swap et ne charger que les caracteres necessaires.".to_string()
        }
        "HTML" => {
            "Minifier le HTML et activer la compression Gzip.".to_string()
        }
        _ => "Verifier la necessite de cette ressource.".to_string()
    }
}

/// Construit le resultat complet du scan
pub fn build_scan_result(
    url: &str,
    country: Option<String>,
    page_size_kb: f64,
    nb_requests: usize,
    dom_size: usize,
    resources: Vec<(String, f64)>,
) -> ScanResult {
    let emission_factor = match &country {
        Some(code) => country_data::get_emission_factor(code),
        None => country_data::load_country_factors().world_average,
    };

    let ecoindex_score = calculate_ecoindex(page_size_kb, nb_requests, dom_size);
    let grade = Grade::from_score(ecoindex_score);
    let estimated_carbon = estimate_carbon(page_size_kb, emission_factor);

    // CORRECTION 1 : On calcule production_ready AVANT de déplacer 'grade' dans la struct
    let production_ready = matches!(grade, Grade::A | Grade::B);

    // Tri des ressources par taille décroissante pour trouver les plus lourdes
    let mut sorted_resources = resources;
    sorted_resources.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let heaviest_resources: Vec<ResourceInfo> = sorted_resources
        .into_iter()
        .take(5) // Top 5 des ressources les plus lourdes
        .map(|(res_url, size_kb)| {
            let res_type = get_resource_type(&res_url);
            ResourceInfo {
                // CORRECTION 2 : On clone res_url pour éviter le conflit d'emprunt
                url: res_url.clone(),
                size_kb,
                resource_type: res_type.to_string(),
                recommendation: get_recommendation(res_type, size_kb),
            }
        })
        .collect();

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    ScanResult {
        url: url.to_string(),
        country,
        emission_factor,
        metrics: Metrics {
            page_size_kb,
            nb_requests,
            dom_size,
            estimated_carbon_g: estimated_carbon,
        },
        ecoindex_score,
        grade,
        production_ready, // On utilise la variable pré-calculée
        heaviest_resources,
        timestamp,
    }
}