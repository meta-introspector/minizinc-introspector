include "all_different.mzn";

% Constraints for Lambda Calculus Embedding

% Unit norm constraints (sum of squares)
% We aim for a consistent magnitude for each embedding vector.
% This constraint encourages the solver to find combinations of partition_values
% that result in a sum of squares within a reasonable range, promoting diversity
% in the embeddings.
constraint forall(i in 1..num_vec) (
  let { var int: current_sum_sq = sum(k in 1..d) (p_actual_values[i,k] * p_actual_values[i,k]) } in
  current_sum_sq >= 0 /\ current_sum_sq <= (PARTITION_SCALE * PARTITION_SCALE * d)
);

% Constraint to encourage diversity in embedding dimensions (for k > 1)
% Forces p_idx values to be distinct for each vector across dimensions 2 to d.
constraint forall(i in 1..num_vec) (
  alldifferent([p_idx[i,k] | k in 2..d])
);