% Objective for Lambda Calculus Embedding (Version 4 - Maximize Diversity)

% Maximize the sum of squared Euclidean distances between all pairs of embedding vectors.
% This encourages the solver to spread out the embeddings as much as possible.

var int: DiversityObjective = 
  sum(i in 1..num_vec, j in i+1..num_vec) (
    sum(k in 1..d) (
      (p_actual_values[i,k] - p_actual_values[j,k]) * (p_actual_values[i,k] - p_actual_values[j,k])
    )
  );

solve maximize DiversityObjective;
