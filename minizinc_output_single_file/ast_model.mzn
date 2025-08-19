array[int] of int: ast_elements_numerical;
int: num_elements = length(ast_elements_numerical);
int: target_index;

int: modularity_prime = 3;

int: original_target_value = 11;

var 0..3: suggested_numerical_vector;

constraint suggested_numerical_vector mod modularity_prime = 0;

solve minimize abs(original_target_value - suggested_numerical_vector);

output [
    "suggested_numerical_vector = ", show(suggested_numerical_vector), "\n"];
