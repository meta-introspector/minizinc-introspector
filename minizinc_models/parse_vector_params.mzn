% File: parse_vector_params.mzn
% Purpose: MiniZinc model to parse and validate vector parameters DZN files.

int: num_vec;

array[1..num_vec] of float: alpha_coeff;
array[1..num_vec] of float: beta_coeff;
array[1..num_vec] of int: m_idx;
array[1..num_vec] of int: n_idx;
array[1..num_vec] of int: t_idx;

solve satisfy;