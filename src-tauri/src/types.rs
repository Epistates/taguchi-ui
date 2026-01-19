//! Type definitions for Tauri commands.
//!
//! These types bridge between the taguchi library and the frontend,
//! providing serializable data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an orthogonal array for frontend consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAData {
    /// Unique identifier for this array.
    pub id: String,
    /// Number of runs (rows).
    pub runs: usize,
    /// Number of factors (columns).
    pub factors: usize,
    /// Number of levels per factor.
    pub levels: Vec<u32>,
    /// Strength (orthogonality degree).
    pub strength: u32,
    /// The array data as a 2D vector (row-major).
    pub data: Vec<Vec<u32>>,
    /// Metadata about the array.
    pub metadata: OAMetadata,
}

/// Metadata for an orthogonal array.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAMetadata {
    /// Optional user-provided name.
    pub name: Option<String>,
    /// The construction algorithm used.
    pub algorithm: String,
    /// Creation timestamp (ISO 8601).
    pub created_at: String,
    /// Optional user notes.
    pub notes: Option<String>,
}

/// Request to build an orthogonal array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildRequest {
    /// Number of levels - either a single value (symmetric) or array (mixed).
    pub levels: LevelSpec,
    /// Number of factors.
    pub factors: usize,
    /// Desired strength.
    pub strength: u32,
    /// Optional minimum runs constraint.
    pub min_runs: Option<usize>,
}

/// Level specification - symmetric or mixed.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LevelSpec {
    /// Same number of levels for all factors.
    Symmetric(u32),
    /// Different levels per factor.
    Mixed(Vec<u32>),
}

/// An available construction option.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstructionOption {
    /// Name of the construction algorithm.
    pub name: String,
    /// Number of runs this construction produces.
    pub runs: usize,
    /// Maximum number of factors supported.
    pub max_factors: usize,
    /// Human-readable description.
    pub description: String,
    /// Any constraints or requirements.
    pub constraints: Vec<String>,
}

/// Validation result for build parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    /// Whether the parameters are valid.
    pub valid: bool,
    /// Error messages if invalid.
    pub errors: Vec<String>,
    /// Warning messages.
    pub warnings: Vec<String>,
    /// Suggested constructions.
    pub suggestions: Vec<ConstructionOption>,
}

/// Verification result from checking array strength.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationData {
    /// Whether the array is valid.
    pub is_valid: bool,
    /// The strength that was claimed.
    pub claimed_strength: u32,
    /// The actual verified strength.
    pub actual_strength: u32,
    /// Any issues found.
    pub issues: Vec<VerificationIssue>,
}

/// A verification issue.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationIssue {
    /// Type of issue.
    pub issue_type: String,
    /// Human-readable description.
    pub description: String,
    /// Location in the array, if applicable.
    pub location: Option<IssueLocation>,
}

/// Location of an issue in the array.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLocation {
    /// Row index, if applicable.
    pub row: Option<usize>,
    /// Column index, if applicable.
    pub col: Option<usize>,
    /// Columns involved, if multiple.
    pub columns: Option<Vec<usize>>,
}

/// Balance report for an orthogonal array.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    /// Whether each factor is balanced.
    pub factor_balance: Vec<bool>,
    /// Level counts for each factor.
    pub level_counts: Vec<HashMap<u32, usize>>,
    /// Expected count per level.
    pub expected_count: usize,
}

/// Correlation data between factors.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationData {
    /// Correlation matrix (factors x factors).
    pub matrix: Vec<Vec<f64>>,
    /// Number of factors.
    pub factors: usize,
}

/// Information about a standard (catalogue) array.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardArrayInfo {
    /// Name (e.g., "L9", "L18").
    pub name: String,
    /// Number of runs.
    pub runs: usize,
    /// Number of factors.
    pub factors: usize,
    /// Number of levels (symmetric).
    pub levels: u32,
    /// Strength.
    pub strength: u32,
    /// Human-readable description.
    pub description: String,
}

/// Validation result for imported array data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportValidation {
    /// Number of runs detected.
    pub runs: usize,
    /// Number of factors detected.
    pub factors: usize,
    /// Levels per factor.
    pub levels: Vec<u32>,
    /// Whether this is a mixed-level array.
    pub is_mixed: bool,
    /// Estimated strength.
    pub estimated_strength: u32,
    /// Any warnings about the data.
    pub warnings: Vec<String>,
}

// ========================================
// DOE (Design of Experiments) Types
// ========================================

