% Relation Parameters for Lambda Calculus Embedding (Version 4 - without num_vec) 🔗✨ -- The Interwoven Fabric of Logic, Refined! -- 🕸️💡

% int: num_bindings; % 🤝🔢 -- The Count of Binding Relationships, Forging Connections! -- 🔗✨
% array[1..num_bindings] of int: binder_idx; % 🔗🔢 -- Indices of the Binders, Guiding the Bound! -- 🧭✨
% array[1..num_bindings] of int: bound_idx;  % 🔗🔢 -- Indices of the Bound, Held by the Binder's Embrace! -- 🫂✨

% int: num_free;         % 🕊️🔢 -- The Count of Free Variables, Unbound and Seeking Connection! -- 🌌✨
% array[1..num_free] of int: free_idx; % 🕊️🔢 -- Indices of the Free, Drifting in the Semantic Winds! -- 🌬️✨
% array[1..num_free] of int: unrelated_binder_idx; % 🚫🔗🔢 -- Unrelated Binders, Repelling the Free! -- 💥✨

% int: num_apps;         % 🤝🔢 -- The Count of Applications, Orchestrating Function Calls! -- 🎶✨
% array[1..num_apps] of int: func_idx; % 🤝🔢 -- Indices of the Functions, Guiding the Application's Flow! -- 🧭✨
% array[1..num_apps] of int: arg_idx;  % 🤝🔢 -- Indices of the Arguments, Feeding the Function's Hunger! -- 🍎✨

% int: num_hier;         % 🌳🔢 -- The Count of Hierarchical Connections, Building the Tree! -- 🌲✨
% array[1..num_hier] of int: parent_idx; % 🌳🔢 -- Indices of the Parents, Guiding the Lineage! -- 🧬✨
% array[1..num_hier] of int: child_idx;  % 🌳🔢 -- Indices of the Children, Extending the Family Tree! -- 🌿✨

% int: num_alpha;        % 🔄🔢 -- The Count of Alpha Equivalences, Unifying Diverse Forms! -- 💫✨
% array[1..num_alpha] of int: eq1_idx; % 🔄🔢 -- Indices of the First Equivalent, Seeking its Twin! -- 👯✨
% array[1..num_alpha] of int: eq2_idx; % 🔄🔢 -- Indices of the Second Equivalent, Completing the Pair! -- 👯✨

% int: num_beta;         % ⬇️🔢 -- The Count of Beta Reductions, Simplifying Expressions! -- 📉✨
% array[1..num_beta] of int: redex_idx; % ⬇️🔢 -- Indices of the Redexes, Awaiting Transformation! -- 💥✨
% array[1..num_beta] of int: reduced_idx; % ⬇️🔢 -- Indices of the Reduced Forms, The Outcome of Transformation! -- ✅✨
