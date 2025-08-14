% File: generate_vector_params.mzn
% Purpose: MiniZinc model to generate dummy vector parameters for testing.
% Takes num_vec as input and outputs arrays of the specified size.

int: num_vec;

array[1..num_vec] of float: alpha_coeff = [0.0 | i in 1..num_vec];
array[1..num_vec] of float: beta_coeff = [0.0 | i in 1..num_vec];
array[1..num_vec] of int: m_idx = [0 | i in 1..num_vec];
array[1..num_vec] of int: n_idx = [0 | i in 1..num_vec];
array[1..num_vec] of int: t_idx = [0 | i in 1..num_vec];

output [
  "alpha_coeff = ", show(alpha_coeff), ";\n",
  "beta_coeff = ", show(beta_coeff), ";\n",
  "m_idx = ", show(m_idx), ";\n",
  "n_idx = ", show(n_idx), ";\n",
  "t_idx = ", show(t_idx), ";\n"
];
