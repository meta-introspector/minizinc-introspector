% Parameters for Partitioned Manifold
int: P = 5; % Number of partitions (e.g., for prime 5)
int: PARTITION_SCALE = 1000; % Scaling factor for partition values

% Mapping from integer index to scaled integer value
array[1..P] of int: scaled_partition_values = [round((-1.0 + (2.0 * (i-1) / (P-1))) * PARTITION_SCALE) | i in 1..P];
