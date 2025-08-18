% MiniZinc Model for ZOS Project Hierarchical Decomposition

% Define the total number of components at each level
int: num_level1_components = 2;
int: num_level2_components = 6;

% Define component types (conceptual)
enum ComponentType = {CoreComputationalEngine, IntelligentApplicationLayer,
                        MiniZincCore, FFI, BaseModelsAndData,
                        ZosBootstrap, KnowledgeManagement, LLMIntegration};

% Array to hold all components, indexed by a flat ID
int: total_components = num_level1_components + num_level2_components; % For now, just two levels
array[1..total_components] of var ComponentType: components;

% Define parent-child relationships
% parent_of[child_id] = parent_id
array[1..total_components] of var opt int: parent_of;

% --- Level 1: Split by 2 ---
constraint components[1] = CoreComputationalEngine;
constraint components[2] = IntelligentApplicationLayer;

constraint parent_of[1] = <>;
constraint parent_of[2] = <>;

% --- Level 2: Split each of the 2 by 3 (making 6 total) ---
% CoreComputationalEngine (ID 1) splits into 3
constraint components[3] = MiniZincCore;
constraint parent_of[3] = 1;
constraint components[4] = FFI;
constraint parent_of[4] = 1;
constraint components[5] = BaseModelsAndData;
constraint parent_of[5] = 1;

% IntelligentApplicationLayer (ID 2) splits into 3
constraint components[6] = ZosBootstrap;
constraint parent_of[6] = 2;
constraint components[7] = KnowledgeManagement;
constraint parent_of[7] = 2;
constraint components[8] = LLMIntegration;
constraint parent_of[8] = 2;

% Constraints to ensure correct splitting (e.g., each parent has exactly 3 children at this level)
constraint forall(p in 1..num_level1_components) (
    count([c | c in 1..total_components where parent_of[c] = p]) = 3
);

% Solve for the component types and parent relationships
solve satisfy;

output [    "ZOS Project Decomposition:\n",    "Level 1 Components:\n",    "  1: " ++ show(components[1]) ++ "\n",    "  2: " ++ show(components[2]) ++ "\n",    "\nLevel 2 Components:\n"] ++ 
[
    "  " ++ show(i) ++ ": " ++ show(components[i]) ++ " (Parent: " ++ show(parent_of[i]) ++ ")\n"
    | i in 3..total_components
];
