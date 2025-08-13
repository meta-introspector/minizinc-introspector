% Parameters for Lambda Calculus Embedding on Partitioned Sphere

include "partitioned_manifold_params.mzn";

int: n;                % Number of subterms
int: d = 8;            % Embedding dimension

% Kappa values (scaled to integers)
int: KAPPA_SCALE = 100; % Scaling factor for kappa values and other floats
float: kappa_global;
float: kappa_bind;
float: kappa_free;
float: kappa_app;
float: kappa_hier;
float: kappa_alpha;
float: kappa_beta;
float: kappa_vec;

float: epsilon; % Avoid div by zero
float: theta_vec;

int: kappa_global_scaled = round(kappa_global * KAPPA_SCALE);
int: kappa_bind_scaled = round(kappa_bind * KAPPA_SCALE);
int: kappa_free_scaled = round(kappa_free * KAPPA_SCALE);
int: kappa_app_scaled = round(kappa_app * KAPPA_SCALE);
int: kappa_hier_scaled = round(kappa_hier * KAPPA_SCALE);
int: kappa_alpha_scaled = round(kappa_alpha * KAPPA_SCALE);
int: kappa_beta_scaled = round(kappa_beta * KAPPA_SCALE);
int: kappa_vec_scaled = round(kappa_vec * KAPPA_SCALE);

int: epsilon_scaled = round(epsilon * KAPPA_SCALE);
int: theta_vec_scaled = round(theta_vec * KAPPA_SCALE);