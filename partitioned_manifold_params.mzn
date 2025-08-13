% Parameters for Partitioned Manifold
int: P = 5; % Number of partitions (e.g., for prime 5)

% Mapping from integer index to value
% For P=3, we can map 1 -> -1.0, 2 -> 0.0, 3 -> 1.0
array[1..P] of float: partition_values = [-1.0 + (2.0 * (i-1) / (P-1)) | i in 1..P];
