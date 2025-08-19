use minizinc_macro::minizinc_solve;

// Dummy struct to represent the output of MiniZinc
#[derive(Default, Debug)]
struct MySolution {
    // Add fields as needed for your MiniZinc output
    // For now, just a placeholder
    #[allow(dead_code)]
    value: i32,
}

#[minizinc_solve(
    model = "test_minimal.mzn",
    data = "test_dzn_word_embeddings.mzn",
    output_type = "MySolution"
)]
fn solve_my_problem() -> MySolution {
    // This function body will be replaced by the macro
    println!("This line will be replaced by the macro-generated code.");
    MySolution::default()
}

fn main() {
    println!("Calling solve_my_problem...");
    let solution = solve_my_problem();
    println!("Received solution: {:?}", solution);
}
