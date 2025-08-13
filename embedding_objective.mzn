% Objective for Lambda Calculus Embedding

var int: E =
  sum(i in 1..n, j in i+1..n) (kappa_global_scaled * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE) * (dot_product_scaled(i,j) + PARTITION_SCALE * PARTITION_SCALE)) +
  sum(b in 1..num_bindings) (kappa_bind_scaled * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(binder_idx[b], bound_idx[b])) * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(binder_idx[b], bound_idx[b]))) +
  sum(f in 1..num_free) (kappa_free_scaled * (dot_product_scaled(free_idx[f], unrelated_binder_idx[f]) + PARTITION_SCALE * PARTITION_SCALE) * (dot_product_scaled(free_idx[f], unrelated_binder_idx[f]) + PARTITION_SCALE * PARTITION_SCALE)) +
  sum(a in 1..num_apps) (kappa_app_scaled * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(func_idx[a], arg_idx[a])) * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(func_idx[a], arg_idx[a]))) +
  sum(h in 1..num_hier) (kappa_hier_scaled * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(parent_idx[h], child_idx[h])) * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(parent_idx[h], child_idx[h]))) +
  sum(e in 1..num_alpha) (kappa_alpha_scaled * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(eq1_idx[e], eq2_idx[e])) * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(eq1_idx[e], eq2_idx[e]))) +
  sum(r in 1..num_beta) (kappa_beta_scaled * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(redex_idx[r], reduced_idx[r])) * (PARTITION_SCALE * PARTITION_SCALE - dot_product_scaled(redex_idx[r], reduced_idx[r]))) +
  sum(v in 1..num_vec) (
    let {
      array[1..d] of var int: expected_scaled = [ (alpha_coeff_scaled[v] * p_actual_values[m_idx[v],k] + beta_coeff_scaled[v] * p_actual_values[n_idx[v],k]) div PARTITION_SCALE | k in 1..d ];
      var int: violation_sq_scaled = sum(k in 1..d) ( (expected_scaled[k] - p_actual_values[t_idx[v],k]) * (expected_scaled[k] - p_actual_values[t_idx[v],k]) );
      int: theta_vec_sq_scaled = theta_vec_scaled * theta_vec_scaled;
    } in kappa_vec_scaled * max(0, violation_sq_scaled - theta_vec_sq_scaled)
  );

solve minimize E;