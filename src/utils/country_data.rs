use serde::Deserialize;
use std::collections::HashMap;

/// Facteur d'émission en gCO2eq/kWh par code pays ISO
#[derive(Deserialize, Debug)]
pub struct CountryFactors {
    pub countries: HashMap<String, f64>,
    pub world_average: f64,
}

/// Charge les facteurs d'émission embarqués dans le binaire
pub fn load_country_factors() -> CountryFactors {
    let json_data = include_str!("../../data/country_factors.json");
    serde_json::from_str(json_data).expect("Erreur lors du chargement des facteurs pays")
}

/// Récupère le facteur d'émission pour un code pays (ex: "TG")
/// Retourne la moyenne mondiale si le pays n'est pas trouvé
pub fn get_emission_factor(country_code: &str) -> f64 {
    let factors = load_country_factors();
    let code = country_code.to_uppercase();
    
    match factors.countries.get(&code) {
        Some(factor) => *factor,
        None => factors.world_average,
    }
}