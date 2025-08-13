% Sieve Results

% Collect the primes into a list using a fixed-size array
array[1..MAX_SIEVE_NUM] of var int: primes_list_raw;

constraint forall(i in 1..MAX_SIEVE_NUM) (
  if i >= 2 /\ is_prime[i] then
    primes_list_raw[i] = i
  else
    primes_list_raw[i] = 0 % Use 0 as a sentinel for non-primes
  endif
);

% We still need primes_found_length for the output show.
var int: primes_found_length = sum(i in 2..MAX_SIEVE_NUM) (church_if_int(is_prime[i], 1, 0));
