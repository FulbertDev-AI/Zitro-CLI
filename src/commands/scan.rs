use anyhow::Result;
use colored::Colorize;
use reqwest;
use scraper::{Html, Selector};
use std::fs;
use std::path::Path;

use crate::calculator::ecoindex;
use crate::models::report::ScanResult;

/// Exécute un scan complet sur une URL
pub async fn execute(url: &str, country: Option<String>) -> Result<()> {
    println!("Connexion a {} ...", url);

    // 1. Requête HTTP principale
    let client = reqwest::Client::builder()
        .user_agent("ZitroCLI/1.0.0 (Carbon Auditor)")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Erreur HTTP : {}. Verifiez que l'URL est correcte et que le serveur est actif.", response.status());
    }

    let html_content = response.text().await?;
    let page_size_bytes = html_content.len();
    
    println!("Page HTML recuperee : {:.2} Ko", page_size_bytes as f64 / 1024.0);

    // 2. Parsing du DOM
    let document = Html::parse_document(&html_content);
    let dom_size = document.tree.root().descendants().count();
    println!("Taille du DOM : {} noeuds", dom_size);

    // 3. Extraction des ressources (CSS, JS, images)
    let mut resources: Vec<(String, f64)> = Vec::new();
    resources.push((url.to_string(), page_size_bytes as f64 / 1024.0));

    let mut total_size_bytes = page_size_bytes;
    let mut nb_requests = 1;

    // Extraction des liens CSS
    if let Ok(css_selector) = Selector::parse("link[rel='stylesheet']") {
        for element in document.select(&css_selector) {
            if let Some(href) = element.value().attr("href") {
                let full_url = resolve_url(url, href);
                if let Ok(size) = fetch_resource_size(&client, &full_url).await {
                    resources.push((full_url, size as f64 / 1024.0));
                    total_size_bytes += size;
                    nb_requests += 1;
                }
            }
        }
    }

    // Extraction des scripts JS
    if let Ok(js_selector) = Selector::parse("script[src]") {
        for element in document.select(&js_selector) {
            if let Some(src) = element.value().attr("src") {
                let full_url = resolve_url(url, src);
                if let Ok(size) = fetch_resource_size(&client, &full_url).await {
                    resources.push((full_url, size as f64 / 1024.0));
                    total_size_bytes += size;
                    nb_requests += 1;
                }
            }
        }
    }

    // Extraction des images
    if let Ok(img_selector) = Selector::parse("img[src]") {
        for element in document.select(&img_selector) {
            if let Some(src) = element.value().attr("src") {
                if src.starts_with("data:") {
                    continue; // On ignore les images en base64
                }
                let full_url = resolve_url(url, src);
                if let Ok(size) = fetch_resource_size(&client, &full_url).await {
                    resources.push((full_url, size as f64 / 1024.0));
                    total_size_bytes += size;
                    nb_requests += 1;
                }
            }
        }
    }

    println!("Ressources analysees : {}", nb_requests);
    println!("Poids total : {:.2} Ko", total_size_bytes as f64 / 1024.0);
    println!();

    // 4. Calcul EcoIndex
    let result = ecoindex::build_scan_result(
        url,
        country.clone(),
        total_size_bytes as f64 / 1024.0,
        nb_requests,
        dom_size,
        resources,
    );

    // 5. Affichage des résultats dans le terminal
    display_results(&result);

    // 6. Génération du rapport Texte (.txt)
    let report_path = generate_text_report(&result)?;
    println!();
    println!("Rapport texte sauvegarde a l'emplacement :");
    println!("> {}", report_path.green().bold());
    println!("(Vous pouvez copier ce chemin pour ouvrir ou deplacer le fichier)");

    Ok(())
}

/// Récupère la taille d'une ressource distante
async fn fetch_resource_size(client: &reqwest::Client, url: &str) -> Result<usize> {
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Ok(0);
    }
    let bytes = response.bytes().await?;
    Ok(bytes.len())
}

/// Résout une URL relative en URL absolue
fn resolve_url(base: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") {
        return relative.to_string();
    }
    if let Ok(base_url) = url::Url::parse(base) {
        if let Ok(full_url) = base_url.join(relative) {
            return full_url.to_string();
        }
    }
    relative.to_string()
}

