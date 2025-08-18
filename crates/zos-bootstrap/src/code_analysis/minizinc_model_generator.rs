pub fn generate_ast_minizinc_model_string() -> String {
    r###"array[int] of int: ast_elements_numerical;
int: num_elements = length(ast_elements_numerical);
int: target_index;

% Define the prime for "modularity"
int: modularity_prime = 3;

% Original target value
int: original_target_value = ast_elements_numerical[target_index];

% Decision variable for the suggested numerical vector of the target element
var int: suggested_numerical_vector;

% Constraint: suggested_numerical_vector mod modularity_prime = 0;

% Objective: Minimize the absolute difference between the original target value
% and the suggested numerical vector.
solve minimize abs(original_target_value - suggested_numerical_vector);

output [
    "suggested_numerical_vector = ", show(suggested_numerical_vector), "\n"
];
"###.to_string()
}