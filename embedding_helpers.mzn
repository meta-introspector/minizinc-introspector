% Helper Functions for Lambda Calculus Embedding

% Get the actual value from the partition_values array
function var int: get_partition_value(var int: idx) = scaled_partition_values[idx];

% Dot product (using float values from partitions)
function var int: dot_product_scaled(int: i, int: j) =
  sum(k in 1..d) (p_actual_values[i,k] * p_actual_values[j,k]);