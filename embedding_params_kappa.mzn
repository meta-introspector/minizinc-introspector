% Kappa Parameters for Lambda Calculus Embedding

float: kappa_global;
float: kappa_bind;
float: kappa_free;
float: kappa_app;
float: kappa_hier;
float: kappa_alpha;
float: kappa_beta;
float: kappa_vec;

% Derived scaled integer parameters
int: kappa_global_scaled = round(kappa_global * KAPPA_SCALE);
int: kappa_bind_scaled = round(kappa_bind * KAPPA_SCALE);
int: kappa_free_scaled = round(kappa_free * KAPPA_SCALE);
int: kappa_app_scaled = round(kappa_app * KAPPA_SCALE);
int: kappa_hier_scaled = round(kappa_hier * KAPPA_SCALE);
int: kappa_alpha_scaled = round(kappa_alpha * KAPPA_SCALE);
int: kappa_beta_scaled = round(kappa_beta * KAPPA_SCALE);
int: kappa_vec_scaled = round(kappa_vec * KAPPA_SCALE);

float: kappa_unit_norm; % New parameter for unit norm penalty
int: kappa_unit_norm_scaled = round(kappa_unit_norm * KAPPA_SCALE);
