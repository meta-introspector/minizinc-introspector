use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let results_file = "/data/data/com.termux/files/home/storage/github/libminizinc/v7_debug_test_results.txt";
    let report_file = "/data/data/com.termux/files/home/storage/github/libminizinc/docs/sieve_performance_report.md";

    let file = fs::File::open(results_file)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("---") || line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 7 {
            let duration: f64 = parts[0].parse().unwrap_or(0.0);
            let num_vars: usize = parts[4].parse().unwrap_or(0);
            let num_values: usize = parts[5].parse().unwrap_or(0);
            let num_partitions: usize = parts[6].parse().unwrap_or(0);
            data.push((duration, num_vars, num_values, num_partitions));
        }
    }

    // Sort data by duration for easier analysis
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut report_content = String::new();
    report_content.push_str("# MiniZinc Sieve Model Performance Report\n\n");
    report_content.push_str("## 1. Introduction\n\nThis report summarizes the performance of the MiniZinc sieve model (`sieve_embedding.mzn`) across a defined parameter lattice. The goal is to understand the relationship between model parameters (`num_vars`, `num_values`, `num_partitions`) and execution time.\n\n");
    report_content.push_str("## 2. Test Setup\n\nTests were conducted using the `minizinc_test_runner_rs` Rust program, which dynamically generates DZN data files for each test case and executes the MiniZinc solver. The following parameter ranges were explored:\n\n*   `num_vars`: 1, 2, 3\n*   `num_values`: 2, 3\n*   `num_partitions`: 2, 3\n\n## 3. Performance Results\n\nThe following table presents the execution time for each parameter combination, sorted from fastest to slowest:\n\n| Duration (s) | num_vars | num_values | num_partitions |\n|--------------|----------|------------|----------------|\n");

    for (duration, num_vars, num_values, num_partitions) in &data {
        report_content.push_str(&format!("| {:.9} | {} | {} | {} |\n", duration, num_vars, num_values, num_partitions));
    }

    report_content.push_str("\n## 4. Analysis and Observations\n\n*(Analysis will go here. This section will be populated with insights after reviewing the data, potentially including plots and regression analysis.)*\n\n");

    report_content.push_str("## 5. Runtime Prediction (Placeholder)\n\n*(Runtime prediction model and results will go here.)*\n\n");

    fs::write(report_file, report_content)?;

    println!("Report generated: {}", report_file);

    Ok(())
}