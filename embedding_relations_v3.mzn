% Relations for Lambda Calculus Embedding (Version 3 - Simplified)

% int: num_bindings;
% array[1..num_bindings] of int: binder_idx;
% array[1..num_bindings] of int: bound_idx;

% int: num_free;
% array[1..num_free] of int: free_idx;
% array[1..num_free] of int: unrelated_binder_idx;

% int: num_apps;
% array[1..num_apps] of int: func_idx;
% array[1..num_apps] of int: arg_idx;

% int: num_hier;
% array[1..num_hier] of int: parent_idx;
% array[1..num_hier] of int: child_idx;

% int: num_alpha;
% array[1..num_alpha] of int: eq1_idx;
% array[1..num_alpha] of int: eq2_idx;

% int: num_beta;
% array[1..num_beta] of int: redex_idx;
% array[1..num_beta] of int: reduced_idx;

% int: num_vec;

% array[1..num_vec] of int: m_idx;
% array[1..num_vec] of int: n_idx;
% array[1..num_vec] of int: t_idx;