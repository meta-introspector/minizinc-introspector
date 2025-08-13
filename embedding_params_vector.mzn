% Vector Axiom Parameters for Lambda Calculus Embedding

int: num_vec;
array[1..num_vec] of float: alpha_coeff;
array[1..num_vec] of float: beta_coeff;
array[1..num_vec] of int: m_idx;
array[1..num_vec] of int: n_idx;
array[1..num_vec] of int: t_idx;

% Derived scaled integer parameters
array[1..num_vec] of int: alpha_coeff_scaled = [round(alpha_coeff[i] * PARTITION_SCALE) | i in 1..num_vec];
array[1..num_vec] of int: beta_coeff_scaled = [round(beta_coeff[i] * PARTITION_SCALE) | i in 1..num_vec];
