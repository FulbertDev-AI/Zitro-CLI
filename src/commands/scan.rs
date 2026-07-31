use anyhow::Result;
use colored::Colorize;
use reqwest;
use scraper::{Html, Selector};

use crate::calculator::ecoindex;
use crate::models::report::ScanResult;

/// Exécute un scan complet sur une URL et affiche le résultat dans le terminal
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
    println!("Poids total estime : {:.2} Ko", total_size_bytes as f64 / 1024.0);
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

    // 5. Affichage des résultats dans le terminal (Version simplifiée)
    display_results(&result);

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

/// Affiche les résultats de manière sobre et directe dans le terminal
fn display_results(result: &ScanResult) {
    println!("============================================================");
    println!("  RESULTATS DE L'AUDIT ZITRO");
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