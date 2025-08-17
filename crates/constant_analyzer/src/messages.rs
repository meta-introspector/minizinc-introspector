#![allow(unused_macros)]

#[macro_export]
macro_rules! message {
    (Phase1, $project_root:expr) => {
        println!("Phase 1: Ingesting and parsing Rust files from {:?}", $project_root)
    };
    (ProcessingFile, $path:expr) => {
        println!("  Processing file: {:?}", $path)
    };
    (WritingReport, $output_file_path:expr) => {
        println!("Writing report to: {:?}", $output_file_path)
    };
    (AnalysisComplete, $output_file_path:expr) => {
        println!("\nAnalysis complete. Report saved to {:?}", $output_file_path)
    };
    (ConstantDetail, $name:expr, $declaration_path:expr, $usage_count:expr) => {
        format!(
            "Constant: '{}'\n  Declared in: {:?}\n  Usage Count: {}",
            $name, $declaration_path, $usage_count
        )
    };
    (StatusFlagged) => {
        "  STATUS: FLAGGED (Used more than once)"
    };
    (StatusOK) => {
        "  STATUS: OK (Used once or not at all)"
    };
    (PlanMoveConstant) => {
        "  PLAN: Consider moving this constant to a dedicated constants module (e.g., `constants.rs`, `config.rs`, or `types.rs`) to centralize its definition."
    };
    (NoteSingleUseNotDedicated) => {
        "  NOTE: This constant is not in a dedicated constants file, but is only used once. Consider if it should be inlined or moved for future clarity."
    };
    (NoteAlreadyDedicated) => {
        "  NOTE: This constant is already in a dedicated constants file/directory."
    };
    (ReportHeader) => {
        "--- Constant Analysis Report ---"
    };
    (ReportSeparator) => {
        "--------------------------------------------------"
    };
    (Phase2) => {
        "\nPhase 2: Analyzing constant usages..."
    };
    (Phase3) => {
        "\nPhase 3: Generating Constant Analysis Report and Refactoring Plan..."
    };
}
