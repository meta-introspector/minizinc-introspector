% File: generate_vector_params_v2.mzn
% Purpose: MiniZinc model to generate parameterized vector parameters for testing.
% Takes num_vec (number of elements) and base_size (max value for indices/coeffs) as input.

int: num_vec;
int: base_size;

array[1..num_vec] of float: alpha_coeff = [int2float(i mod base_size) / int2float(base_size) | i in 1..num_vec];
array[1..num_vec] of float: beta_coeff = [int2float(i mod base_size) / int2float(base_size) | i in 1..num_vec];
array[1..num_vec] of int: m_idx = [i mod base_size + 1 | i in 1..num_vec]; % +1 to avoid 0-indexing if not desired
array[1..num_vec] of int: n_idx = [i mod base_size + 1 | i in 1..num_vec];
array[1..num_vec] of int: t_idx = [i mod base_size + 1 | i in 1..num_vec];

output [
  "alpha_coeff = [", join(", ", [show(x) | x in alpha_coeff]), "];\n",
  "beta_coeff = [", join(", ", [show(x) | x in beta_coeff]), "];\n",
  "m_idx = [", join(", ", [show(x) | x in m_idx]), "];\n",
  "n_idx = [", join(", ", [show(x) | x in n_idx]), "];\n",
  "t_idx = [", join(", ", [show(x) | x in t_idx]), "];\n"
];