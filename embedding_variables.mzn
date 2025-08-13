% Variables for Lambda Calculus Embedding

% Variables: positions as indices into the partition_values array
array[1..MAX_SIEVE_NUM, 1..d] of var 1..P: p_idx;

array[1..MAX_SIEVE_NUM, 1..d] of var int: p_actual_values;
constraint forall(i in 1..MAX_SIEVE_NUM, k in 1..d) (
  if k == 1 then
    % No constraint here, as it will be set by embedding_prime_constraints.mzn
    true
  else
    p_actual_values[i,k] = get_partition_value(p_idx[i,k])
  endif
);