//! OA Builder commands.

use crate::types::{BuildRequest, ConstructionOption, LevelSpec, OAData, OAMetadata, ValidationResult};
use chrono::Utc;
use taguchi::{available_constructions, OABuilder};
use uuid::Uuid;

/// Build an orthogonal array with automatic algorithm selection.
#[tauri::command]
pub fn build_oa(request: BuildRequest) -> Result<OAData, String> {
    let mut builder = OABuilder::new();

    // Set levels
    match &request.levels {
        LevelSpec::Symmetric(s) => {
            builder = builder.levels(*s);
        }
        LevelSpec::Mixed(levels) => {
            builder = builder.mixed_levels(levels.clone());
        }
    }

    // Set factors and strength
    builder = builder.factors(request.factors).strength(request.strength);

    // Set minimum runs if specified
    if let Some(min_runs) = request.min_runs {
        builder = builder.min_runs(min_runs);
    }

    // Build the array
    let oa = builder.build().map_err(|e| e.to_string())?;

    // Convert to frontend-friendly format
    let data: Vec<Vec<u32>> = (0..oa.runs())
        .map(|r| oa.row(r).iter().copied().collect())
        .collect();

    Ok(OAData {
        id: Uuid::new_v4().to_string(),
        runs: oa.runs(),
        factors: oa.factors(),
        levels: oa.levels_vec().to_vec(),
        strength: oa.strength(),
        data,
        metadata: OAMetadata {
            name: None,
            algorithm: detect_algorithm(&oa),
            created_at: Utc::now().to_rfc3339(),
            notes: None,
        },
    })
}

/// Get available constructions for given parameters.
#[tauri::command]
pub fn get_available_constructions(levels: u32, strength: u32) -> Vec<ConstructionOption> {
    let constructions = available_constructions(levels, strength);

    constructions
        .into_iter()
        .map(|(name, runs, max_factors)| ConstructionOption {
            name: name.to_string(),
            runs,
            max_factors,
            description: get_construction_description(&name),
            constraints: get_construction_constraints(&name, levels),
        })
        .collect()
}

/// Validate build parameters before construction.
#[tauri::command]
pub fn validate_build_params(request: BuildRequest) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Extract the symmetric level if applicable
    let levels = match &request.levels {
        LevelSpec::Symmetric(s) => *s,
        LevelSpec::Mixed(lvls) => {
            if lvls.is_empty() {
                errors.push("At least one level must be specified".to_string());
                return ValidationResult {
                    valid: false,
                    errors,
                    warnings,
                    suggestions: vec![],
                };
            }
            // For mixed levels, use max for construction lookup
            *lvls.iter().max().unwrap()
        }
    };

    // Basic validation
    if levels < 2 {
        errors.push("Levels must be at least 2".to_string());
    }

    if request.factors < 1 {
        errors.push("Factors must be at least 1".to_string());
    }

    if request.strength as usize > request.factors {
        errors.push(format!(
            "Strength {} cannot exceed factors {}",
            request.strength, request.factors
        ));
    }

    // Check if prime power
    if !taguchi::is_prime_power(levels) {
        warnings.push(format!(
            "Levels {} is not a prime power - limited constructions available",
            levels
        ));
    }

    // Get available constructions
    let suggestions = if errors.is_empty() {
        available_constructions(levels, request.strength)
            .into_iter()
            .filter(|(_, _, max_factors)| *max_factors >= request.factors)
            .map(|(name, runs, max_factors)| ConstructionOption {
                name: name.to_string(),
                runs,
                max_factors,
                description: get_construction_description(&name),
                constraints: get_construction_constraints(&name, levels),
            })
            .collect()
    } else {
        vec![]
    };

    if suggestions.is_empty() && errors.is_empty() {
        errors.push(format!(
            "No construction available for {} levels, {} factors, strength {}",
            levels, request.factors, request.strength
        ));
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings,
        suggestions,
    }
}

// Helper functions

fn detect_algorithm(oa: &taguchi::oa::OA) -> String {
    // Heuristic based on array properties
    let runs = oa.runs();
    let factors = oa.factors();
    let levels = oa.levels();

    if levels == 2 {
        if runs.is_power_of_two() {
            return "Hadamard-Sylvester".to_string();
        }
        if runs > 1 && taguchi::is_prime((runs - 1) as u32) {
            return "Hadamard-Paley".to_string();
        }
    }

    let q_squared = (levels as usize).pow(2);
    if runs == q_squared && factors <= (levels as usize) + 1 {
        return "Bose".to_string();
    }

    if runs == 2 * q_squared && factors <= 2 * (levels as usize) + 1 {
        if levels == 2 {
            return "Bose-Bush".to_string();
        }
        return "Addelman-Kempthorne".to_string();
    }

    "Unknown".to_string()
}

fn get_construction_description(name: &str) -> String {
    match name {
        "Bose" => "Primary construction for strength 2 arrays".to_string(),
        "Bush" => "Higher strength arrays (t >= 2)".to_string(),
        "Bose-Bush" => "Extended Bose for binary (2 level) arrays".to_string(),
        "Hadamard-Sylvester" => "Binary arrays from Hadamard matrices".to_string(),
        "Hadamard-Paley" => "Binary arrays using Paley construction".to_string(),
        "Addelman-Kempthorne" => "Extended construction for odd prime powers".to_string(),
        "Rao-Hamming" => "Arrays from linear codes".to_string(),
        _ => "Construction method".to_string(),
    }
}

fn get_construction_constraints(name: &str, levels: u32) -> Vec<String> {
    let mut constraints = vec![];

    match name {
        "Bose" => {
            constraints.push(format!("Requires {} to be a prime power", levels));
            constraints.push(format!("Max {} factors", levels + 1));
        }
        "Bush" => {
            constraints.push(format!("Requires {} to be a prime power", levels));
        }
        "Bose-Bush" => {
            constraints.push("Only for 2 levels".to_string());
        }
        "Hadamard-Sylvester" => {
            constraints.push("Only for 2 levels".to_string());
            constraints.push("Runs must be power of 2".to_string());
        }
        "Hadamard-Paley" => {
            constraints.push("Only for 2 levels".to_string());
            constraints.push("Requires (runs-1) to be prime ≡ 3 (mod 4)".to_string());
        }
        "Addelman-Kempthorne" => {
            constraints.push("Requires odd prime power levels".to_string());
        }
        _ => {}
    }

    constraints
}
