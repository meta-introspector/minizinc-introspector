% File: sieve_embedding.mzn
% Purpose: MiniZinc model for Sieve of Eratosthenes and embedding of primes.

% Input parameters from DZN data
int: num_vars;       % Number of lambda variables (corresponds to num_vec in data generator)
int: num_values;     % Number of values each variable can take
int: num_partitions; % Number of partitions in the embedding space

% --- Sieve of Eratosthenes ---
% Finds prime numbers up to 'num_values'.

set of int: NUMBERS = 1..num_values; % Numbers to check for primality

array[NUMBERS] of var bool: is_prime;

constraint is_prime[1] = false; % 1 is not prime

constraint forall(i in 2..num_values) (
  is_prime[i] <-> forall(j in 2..i-1) (
    (i mod j != 0)
  )
);

% --- Embedding Variables ---
% We will embed the primes found by the sieve.
% The embedding space has 'num_partitions' dimensions.

set of int: PRIMES_FOUND = {i | i in NUMBERS where is_prime[i]};
int: num_primes_found = card(PRIMES_FOUND);

% Embedding coordinates for each prime
array[1..num_primes_found, 1..num_partitions] of var int: prime_embedding_coords;

% Map prime values to their index in the embedding array
array[PRIMES_FOUND] of int: prime_to_idx;
constraint forall(i in 1..num_primes_found) (
  prime_to_idx[sorted(PRIMES_FOUND)[i]] = i
);

% --- Embedding Constraints ---
% For now, a simple constraint: all embedding coordinates are non-negative.
% More complex embedding constraints will be added later.
constraint forall(i in 1..num_primes_found, j in 1..num_partitions) (
  prime_embedding_coords[i, j] >= 0
);

% --- Output ---
output [
  "is_prime = ", show(is_prime), ";\n",
  "primes_found = ", show(PRIMES_FOUND), ";\n",
  "prime_embedding_coords = ", show(prime_embedding_coords), ";\n"
];

