% Objective for Lambda Calculus Embedding (Version 3 - Simplified)

var int: E =
  sum(i in 1..n, j in i+1..n) (kappa_global_scaled * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE) * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE));

solve minimize E;
