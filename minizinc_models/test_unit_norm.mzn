include "partitioned_manifold_params.mzn";

int: n = 1; % One point
int: d = 8; % Eight dimensions

% Variables: positions as indices into the partition_values array
array[1..n, 1..d] of var 1..P: p_idx;

array[1..n, 1..d] of var int: p_actual_values;
constraint forall(i in 1..n, k in 1..d) (
  p_actual_values[i,k] = get_partition_value(p_idx[i,k])
);

% Get the actual value from the partition_values array
function var int: get_partition_value(var int: idx) = scaled_partition_values[idx];

% Unit norm constraint
constraint forall(i in 1..n) (
  sum(k in 1..d) (p_actual_values[i,k] * p_actual_values[i,k]) = PARTITION_SCALE * PARTITION_SCALE
);

solve satisfy;

output [ "Positions (indices): ", show(p_idx[1,..]), "\n",
         "Positions (actual values): ", show(p_actual_values[1,..]), "\n" ];

