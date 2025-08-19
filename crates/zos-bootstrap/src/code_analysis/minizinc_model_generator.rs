use crate::code_analysis::ast_to_numerical_vector_converter::AstNumericalVector;
use minizinc_ffi::environment::MiniZincEnvironment;

pub fn generate_ast_minizinc_model_string(
    _env: &mut MiniZincEnvironment,
    ast_numerical_vectors: &Vec<AstNumericalVector>,
    _target_index: usize,
    complexity_index: u8,
) -> Result<String, String> {
    let mut model_content = String::new();

    // 1. Declarations
    model_content.push_str("array[int] of int: ast_elements_numerical;\n");
    model_content.push_str("int: num_elements = length(ast_elements_numerical);\n\n");
    model_content.push_str("int: target_index;\n\n");
    model_content.push_str("int: modularity_prime = 3;\n\n"); // Define the prime for "modularity"

    // 2. Decision variables for the suggested numerical vector of each AST element
    let max_val = 2u32.pow(complexity_index as u32) - 1;
    model_content.push_str(&format!("array[1..num_elements] of var 0..{}: suggested_numerical_vectors;\n\n", max_val));

    // 3. Constraints for each suggested numerical vector
    for i in 0..ast_numerical_vectors.len() {
        // Constraint: suggested_numerical_vectors[i] mod modularity_prime = 0;
        model_content.push_str(&format!("constraint suggested_numerical_vectors[{}] mod modularity_prime = 0;\n", i + 1));

        // Constraint: abs(ast_elements_numerical[i] - suggested_numerical_vectors[i]) <= max_val;
        model_content.push_str(&format!("constraint abs(ast_elements_numerical[{}] - suggested_numerical_vectors[{}]) <= {};\n", i + 1, i + 1, max_val));
    }
    model_content.push_str("\n");

    // 4. Objective: Minimize the sum of absolute differences between original and suggested numerical vectors
    model_content.push_str("solve minimize sum(i in 1..num_elements) (abs(ast_elements_numerical[i] - suggested_numerical_vectors[i]));\n\n");

    // 5. Output item
    model_content.push_str("output [\n");
    model_content.push_str("    \"suggested_numerical_vectors = \", show(suggested_numerical_vectors), \"\\n\"");
    model_content.push_str("];\n");

    Ok(model_content)
}