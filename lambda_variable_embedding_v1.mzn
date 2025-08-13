% MiniZinc model for Lambda Calculus Embedding (Variable - Version 1)

% Include core parameters to get n and d
include "embedding_params_core_composed_v3.mzn";

% Define types for lambda terms
enum LambdaTermType = {VAR, ABS, APP};

% Map enum values to integers for array representation
int: VAR_TYPE = 0;
int: ABS_TYPE = 1;
int: APP_TYPE = 2;

% Define a maximum depth for lambda terms to keep the problem finite
int: MAX_DEPTH = 3;

% Define the number of variables (for 'two single-bit data types as bindings')
int: NUM_VARS = 2; % e.g., x, y

% Representation of a lambda term as an array of integers
% Each element in the array represents a node in the abstract syntax tree (AST).
% Format: [NodeType, ...arguments]
% VAR: [VAR_TYPE, var_index]
% ABS: [ABS_TYPE, bound_var_index, body_start_index]
% APP: [APP_TYPE, func_start_index, arg_start_index]

% Placeholder for the AST of a lambda term
array[0..1] of var int: lambda_ast_nodes; % Size adjusted for a single variable

% Define the structure for a variable x
constraint lambda_ast_nodes[0] = VAR_TYPE; % Node 0 is a variable
constraint lambda_ast_nodes[1] = 0;        % Variable is x (index 0)

% Hyperspace location for each lambda term
% n will be the number of lambda terms we are embedding
% d is the dimensionality of the hyperspace
array[1..n, 1..d] of var int: p_actual_values;

% Assign a specific hyperspace location for variable x (n=1)
% This is an arbitrary initial placement for demonstration
constraint p_actual_values[1,1] = 10;
constraint p_actual_values[1,2] = 20;
constraint p_actual_values[1,3] = 30;
constraint p_actual_values[1,4] = 40;
constraint p_actual_values[1,5] = 50;
constraint p_actual_values[1,6] = 60;
constraint p_actual_values[1,7] = 70;
constraint p_actual_values[1,8] = 80;


% Solve statement (for now, just satisfy)
solve satisfy;

% Output the lambda term representation and its hyperspace location (for debugging)
output ["Lambda AST: " ++ show(lambda_ast_nodes) ++ "\n" ++        "Hyperspace Location: " ++ show(p_actual_values)];