/// Affiche les résultats dans le terminal (avec couleurs)
fn display_results(result: &ScanResult) {
    println!("============================================================");
    println!("  RESULTATS DE L'AUDIT");
    println!("============================================================");
    println!();
    println!("URL auditee      : {}", result.url);
    println!("Mix energetique  : {} gCO2eq/kWh", result.emission_factor);
    if let Some(code) = &result.country {
        println!("Pays cible       : {}", code.to_uppercase());
    } else {
        println!("Pays cible       : International (moyenne mondiale)");
    }
    println!();
    println!("--- Metriques ---");
    println!("Poids total      : {:.2} Ko", result.metrics.page_size_kb);
    println!("Nombre requetes  : {}", result.metrics.nb_requests);
    println!("Taille DOM       : {} noeuds", result.metrics.dom_size);
    println!("Carbone estime   : {:.6} g CO2eq", result.metrics.estimated_carbon_g);
    println!();
    println!("--- Score EcoIndex ---");
    println!("Score            : {:.2} / 100", result.ecoindex_score);
    println!("Note             : {}", result.grade.label());
    println!("Appreciation     : {}", result.grade.message());
    println!();
    
    let (prod_msg, ready) = result.grade.production_message();
    if ready {
        println!("Production       : {}", "AUTORISE".green().bold());
    } else {
        println!("Production       : {}", "DECONSEILLE".red().bold());
    }
    println!("                 {}", prod_msg);
    println!("============================================================");
}

/// Génère un rapport Texte (.txt) sobre et lisible
fn generate_text_report(result: &ScanResult) -> Result<String> {
    // Création du dossier zitro-reports s'il n'existe pas
    let reports_dir = Path::new("zitro-reports");
    if !reports_dir.exists() {
        fs::create_dir_all(reports_dir)?;
    }

    // Nom de fichier sécurisé
    let safe_filename = result.url
        .replace("://", "_")
        .replace('/', "_")
        .replace(':', "_")
        .replace('.', "_");
    
    let filename = format!("zitro-rapport-{}.txt", safe_filename);
    let file_path = reports_dir.join(&filename);
    let absolute_path = file_path.canonicalize()?.to_string_lossy().to_string();
    
    let (prod_msg, ready) = result.grade.production_message();
    let prod_status = if ready { "AUTORISE" } else { "DECONSEILLE" };

    let mut report = String::new();
    report.push_str("============================================================\n");
    report.push_str("        RAPPORT D'AUDIT ZITRO CLI\n");
    report.push_str("============================================================\n\n");
    
    report.push_str(&format!("Date : {}\n", result.timestamp));
    report.push_str(&format!("URL auditee : {}\n", result.url));
    report.push_str(&format!("Mix energetique : {} gCO2eq/kWh\n", result.emission_factor));
    if let Some(code) = &result.country {
        report.push_str(&format!("Pays cible : {}\n\n", code.to_uppercase()));
    } else {
        report.push_str("Pays cible : International (moyenne mondiale)\n\n");
    }

    report.push_str("--- METRIQUES ---\n");
    report.push_str(&format!("Poids total : {:.2} Ko\n", result.metrics.page_size_kb));
    report.push_str(&format!("Nombre de requetes : {}\n", result.metrics.nb_requests));
    report.push_str(&format!("Taille du DOM : {} noeuds\n", result.metrics.dom_size));
    report.push_str(&format!("Carbone estime : {:.6} g CO2eq\n\n", result.metrics.estimated_carbon_g));

    report.push_str("--- SCORE ECOINDEX ---\n");
    report.push_str(&format!("Score : {:.2} / 100\n", result.ecoindex_score));
    report.push_str(&format!("Note : {}\n", result.grade.label()));
    report.push_str(&format!("Appreciation : {}\n\n", result.grade.message()));

    report.push_str("--- DECISION DE MISE EN PRODUCTION ---\n");
    report.push_str(&format!("Statut : {}\n", prod_status));
    report.push_str(&format!("Message : {}\n\n", prod_msg));

    if !result.heaviest_resources.is_empty() {
        report.push_str("--- RESSOURCES LES PLUS IMPACTANTES ---\n");
        for (i, res) in result.heaviest_resources.iter().enumerate() {
            report.push_str(&format!("{}. {} ({:.2} Ko) - {}\n", 
                i + 1, res.url, res.size_kb, res.resource_type));
            report.push_str(&format!("   -> Action recommandee : {}\n\n", res.recommendation));
        }
    }

    report.push_str("============================================================\n");
    report.push_str("Genere par ZITRO CLI v1.0.0\n");

    fs::write(&file_path, &report)?;
    Ok(absolute_path)
}