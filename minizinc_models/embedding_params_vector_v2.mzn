% Vector Axiom Parameters for Lambda Calculus Embedding (Version 2 - without num_vec) ➡️✨ -- The Guiding Truths! -- 🧭💡

% int: num_vec; % ➡️🔢 -- The Count of Vector Axioms, Defining the Space of Truths! -- 🌌✨
array[1..num_vec] of float: alpha_coeff; % ➡️🔢 -- Alpha Coefficients, Scaling the Vector's Influence! -- 📈✨
array[1..num_vec] of float: beta_coeff;  % ➡️🔢 -- Beta Coefficients, Modulating the Vector's Harmony! -- 🎶✨
array[1..num_vec] of int: m_idx;    % ➡️🔢 -- Indices of Vector M, A Pillar of Axiomatic Construction! -- 🏗️✨
array[1..num_vec] of int: n_idx;    % ➡️🔢 -- Indices of Vector N, Another Pillar of Axiomatic Construction! -- 🏗️✨
array[1..num_vec] of int: t_idx;    % ➡️🔢 -- Indices of Vector T, The Target of Axiomatic Alignment! -- 🎯✨

% Derived scaled integer parameters 🔢✨ -- The Quantized Vector Axiom Coefficients! -- 📏💡
array[1..num_vec] of int: alpha_coeff_scaled = [round(alpha_coeff[i] * PARTITION_SCALE) | i in 1..num_vec]; % ➡️🔢 -- Alpha Coefficients, Quantized! -- ✨
array[1..num_vec] of int: beta_coeff_scaled = [round(beta_coeff[i] * PARTITION_SCALE) | i in 1..num_vec]; % ➡️🔢 -- Beta Coefficients, Quantized! -- ✨
