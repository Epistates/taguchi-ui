//! Export and import commands for orthogonal arrays.

use crate::types::OAData;
use std::path::PathBuf;

/// Export an array to CSV format.
#[tauri::command]
pub async fn export_csv(data: OAData, path: PathBuf) -> Result<(), String> {
    let mut csv_content = String::new();

    // Header row
    let headers: Vec<String> = (1..=data.factors)
        .map(|i| format!("Factor{}", i))
        .collect();
    csv_content.push_str(&headers.join(","));
    csv_content.push('\n');

    // Data rows
    for row in &data.data {
        let row_str: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        csv_content.push_str(&row_str.join(","));
        csv_content.push('\n');
    }

    std::fs::write(&path, csv_content).map_err(|e| format!("Failed to write CSV: {}", e))?;

    Ok(())
}

/// Export an array to JSON format.
#[tauri::command]
pub async fn export_json(data: OAData, path: PathBuf) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    std::fs::write(&path, json).map_err(|e| format!("Failed to write JSON: {}", e))?;

    Ok(())
}

/// Export an array to LaTeX tabular format.
#[tauri::command]
pub fn export_latex(data: OAData) -> Result<String, String> {
    let mut latex = String::new();

    // Begin tabular environment
    let col_spec = format!("|{}|", "c|".repeat(data.factors));
    latex.push_str(&format!("\\begin{{tabular}}{{{}}}\n", col_spec));
    latex.push_str("\\hline\n");

    // Header row
    let headers: Vec<String> = (1..=data.factors)
        .map(|i| format!("$F_{{{}}}$", i))
        .collect();
    latex.push_str(&headers.join(" & "));
    latex.push_str(" \\\\\n\\hline\n");

    // Data rows
    for row in &data.data {
        let row_str: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        latex.push_str(&row_str.join(" & "));
        latex.push_str(" \\\\\n");
    }

    latex.push_str("\\hline\n");
    latex.push_str("\\end{tabular}\n");

    // Add caption with array notation
    let levels = if data.levels.len() == 1 {
        data.levels[0].to_string()
    } else {
        format!("({})", data.levels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(","))
    };

    latex.push_str(&format!(
        "\n% OA({}, {}, {}, {})\n",
        data.runs, data.factors, levels, data.strength
    ));

    if let Some(name) = &data.metadata.name {
        latex.push_str(&format!("% Name: {}\n", name));
    }
    latex.push_str(&format!("% Algorithm: {}\n", data.metadata.algorithm));

    Ok(latex)
}

/// Import an array from CSV file.
#[tauri::command]
pub async fn import_csv(path: PathBuf) -> Result<Vec<Vec<u32>>, String> {
    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut data: Vec<Vec<u32>> = Vec::new();
    let mut lines = content.lines();

    // Skip header if present (check if first line contains non-numeric values)
    if let Some(first_line) = lines.next() {
        let first_row: Result<Vec<u32>, _> = first_line
            .split(',')
            .map(|s| s.trim().parse::<u32>())
            .collect();

        match first_row {
            Ok(row) => data.push(row),
            Err(_) => {} // Skip header row
        }
    }

    // Parse remaining rows
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let row: Vec<u32> = line
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid value '{}': {}", s, e))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if !data.is_empty() && row.len() != data[0].len() {
            return Err(format!(
                "Inconsistent row length: expected {}, got {}",
                data[0].len(),
                row.len()
            ));
        }

        data.push(row);
    }

    if data.is_empty() {
        return Err("No data found in file".to_string());
    }

    Ok(data)
}

/// Import an array from JSON file.
#[tauri::command]
pub async fn import_json(path: PathBuf) -> Result<OAData, String> {
    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
}

/// Validate imported array data.
#[tauri::command]
pub fn validate_import(data: Vec<Vec<u32>>) -> Result<crate::types::ImportValidation, String> {
    if data.is_empty() {
        return Err("Array data is empty".to_string());
    }

    let runs = data.len();
    let factors = data[0].len();

    // Check all rows have same length
    for (i, row) in data.iter().enumerate() {
        if row.len() != factors {
            return Err(format!(
                "Row {} has {} columns, expected {}",
                i + 1,
                row.len(),
                factors
            ));
        }
    }

    // Detect levels per factor
    let mut levels = vec![0u32; factors];
    for col in 0..factors {
        let max_val = data.iter().map(|row| row[col]).max().unwrap_or(0);
        levels[col] = max_val + 1;
    }

    // Check for consistent levels (pure vs mixed)
    let is_mixed = levels.iter().collect::<std::collections::HashSet<_>>().len() > 1;

    // Estimate strength by checking balance
    let estimated_strength = estimate_strength(&data, &levels);

    Ok(crate::types::ImportValidation {
        runs,
        factors,
        levels: levels.clone(),
        is_mixed,
        estimated_strength,
        warnings: generate_warnings(&data, &levels),
    })
}

fn estimate_strength(data: &[Vec<u32>], levels: &[u32]) -> u32 {
    // Simple heuristic: check if runs match expected for various strengths
    let runs = data.len();

    // For strength 2, runs should be >= s^2 where s is max level
    let max_level = *levels.iter().max().unwrap_or(&2) as usize;

    if runs >= max_level.pow(3) {
        3
    } else if runs >= max_level.pow(2) {
        2
    } else {
        1
    }
}

fn generate_warnings(data: &[Vec<u32>], levels: &[u32]) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check for unbalanced factors
    for (col, &level_count) in levels.iter().enumerate() {
        let mut counts = std::collections::HashMap::new();
        for row in data {
            *counts.entry(row[col]).or_insert(0) += 1;
        }

        let expected = data.len() / level_count as usize;
        let is_balanced = counts.values().all(|&c| c == expected);

        if !is_balanced {
            warnings.push(format!("Factor {} may not be balanced", col + 1));
        }
    }

    // Check for unusually large or small arrays
    if data.len() < 4 {
        warnings.push("Array has very few runs".to_string());
    }

    if data[0].len() > 50 {
        warnings.push("Array has many factors - analysis may be slow".to_string());
    }

    warnings
}
