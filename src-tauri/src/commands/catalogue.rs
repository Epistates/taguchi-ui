//! Catalogue commands for standard Taguchi arrays.

use crate::types::{OAData, OAMetadata, StandardArrayInfo};
use chrono::Utc;
use taguchi::get_standard_oa;
use uuid::Uuid;

/// Standard Taguchi arrays metadata.
const STANDARD_ARRAYS: &[(&str, usize, usize, u32, u32, &str)] = &[
    ("L4", 4, 3, 2, 2, "Smallest 2-level array"),
    ("L8", 8, 7, 2, 2, "Common 2-level array"),
    ("L9", 9, 4, 3, 2, "Smallest 3-level array"),
    ("L12", 12, 11, 2, 2, "Plackett-Burman 12-run"),
    ("L16", 16, 15, 2, 2, "16-run 2-level array"),
    ("L18", 18, 7, 3, 2, "Mixed 2/3-level array (modified)"),
    ("L25", 25, 6, 5, 2, "5-level Bose array"),
    ("L27", 27, 13, 3, 2, "Full 3-level array"),
    ("L32", 32, 31, 2, 2, "32-run Hadamard array"),
    ("L36", 36, 11, 6, 2, "6-level array"),
    ("L49", 49, 8, 7, 2, "7-level Bose array"),
    ("L50", 50, 11, 5, 2, "Extended 5-level array"),
    ("L64", 64, 63, 2, 2, "64-run Hadamard array"),
    ("L81", 81, 40, 3, 2, "Large 3-level array"),
];

/// List all standard arrays.
#[tauri::command]
pub fn list_standard_arrays() -> Vec<StandardArrayInfo> {
    STANDARD_ARRAYS
        .iter()
        .map(|&(name, runs, factors, levels, strength, desc)| StandardArrayInfo {
            name: name.to_string(),
            runs,
            factors,
            levels,
            strength,
            description: desc.to_string(),
        })
        .collect()
}

/// Get a standard array by name.
#[tauri::command]
pub fn get_standard_array(name: String) -> Result<OAData, String> {
    let oa = get_standard_oa(&name).map_err(|e| e.to_string())?;

    // Convert to frontend-friendly format
    let data: Vec<Vec<u32>> = (0..oa.runs())
        .map(|r| oa.row(r).iter().copied().collect())
        .collect();

    // Find description from metadata
    let description = STANDARD_ARRAYS
        .iter()
        .find(|&&(n, _, _, _, _, _)| n == name)
        .map(|&(_, _, _, _, _, d)| d.to_string())
        .unwrap_or_default();

    Ok(OAData {
        id: Uuid::new_v4().to_string(),
        runs: oa.runs(),
        factors: oa.factors(),
        levels: oa.levels_vec().to_vec(),
        strength: oa.strength(),
        data,
        metadata: OAMetadata {
            name: Some(format!("{} - {}", name, description)),
            algorithm: "Catalogue".to_string(),
            created_at: Utc::now().to_rfc3339(),
            notes: None,
        },
    })
}

/// Search/filter standard arrays.
#[tauri::command]
pub fn search_catalogue(
    min_runs: Option<usize>,
    max_runs: Option<usize>,
    levels: Option<u32>,
    min_factors: Option<usize>,
) -> Vec<StandardArrayInfo> {
    STANDARD_ARRAYS
        .iter()
        .filter(|&&(_, runs, factors, lvls, _, _)| {
            if let Some(min) = min_runs {
                if runs < min {
                    return false;
                }
            }
            if let Some(max) = max_runs {
                if runs > max {
                    return false;
                }
            }
            if let Some(l) = levels {
                if lvls != l {
                    return false;
                }
            }
            if let Some(min_f) = min_factors {
                if factors < min_f {
                    return false;
                }
            }
            true
        })
        .map(|&(name, runs, factors, levels, strength, desc)| StandardArrayInfo {
            name: name.to_string(),
            runs,
            factors,
            levels,
            strength,
            description: desc.to_string(),
        })
        .collect()
}
