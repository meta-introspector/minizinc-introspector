% Constraints for Prime Embeddings

% Map the value of each prime to the first dimension of its embedding.
constraint forall(k in 1..min(primes_found_length, num_vec)) (
  p_actual_values[k, 1] = primes_list_raw[k]
);