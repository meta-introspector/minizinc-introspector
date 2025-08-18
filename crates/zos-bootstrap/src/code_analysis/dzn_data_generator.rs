use crate::code_analysis::ast_to_numerical_vector_converter::AstNumericalVector;

pub fn generate_ast_dzn_data_string(
    all_ast_numerical_vectors: &[AstNumericalVector],
    target_index: usize,
) -> String {
    let mut dzn_content = String::new();
    dzn_content.push_str("ast_elements_numerical = [\n");
    for (i, vec) in all_ast_numerical_vectors.iter().enumerate() {
        dzn_content.push_str(&vec.numerical_vector.to_string());
        if i < all_ast_numerical_vectors.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    // FIX: Corrected the format string for dzn_content
    dzn_content.push_str(&format!("];\ntarget_index = {};", target_index));
    dzn_content
}
