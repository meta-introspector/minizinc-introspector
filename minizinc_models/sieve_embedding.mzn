% File: sieve_embedding.mzn
% Purpose: MiniZinc model for Sieve of Eratosthenes and embedding of primes.

% Input parameters from DZN data
int: num_vars;       % Number of lambda variables (corresponds to num_vec in data generator)
int: num_values;     % Number of values each variable can take
int: num_partitions; % Number of partitions in the embedding space

% --- Sieve of Eratosthenes (up to limit 3, to find prime 2) ---
% We'll keep this simple for now, focusing on finding prime 2.
% A more general sieve would iterate up to a larger limit.

set of int: NUMBERS = 1..3; % Numbers to check for primality

array[NUMBERS] of var bool: is_prime;

constraint is_prime[1] = false; % 1 is not prime
constraint is_prime[2] = true;  % 2 is prime
constraint is_prime[3] = true;  % 3 is prime

% --- Embedding Variables (simplified for now) ---
% These variables will represent the embedding of the found primes.
% For simplicity, we'll assume a 1D embedding for now, and expand later.

% Assuming we are embedding the primes found by the sieve.
% For limit 3, primes are {2, 3}.
% We need to map these to embedding variables.

% Let's define embedding variables for each number up to num_vars
% (which is 1 in our initial test case, so we'll embed just one prime for now)
array[1..num_vars] of var int: prime_embedding;

% --- Constraints for Embedding ---
% For the initial test, we'll just constrain the embedding of prime 2.
% This is a placeholder constraint.
constraint if is_prime[2] then prime_embedding[1] >= 0 else true endif;

% --- Output ---
output [
  "is_prime = ", show(is_prime), ";\n",
  "prime_embedding = ", show(prime_embedding), ";\n"
];
