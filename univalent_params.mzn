% Parameters for Univalent Neural Network Space
int: RECIPROCAL_SCALE = 1000000;
array[1..8] of float: primes = [2, 3, 5, 7, 11, 13, 17, 19];
array[1..8] of int: scaled_reciprocals = [round(RECIPROCAL_SCALE / p) | p in primes];
