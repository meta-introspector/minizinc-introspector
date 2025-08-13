% Other Parameters for Lambda Calculus Embedding

float: epsilon; % Avoid div by zero
float: theta_vec;

% Derived scaled integer parameters
int: epsilon_scaled = round(epsilon * KAPPA_SCALE);
int: theta_vec_scaled = round(theta_vec * KAPPA_SCALE);
