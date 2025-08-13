include "globals.mzn";

% Parameters
int: n;
int: d = 8;
int: SCALE_BITS = 8;
int: SCALE_MAX = 127; % For signed 8-bit, max value is 127
float: B = 1.0; % Original bound, used for conceptual scaling
int: KAPPA_SCALE = 1000000; % Scaling factor for kappa values and other floats
float: kappa_global;
float: kappa_bind;
float: kappa_free;
float: kappa_app;
float: kappa_hier;
float: kappa_alpha;
float: kappa_beta;
float: kappa_vec;
float: epsilon = 1e-6;
float: theta_vec = 0.05;

% Relations
int: num_bindings;
array[1..num_bindings] of int: binder_idx;
array[1..num_bindings] of int: bound_idx;

int: num_free;
array[1..num_free] of int: free_idx;
array[1..num_free] of int: unrelated_binder_idx;

int: num_apps;
array[1..num_apps] of int: func_idx;
array[1..num_apps] of int: arg_idx;

int: num_hier;
array[1..num_hier] of int: parent_idx;
array[1..num_hier] of int: child_idx;

int: num_alpha;
array[1..num_alpha] of int: eq1_idx;
array[1..num_alpha] of int: eq2_idx;

int: num_beta;
array[1..num_beta] of int: redex_idx;
array[1..num_beta] of int: reduced_idx;

int: num_vec;
array[1..num_vec] of float: alpha_coeff;
array[1..num_vec] of float: beta_coeff;
array[1..num_vec] of int: m_idx;
array[1..num_vec] of int: n_idx;
array[1..num_vec] of int: t_idx;

% Variables: positions on unit sphere
array[1..n, 1..d] of var -SCALE_MAX..SCALE_MAX: p;

% Dot product
function var int: dot_product_scaled(int: i, int: j) =
  sum(k in 1..d) (p[i,k] * p[j,k]);

% Unit norm constraints
constraint forall(i in 1..n) (
  sum(k in 1..d) (p[i,k] * p[i,k]) = SCALE_MAX * SCALE_MAX
);

% Objective: minimize energy
var float: E =
  sum(i in 1..n, j in i+1..n) (kappa_global * (dot_product_scaled(i,j) + SCALE_MAX * SCALE_MAX) * (dot_product_scaled(i,j) + SCALE_MAX * SCALE_MAX)) +
  sum(b in 1..num_bindings) (kappa_bind * (SCALE_MAX * SCALE_MAX - dot_product_scaled(binder_idx[b], bound_idx[b])) * (SCALE_MAX * SCALE_MAX - dot_product_scaled(binder_idx[b], bound_idx[b]))) +
  sum(f in 1..num_free) (kappa_free * (dot_product_scaled(free_idx[f], unrelated_binder_idx[f]) + SCALE_MAX * SCALE_MAX) * (dot_product_scaled(free_idx[f], unrelated_binder_idx[f]) + SCALE_MAX * SCALE_MAX)) +
  sum(a in 1..num_apps) (kappa_app * (SCALE_MAX * SCALE_MAX - dot_product_scaled(func_idx[a], arg_idx[a])) * (SCALE_MAX * SCALE_MAX - dot_product_scaled(func_idx[a], arg_idx[a]))) +
  sum(h in 1..num_hier) (kappa_hier * (SCALE_MAX * SCALE_MAX - dot_product_scaled(parent_idx[h], child_idx[h])) * (SCALE_MAX * SCALE_MAX - dot_product_scaled(parent_idx[h], child_idx[h]))) +
  sum(e in 1..num_alpha) (kappa_alpha * (SCALE_MAX * SCALE_MAX - dot_product_scaled(eq1_idx[e], eq2_idx[e])) * (SCALE_MAX * SCALE_MAX - dot_product_scaled(eq1_idx[e], eq2_idx[e]))) +
  sum(r in 1..num_beta) (kappa_beta * (SCALE_MAX * SCALE_MAX - dot_product_scaled(redex_idx[r], reduced_idx[r])) * (SCALE_MAX * SCALE_MAX - dot_product_scaled(redex_idx[r], reduced_idx[r]))) +
  sum(v in 1..num_vec) (
    let {
      array[1..d] of var float: expected = [ alpha_coeff[v] * p[m_idx[v],k] + beta_coeff[v] * p[n_idx[v],k] | k in 1..d ];
      var float: violation_sq = sum(k in 1..d) (pow(expected[k] - p[t_idx[v],k], 2.0))
    } in kappa_vec * max(0.0, violation_sq - theta_vec * theta_vec)
  );

solve minimize E;

output [ "Positions on unit sphere in R^8:\n" ++ show2d(p) ++ "\n" ];
