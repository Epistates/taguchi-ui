//! DOE Analysis Commands
//!
//! Thin wrapper around the taguchi library's DOE analysis module.
//! Adds UI-specific metadata (factor IDs, names, timestamps) to library results.

use std::collections::HashMap;

use ndarray::Array2;
use taguchi::doe::{self, AnalysisConfig, OptimizationType as LibOptType};
use taguchi::oa::{OA, OAParams};

use crate::types::{
    ANOVAEntry, ANOVAResult, ConfidenceInterval, DOEAnalysis, DOEAnalysisRequest, MainEffect,
    OptimalSettings, OptimizationType, SNRatioEffect,
};

/// Main entry point for DOE analysis
#[tauri::command]
pub fn run_doe_analysis(request: DOEAnalysisRequest) -> Result<DOEAnalysis, String> {
    // Validate inputs
    if request.array_data.is_empty() {
        return Err("Array data is empty".to_string());
    }
    if request.response_data.is_empty() {
        return Err("Response data is empty".to_string());
    }
    if request.array_data.len() != request.response_data.len() {
        return Err("Array data and response data must have same number of runs".to_string());
    }

    let num_runs = request.array_data.len();
    let num_factors = request.array_data[0].len();

    if request.factor_ids.len() != num_factors {
        return Err("Number of factor IDs must match number of columns".to_string());
    }
    if request.factor_names.len() != num_factors {
        return Err("Number of factor names must match number of columns".to_string());
    }

    // Determine levels per factor from the array data
    let levels_per_factor: Vec<u32> = (0..num_factors)
        .map(|col| {
            let mut levels: Vec<u32> = request.array_data.iter().map(|row| row[col]).collect();
            levels.sort();
            levels.dedup();
            levels.len() as u32
        })
        .collect();

    // Convert Vec<Vec<u32>> to Array2<u32>
    let array_2d = convert_to_array2(&request.array_data)
        .map_err(|e| format!("Failed to convert array data: {}", e))?;

    // Create OA params and OA
    let params = OAParams::new_mixed(num_runs, levels_per_factor, 2)
        .map_err(|e| format!("Invalid OA parameters: {}", e))?;
    let oa = OA::try_new(array_2d, params)
        .map_err(|e| format!("Failed to create OA: {}", e))?;

    // Convert optimization type
    let lib_opt_type = match request.optimization_type {
        OptimizationType::LargerIsBetter => LibOptType::LargerIsBetter,
        OptimizationType::SmallerIsBetter => LibOptType::SmallerIsBetter,
        OptimizationType::NominalIsBest => LibOptType::NominalIsBest,
    };

    // Configure analysis using request settings with defaults
    let config = AnalysisConfig {
        optimization_type: lib_opt_type,
        target_value: request.target_value,
        pooling_threshold: request.pooling_threshold.unwrap_or(2.0),
        enable_pooling: request.enable_pooling.unwrap_or(true),
        min_unpooled_factors: request.min_unpooled_factors.unwrap_or(1),
        confidence_level: request.confidence_level.unwrap_or(0.95),
    };

    // Run analysis using the library
    let lib_result = doe::analyze(&oa, &request.response_data, &config)
        .map_err(|e| format!("Analysis failed: {}", e))?;

    // Map library results to UI types
    let main_effects = map_main_effects(&lib_result.main_effects, &request.factor_ids, &request.factor_names);
    let sn_ratio_effects = map_sn_ratio_effects(&lib_result.sn_ratio_effects, &request.factor_ids, &request.factor_names);
    let anova = map_anova_result(&lib_result.anova, &request.factor_ids, &request.factor_names);
    let optimal_settings = map_optimal_settings(&lib_result.optimal_settings, &request.factor_ids);

    Ok(DOEAnalysis {
        config_id: String::new(), // Will be set by frontend
        grand_mean: lib_result.grand_mean,
        sn_grand_mean: lib_result.sn_grand_mean,
        main_effects,
        sn_ratio_effects,
        anova,
        optimal_settings,
        analyzed_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Convert Vec<Vec<u32>> to ndarray Array2<u32>
fn convert_to_array2(data: &[Vec<u32>]) -> Result<Array2<u32>, String> {
    if data.is_empty() {
        return Err("Empty data".to_string());
    }

    let rows = data.len();
    let cols = data[0].len();

    // Verify all rows have same length
    for (i, row) in data.iter().enumerate() {
        if row.len() != cols {
            return Err(format!("Row {} has {} columns, expected {}", i, row.len(), cols));
        }
    }

    // Flatten and create array
    let flat: Vec<u32> = data.iter().flat_map(|row| row.iter().copied()).collect();
    Array2::from_shape_vec((rows, cols), flat)
        .map_err(|e| format!("Failed to create array: {}", e))
}

/// Map library MainEffect to UI MainEffect
fn map_main_effects(
    effects: &[doe::MainEffect],
    factor_ids: &[String],
    factor_names: &[String],
) -> Vec<MainEffect> {
    effects
        .iter()
        .map(|e| MainEffect {
            factor_id: factor_ids[e.factor_index].clone(),
            factor_name: factor_names[e.factor_index].clone(),
            level_means: e.level_means.clone(),
            level_effects: e.level_effects.clone(),
            range: e.range,
            rank: e.rank,
        })
        .collect()
}

/// Map library SNRatioEffect to UI SNRatioEffect
fn map_sn_ratio_effects(
    effects: &[doe::SNRatioEffect],
    factor_ids: &[String],
    factor_names: &[String],
) -> Vec<SNRatioEffect> {
    effects
        .iter()
        .map(|e| SNRatioEffect {
            factor_id: factor_ids[e.factor_index].clone(),
            factor_name: factor_names[e.factor_index].clone(),
            level_sn_ratios: e.level_sn_ratios.clone(),
            optimal_level: e.optimal_level,
        })
        .collect()
}

/// Map library ANOVAResult to UI ANOVAResult
fn map_anova_result(
    result: &doe::ANOVAResult,
    factor_ids: &[String],
    factor_names: &[String],
) -> ANOVAResult {
    let entries = result
        .entries
        .iter()
        .map(|e| ANOVAEntry {
            factor_id: factor_ids[e.factor_index].clone(),
            factor_name: factor_names[e.factor_index].clone(),
            sum_of_squares: e.sum_of_squares,
            degrees_of_freedom: e.degrees_of_freedom,
            mean_square: e.mean_square,
            f_ratio: e.f_ratio,
            p_value: e.p_value,
            contribution_percent: e.contribution_percent,
            pooled: e.pooled,
        })
        .collect();

    ANOVAResult {
        entries,
        error_ss: result.error_ss,
        error_df: result.error_df,
        error_ms: result.error_ms,
        total_ss: result.total_ss,
        total_df: result.total_df,
    }
}

/// Map library OptimalSettings to UI OptimalSettings
fn map_optimal_settings(
    settings: &doe::OptimalSettings,
    factor_ids: &[String],
) -> OptimalSettings {
    // Convert Vec<usize> to HashMap<String, usize>
    let factor_levels: HashMap<String, usize> = settings
        .factor_levels
        .iter()
        .enumerate()
        .map(|(idx, &level)| (factor_ids[idx].clone(), level))
        .collect();

    let confidence_interval = settings.confidence_interval.map(|ci| ConfidenceInterval {
        lower: ci.lower,
        upper: ci.upper,
        level: ci.level,
    });

    OptimalSettings {
        factor_levels,
        predicted_mean: settings.predicted_mean,
        predicted_sn_ratio: settings.predicted_sn_ratio,
        confidence_interval,
    }
}
