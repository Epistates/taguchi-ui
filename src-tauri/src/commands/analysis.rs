//! Analysis and verification commands.

use crate::types::{BalanceData, CorrelationData, VerificationData, VerificationIssue};
use std::collections::HashMap;
use taguchi::oa::{OAParams, OA};

/// Verify an array's strength.
#[tauri::command]
pub fn verify_array(data: Vec<Vec<u32>>, claimed_strength: u32) -> Result<VerificationData, String> {
    // Convert input data to OA
    let oa = data_to_oa(data)?;

    // Run verification
    let result = taguchi::verify_strength(&oa, claimed_strength).map_err(|e| e.to_string())?;

    // Convert issues to our format using Debug representation
    let issues: Vec<VerificationIssue> = result
        .issues
        .iter()
        .map(|issue| {
            let debug_str = format!("{:?}", issue);
            // Parse the debug string to extract info
            if debug_str.contains("ValueOutOfRange") {
                VerificationIssue {
                    issue_type: "Value Out of Range".to_string(),
                    description: debug_str,
                    location: None,
                }
            } else {
                VerificationIssue {
                    issue_type: "Balance Violation".to_string(),
                    description: debug_str,
                    location: None,
                }
            }
        })
        .collect();

    Ok(VerificationData {
        is_valid: result.is_valid,
        claimed_strength,
        actual_strength: result.actual_strength,
        issues,
    })
}

/// Compute the actual strength of an array.
#[tauri::command]
pub fn compute_array_strength(data: Vec<Vec<u32>>, max_check: u32) -> Result<u32, String> {
    let oa = data_to_oa(data)?;
    taguchi::compute_strength(&oa, max_check).map_err(|e| e.to_string())
}

/// Get balance report for an array.
#[tauri::command]
pub fn get_balance_report(data: Vec<Vec<u32>>) -> Result<BalanceData, String> {
    let oa = data_to_oa(data)?;

    // Calculate balance for each factor
    let mut factor_balance = Vec::new();
    let mut level_counts: Vec<HashMap<u32, usize>> = Vec::new();

    for col in 0..oa.factors() {
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for row in 0..oa.runs() {
            let val = oa.get(row, col);
            *counts.entry(val).or_insert(0) += 1;
        }

        // Check if balanced (all counts equal)
        let expected = oa.runs() / (oa.levels_for(col) as usize);
        let is_balanced = counts.values().all(|&c| c == expected);

        factor_balance.push(is_balanced);
        level_counts.push(counts);
    }

    let expected_count = if oa.factors() > 0 {
        oa.runs() / (oa.levels_for(0) as usize)
    } else {
        0
    };

    Ok(BalanceData {
        factor_balance,
        level_counts,
        expected_count,
    })
}

/// Get correlation matrix for an array.
#[tauri::command]
pub fn get_correlation_matrix(data: Vec<Vec<u32>>) -> Result<CorrelationData, String> {
    let oa = data_to_oa(data)?;
    let factors = oa.factors();

    // Calculate correlation between each pair of factors
    let mut matrix = vec![vec![0.0; factors]; factors];

    for i in 0..factors {
        for j in 0..factors {
            if i == j {
                matrix[i][j] = 1.0;
            } else {
                matrix[i][j] = calculate_correlation(&oa, i, j);
            }
        }
    }

    Ok(CorrelationData { matrix, factors })
}

// Helper functions

fn data_to_oa(data: Vec<Vec<u32>>) -> Result<OA, String> {
    if data.is_empty() {
        return Err("Array data cannot be empty".to_string());
    }

    let runs = data.len();
    let factors = data[0].len();

    if factors == 0 {
        return Err("Array must have at least one factor".to_string());
    }

    // Check all rows have same length
    if !data.iter().all(|row| row.len() == factors) {
        return Err("All rows must have the same number of columns".to_string());
    }

    // Detect levels per factor
    let mut levels = vec![0u32; factors];
    for col in 0..factors {
        let max_val = data.iter().map(|row| row[col]).max().unwrap_or(0);
        levels[col] = max_val + 1;
    }

    // Create ndarray
    let flat_data: Vec<u32> = data.into_iter().flatten().collect();
    let array = ndarray::Array2::from_shape_vec((runs, factors), flat_data)
        .map_err(|e| e.to_string())?;

    // Create OA params (assume strength 2 for analysis purposes)
    let params = OAParams::new_mixed(runs, levels, 2).map_err(|e| e.to_string())?;

    Ok(OA::new(array, params))
}

fn calculate_correlation(oa: &OA, col_i: usize, col_j: usize) -> f64 {
    let n = oa.runs() as f64;

    // Calculate means
    let mean_i: f64 = (0..oa.runs())
        .map(|r| oa.get(r, col_i) as f64)
        .sum::<f64>()
        / n;
    let mean_j: f64 = (0..oa.runs())
        .map(|r| oa.get(r, col_j) as f64)
        .sum::<f64>()
        / n;

    // Calculate covariance and standard deviations
    let mut cov = 0.0;
    let mut var_i = 0.0;
    let mut var_j = 0.0;

    for r in 0..oa.runs() {
        let xi = oa.get(r, col_i) as f64 - mean_i;
        let xj = oa.get(r, col_j) as f64 - mean_j;
        cov += xi * xj;
        var_i += xi * xi;
        var_j += xj * xj;
    }

    let denom = (var_i * var_j).sqrt();
    if denom < f64::EPSILON {
        0.0
    } else {
        cov / denom
    }
}
