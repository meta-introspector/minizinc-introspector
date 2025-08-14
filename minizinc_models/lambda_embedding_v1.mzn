% MiniZinc model for Lambda Calculus Embedding (Version 1)

% Include core parameters to get n and d
include "embedding_params_core_composed_v3.mzn";

% Define types for lambda terms
enum NodeType = {VAR, ABS, APP};

% Map enum values to integers for array representation (if needed for constraints)
% int: VAR_TYPE = 0;
% int: ABS_TYPE = 1;
% int: APP_TYPE = 2;

% Define a maximum depth for lambda terms to keep the problem finite
int: MAX_DEPTH = 3;

% Define the number of variables (for 'two single-bit data types as bindings')
int: NUM_VARS = 2; % e.g., x, y

% Number of nodes in the AST
int: num_nodes;

% AST representation using parallel arrays
array[0..num_nodes-1] of var NodeType: node_type;
array[0..num_nodes-1] of var int: node_val;      % For VAR: variable index; For ABS: bound var index; For APP: unused
array[0..num_nodes-1] of var int: node_child1;   % For ABS: body index; For APP: function index
array[0..num_nodes-1] of var int: node_child2;   % For APP: argument index

% Constraints to define the AST for lambda x. x
% Assuming num_nodes = 2 (one for ABS, one for VAR)
constraint num_nodes = 2;

% Node 0: Abstraction (lambda x. ...)
constraint node_type[0] = ABS;
constraint node_val[0] = 0;        % Bound variable is x (index 0)
constraint node_child1[0] = 1;     % Body is Node 1
constraint node_child2[0] = -1;    % Not applicable for ABS

% Node 1: Variable (x)
constraint node_type[1] = VAR;
constraint node_val[1] = 0;        % Variable is x (index 0)
constraint node_child1[1] = -1;    % Not applicable for VAR
constraint node_child2[1] = -1;    % Not applicable for VAR

% Hyperspace location for each lambda term
% n will be the number of lambda terms we are embedding
% d is the dimensionality of the hyperspace
array[1..n, 1..d] of var int: p_actual_values;

% --- REMOVED ARBITRARY ASSIGNMENT ---
% The solver will now determine p_actual_values based on new constraints.

% --- NEW CONSTRAINTS TO DETERMINE p_actual_values ---
% For now, let's make a simple mapping:
% The hyperspace location is directly related to the AST node types and values.
% This is a very basic embedding, to be refined later.
constraint forall(j in 1..d) (
    p_actual_values[1,j] = (int2(node_type[0]) * 100) + (node_val[0] * 10) + j
);
% This constraint is a placeholder. It needs to be more sophisticated.
% int2(NodeType) converts the enum to an integer.

% Solve statement (for now, just satisfy)
solve satisfy;

% Output the lambda term representation and its hyperspace location (for debugging)
output ["Node Types: " ++ show(node_type) ++ "\n" ++ 
        "Node Values: " ++ show(node_val) ++ "\n" ++ 
        "Node Child1: " ++ show(node_child1) ++ "\n" ++ 
        "Node Child2: " ++ show(node_child2) ++ "\n" ++ 
        "Hyperspace Location: " ++ show(p_actual_values)];