% Objective for Lambda Calculus Embedding (Version 3 - Simplified)

var int: E =
  sum(i in 1..2, j in i+1..2) (kappa_global_scaled * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE) * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE));

solve minimize E;