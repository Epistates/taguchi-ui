//! Taguchi UI - Desktop application for Design of Experiments.
//!
//! This crate provides the Tauri backend for the Taguchi UI application,
//! wrapping the taguchi library with commands for the frontend.

mod commands;
mod types;

use commands::{
    // Builder commands
    build_oa,
    get_available_constructions,
    validate_build_params,
    // Catalogue commands
    get_standard_array,
    list_standard_arrays,
    search_catalogue,
    // Analysis commands
    compute_array_strength,
    get_balance_report,
    get_correlation_matrix,
    verify_array,
    // DOE Analysis commands
    run_doe_analysis,
    // Export/Import commands
    export_csv,
    export_json,
    export_latex,
    import_csv,
    import_json,
    validate_import,
};

/// Run the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_svelte::init())
        .invoke_handler(tauri::generate_handler![
            // Builder commands
            build_oa,
            get_available_constructions,
            validate_build_params,
            // Catalogue commands
            list_standard_arrays,
            get_standard_array,
            search_catalogue,
            // Analysis commands
            verify_array,
            compute_array_strength,
            get_balance_report,
            get_correlation_matrix,
            // DOE Analysis commands
            run_doe_analysis,
            // Export/Import commands
            export_csv,
            export_json,
            export_latex,
            import_csv,
            import_json,
            validate_import,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