/// Optimization goal for response variable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OptimizationType {
    /// Higher response values are better.
    LargerIsBetter,
    /// Lower response values are better.
    SmallerIsBetter,
    /// Response should be close to a target value.
    NominalIsBest,
}

// Note: DOE config, factors, responses, measurements are managed in frontend store.
// Only analysis request/response types are needed in Rust for the taguchi library bridge.

/// Main effect analysis for a single factor.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainEffect {
    /// Factor ID.
    pub factor_id: String,
    /// Factor name.
    pub factor_name: String,
    /// Mean response at each level.
    pub level_means: Vec<f64>,
    /// Effect = level mean - grand mean.
    pub level_effects: Vec<f64>,
    /// Range = max - min (factor importance).
    pub range: f64,
    /// Rank (1 = most important).
    pub rank: usize,
}

/// Signal-to-Noise ratio analysis for a single factor.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SNRatioEffect {
    /// Factor ID.
    pub factor_id: String,
    /// Factor name.
    pub factor_name: String,
    /// S/N ratio at each level (in dB).
    pub level_sn_ratios: Vec<f64>,
    /// Level index with best S/N.
    pub optimal_level: usize,
}

/// ANOVA table entry for a factor.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ANOVAEntry {
    /// Factor ID.
    pub factor_id: String,
    /// Factor name.
    pub factor_name: String,
    /// Sum of squares.
    pub sum_of_squares: f64,
    /// Degrees of freedom.
    pub degrees_of_freedom: usize,
    /// Mean square (SS / DF).
    pub mean_square: f64,
    /// F-ratio (MS_factor / MS_error).
    pub f_ratio: Option<f64>,
    /// p-value from F-distribution.
    pub p_value: Option<f64>,
    /// Contribution percentage (SS_factor / SS_total * 100).
    pub contribution_percent: f64,
    /// Whether this factor was pooled into error.
    pub pooled: bool,
}

/// Complete ANOVA results.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ANOVAResult {
    /// ANOVA entries for each factor.
    pub entries: Vec<ANOVAEntry>,
    /// Error sum of squares.
    pub error_ss: f64,
    /// Error degrees of freedom.
    pub error_df: usize,
    /// Error mean square.
    pub error_ms: f64,
    /// Total sum of squares.
    pub total_ss: f64,
    /// Total degrees of freedom.
    pub total_df: usize,
}

/// Confidence interval.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfidenceInterval {
    /// Lower bound.
    pub lower: f64,
    /// Upper bound.
    pub upper: f64,
    /// Confidence level (e.g., 0.95).
    pub level: f64,
}

/// Optimal settings prediction.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimalSettings {
    /// Optimal level index for each factor.
    pub factor_levels: HashMap<String, usize>,
    /// Predicted mean response.
    pub predicted_mean: f64,
    /// Predicted S/N ratio.
    pub predicted_sn_ratio: f64,
    /// Confidence interval for prediction.
    pub confidence_interval: Option<ConfidenceInterval>,
}

/// Complete DOE analysis results.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DOEAnalysis {
    /// Reference to DOEConfig.
    pub config_id: String,
    /// Grand mean of all responses.
    pub grand_mean: f64,
    /// Grand mean of all S/N ratios.
    pub sn_grand_mean: f64,
    /// Main effects for each factor.
    pub main_effects: Vec<MainEffect>,
    /// S/N ratio effects for each factor.
    pub sn_ratio_effects: Vec<SNRatioEffect>,
    /// ANOVA results.
    pub anova: ANOVAResult,
    /// Optimal settings.
    pub optimal_settings: OptimalSettings,
    /// Analysis timestamp (ISO 8601).
    pub analyzed_at: String,
}

/// Request for DOE analysis.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DOEAnalysisRequest {
    /// The OA matrix (runs × factors).
    pub array_data: Vec<Vec<u32>>,
    /// Response data (runs × replicates).
    pub response_data: Vec<Vec<f64>>,
    /// Factor IDs in column order.
    pub factor_ids: Vec<String>,
    /// Factor names in column order.
    pub factor_names: Vec<String>,
    /// Optimization type.
    pub optimization_type: OptimizationType,
    /// Target value for nominal-is-best.
    pub target_value: Option<f64>,
    /// F-ratio threshold for pooling (default: 2.0).
    pub pooling_threshold: Option<f64>,
    /// Whether to enable factor pooling (default: true).
    pub enable_pooling: Option<bool>,
    /// Minimum factors to keep unpooled (default: 1).
    pub min_unpooled_factors: Option<usize>,
    /// Confidence level for intervals (default: 0.95).
    pub confidence_level: Option<f64>,
}
