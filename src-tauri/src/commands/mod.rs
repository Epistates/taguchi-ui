//! Tauri command handlers.
//!
//! This module organizes commands by domain:
//! - `builder`: OA construction commands
//! - `catalogue`: Standard array access
//! - `analysis`: Verification and statistics
//! - `export`: Import/export functionality
//! - `doe_analysis`: DOE statistical analysis

pub mod analysis;
pub mod builder;
pub mod catalogue;
pub mod doe_analysis;
pub mod export;

pub use analysis::*;
pub use builder::*;
pub use catalogue::*;
pub use doe_analysis::*;
pub use export::*;
