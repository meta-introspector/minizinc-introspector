% Variables for Lambda Calculus Embedding

% Variables: positions as indices into the partition_values array
array[1..n, 1..d] of var 1..P: p_idx;

array[1..n, 1..d] of var int: p_actual_values;
constraint forall(i in 1..n, k in 1..d) (
  p_actual_values[i,k] = get_partition_value(p_idx[i,k])
);