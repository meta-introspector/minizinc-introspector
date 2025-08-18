use crate::utils::error::Result;

#[derive(Debug)]
pub struct MiniZincAnalysisResults {
    pub suggested_numerical_vector: i32,
}

pub fn parse_minizinc_output(output_str: &str) -> Result<MiniZincAnalysisResults> {
    let mut suggested_numerical_vector = 0;

    for line in output_str.lines() {
        if line.starts_with("suggested_numerical_vector =") {
            suggested_numerical_vector = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        }
    }

    Ok(MiniZincAnalysisResults {
        suggested_numerical_vector,
    })
}
