include "partitioned_manifold_params.mzn";

% Parameters
int: n = 2; % Two points
int: d = 2; % Two dimensions

% Variables: position as an index into the partition_values array
array[1..n, 1..d] of var 1..P: p_idx; % Index into partition_values array

% Get the actual value from the partition_values array
function var float: get_partition_value(var int: idx) = int2float(scaled_partition_values[idx]);

% Constraint: points must be different
constraint p_idx[1,1] != p_idx[2,1];

% Objective: Minimize a constant to just find a feasible solution
solve minimize 0;

output [ "Position 1 (index): ", show(p_idx[1,1]), "\n",
         "Position 1 (value): ", show(get_partition_value(p_idx[1,1])), "\n",
         "Position 2 (index): ", show(p_idx[2,1]), "\n",
         "Position 2 (value): ", show(get_partition_value(p_idx[2,1])), "\n" ];


