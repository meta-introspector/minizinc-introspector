### Documentation of MiniZinc Models in `minizinc_models/`

This document provides an overview of the MiniZinc models (`.mzn` files) found in the `minizinc_models/` directory. The files are grouped by their primary function or thematic area, with a comparison of related files and an assessment of their value within the project.

#### 1. Core Utilities and Foundational Components

*   **`church_embeddings.mzn`**:
    *   **Purpose:** Implements foundational concepts of lambda calculus (Church Booleans, Numerals, Lists, Recursion simulation) and the Sieve of Eratosthenes using MiniZinc functions and constraints. It serves as a library of basic building blocks for more complex models.
    *   **Value:** High. Provides essential, reusable components for representing and manipulating abstract concepts within MiniZinc, crucial for the project's goal of embedding lambda calculus. The Sieve is a practical example of declarative programming.

*   **`embedding_helpers.mzn`**:
    *   **Purpose:** Contains helper functions like `get_partition_value` (to retrieve actual values from partitioned indices) and `dot_product_scaled` (for calculating dot products of scaled vectors).
    *   **Value:** High. These are fundamental utility functions used across various embedding models, promoting code reuse and clarity.

*   **`embedding_output.mzn`**:
    *   **Purpose:** Defines the standard output format for the embedding models, primarily displaying the `p_actual_values` array.
    *   **Value:** Medium. Essential for visualizing model results, but its content is straightforward.

*   **`embedding_variables.mzn`**:
    *   **Purpose:** Declares the core variables used in the embedding models: `p_idx` (indices into partitions) and `p_actual_values` (the actual scaled values of the embedding vectors). It also includes a constraint to map `p_idx` to `p_actual_values`.
    *   **Value:** High. Defines the central data structures for the embedding space.

*   **`embedding_backpack_content.mzn`**:
    *   **Purpose:** A placeholder file designed for dynamic injection of content by the "Backpack Filling Protocol." It contains `BP_START` and `BP_END` markers.
    *   **Value:** High (conceptually). Crucial for the project's "Backpack Filling Protocol" and dynamic model generation, enabling LLM-driven modifications. Its current content is minimal but its role is significant.

*   **`sieve_results.mzn`**:
    *   **Purpose:** Collects the results from the Sieve of Eratosthenes (from `church_embeddings.mzn`) into a `primes_list_raw` array and calculates `primes_found_length`.
    *   **Value:** Medium. Connects the Sieve utility to the embedding models, providing prime numbers as potential semantic dimensions.

#### 2. Parameter Definitions and Composition

This group of files defines various parameters used in the embedding models, often broken down into individual files for modularity and then composed. The `_LXX_` naming convention indicates individual parameter declarations, while `_composed_` files include these individual declarations.

*   **`embedding_params_core_LXX_*.mzn` (e.g., `L1_header_comment`, `L2_n_param`, `L3_d_param`, `L4_P_param`, `L5_KAPPA_SCALE_param`, `L6_PARTITION_SCALE_param`)**:
    *   **Purpose:** Define fundamental constants for the embedding space, such as `n` (number of subterms), `d` (dimensionality), `P` (number of partitions), `KAPPA_SCALE`, and `PARTITION_SCALE`. Each file declares a single parameter or a header comment.
    *   **Value:** High. These are the core configurable parameters of the embedding model. The modularity allows for fine-grained control and easier understanding of each parameter's role.

*   **`embedding_params_core_v2.mzn`**:
    *   **Purpose:** Defines the core parameters (`n`, `d`, `P`, `KAPPA_SCALE`, `PARTITION_SCALE`, `num_vec`) directly within one file. This is a consolidated version.
    *   **Value:** High. Provides a quick overview of the core parameters. It's a more direct way to define them compared to the `_LXX_` files if extreme modularity isn't needed for a specific use case.

*   **`embedding_params_core_composed_v3.mzn`**:
    *   **Purpose:** Composes the core parameters by including `embedding_params_core_v2.mzn`. This acts as an aggregator.
    *   **Value:** High. Facilitates the inclusion of all core parameters with a single `include` statement in higher-level models.

*   **`embedding_params_kappa_LXX_*.mzn` (e.g., `L1_header_comment`, `L3_kappa_global` to `L9_kappa_beta`, `L10_kappa_vec`, `L12_derived_header`, `L13_kappa_global_scaled` to `L20_kappa_vec_scaled`, `L22_kappa_unit_norm`, `L23_kappa_unit_norm_scaled`)**:
    *   **Purpose:** Define various "kappa" parameters, which represent different forces or influences within the embedding space (e.g., global influence, binding force, free variable repulsion, application embrace, hierarchical pull, alpha/beta equivalence, vector axiom guidance, unit norm). Both float and scaled integer versions are provided.
    *   **Value:** High. These parameters are crucial for shaping the objective function and defining the "physics" of the embedding. The modularity and scaled versions are valuable for different solver requirements.

*   **`embedding_params_kappa_composed.mzn`**:
    *   **Purpose:** Composes all individual kappa parameter declarations (`_LXX_kappa_*.mzn` files) into a single file.
    *   **Value:** High. Simplifies the inclusion of all kappa parameters in other models.

*   **`embedding_params_other_LXX_*.mzn` (e.g., `L1_header_comment`, `L3_epsilon`, `L4_theta_vec`, `L6_derived_header`, `L7_epsilon_scaled`, `L8_theta_vec_scaled`)**:
    *   **Purpose:** Define auxiliary parameters like `epsilon` (tolerance) and `theta_vec` (vector axiom threshold), along with their scaled versions.
    *   **Value:** Medium. These parameters provide fine-tuning capabilities for the model's behavior.

*   **`embedding_params_other_composed.mzn`**:
    *   **Purpose:** Composes all individual "other" parameter declarations (`_LXX_*.mzn` files) into a single file.
    *   **Value:** Medium. Simplifies inclusion.

*   **`embedding_params_relations_LXX_*.mzn` (e.g., `L1_header_comment`, `L3_num_bindings` to `L31_t_idx`)**:
    *   **Purpose:** Define parameters related to various lambda calculus relationships: bindings, free variables, applications, hierarchical structures, alpha equivalence, beta reduction, and vector axioms. These include counts (`num_bindings`, `num_apps`, etc.) and arrays of indices (`binder_idx`, `func_idx`, etc.).
    *   **Value:** High. These parameters encode the structural and relational information of the lambda terms, which is central to the embedding process.

*   **`embedding_params_relations_v3.mzn`**:
    *   **Purpose:** A consolidated version of relation parameters, without the individual `_LXX_` includes. It defines the parameters directly.
    *   **Value:** High. Provides a more compact definition of relation parameters.

*   **`embedding_params_relations_v3_decorated.mzn`**:
    *   **Purpose:** Similar to `v3`, but includes the `_LXX_` files, effectively decorating the `v3` structure with comments and modularity.
    *   **Value:** High. Combines the conciseness of `v3` with the detailed documentation and modularity of the `_LXX_` files.

*   **`embedding_params_relations_v4.mzn`**:
    *   **Purpose:** A commented-out version of `embedding_params_relations_v3.mzn`. It appears to be a placeholder or an older version that was commented out for testing or refactoring.
    *   **Value:** Low (as an active model). Its value is primarily historical or for reference during development.

*   **`embedding_params_vector_LXX_*.mzn` (e.g., `L1_header_comment`, `L3_num_vec`, `L4_alpha_coeff`, `L5_beta_coeff`, `L6_m_idx`, `L7_n_idx`, `L8_t_idx`, `L10_derived_header`, `L11_alpha_coeff_scaled`, `L12_beta_coeff_scaled`)**:
    *   **Purpose:** Define parameters for vector axioms, including coefficients (`alpha_coeff`, `beta_coeff`) and indices (`m_idx`, `n_idx`, `t_idx`) for the vectors involved in these axioms. Scaled versions are also provided.
    *   **Value:** High. These parameters are essential for defining the vector-based constraints and objectives.

*   **`embedding_params_vector_L3_num_vec_v2.mzn`**:
    *   **Purpose:** A specific declaration for `num_vec` with a comment. Seems redundant given `embedding_params_vector_L3_num_vec.mzn`.
    *   **Value:** Low. Likely a remnant of refactoring.

*   **`embedding_params_vector_v2.mzn`**:
    *   **Purpose:** A consolidated version of vector axiom parameters, similar to `embedding_params_relations_v3.mzn`.
    *   **Value:** High. Provides a compact definition.

*   **`embedding_params_vector_composed_v3.mzn`**:
    *   **Purpose:** Composes vector axiom parameters by including `embedding_params_vector_v2.mzn` and derived scaled parameters.
    *   **Value:** High. Aggregates vector parameters for easier inclusion.

*   **`embedding_params_composed_v4_decorated.mzn`**:
    *   **Purpose:** The highest-level composition file for parameters, including all core, kappa, other, relations, and vector parameters.
    *   **Value:** Very High. This is the "master" parameter file, simplifying the setup of complex models.

#### 3. Constraints and Objectives

*   **`embedding_constraints.mzn`**:
    *   **Purpose:** Defines constraints for the embedding, including unit norm constraints (to ensure vectors have a consistent magnitude) and `alldifferent` constraints (to encourage diversity in embedding dimensions).
    *   **Value:** High. These constraints are critical for shaping the embedding space and ensuring meaningful solutions. The unit norm constraint was a known bottleneck in `v6`.

*   **`embedding_prime_constraints.mzn`**:
    *   **Purpose:** Maps the value of each prime number (from the Sieve) to the first dimension of its embedding vector.
    *   **Value:** Medium. Connects the prime number generation to the embedding space, providing a specific initial condition.

*   **`embedding_objective_v3.mzn`**:
    *   **Purpose:** A simplified objective function for the lambda calculus embedding. It minimizes a sum of squared dot products.
    *   **Value:** Medium. Useful for initial testing or simpler optimization goals.

*   **`embedding_objective_v4.mzn`**:
    *   **Purpose:** An objective function that maximizes the sum of squared Euclidean distances between all pairs of embedding vectors, encouraging diversity.
    *   **Value:** High. Represents a more sophisticated objective for achieving diverse and well-separated embeddings.

#### 4. Model Compositions and Tests

This group contains files that combine various parameters, variables, constraints, and objectives to form complete MiniZinc models, often for specific testing or final versions.

*   **`embedding_sphere.mzn`**:
    *   **Purpose:** The original, monolithic MiniZinc model for the embedding sphere problem. It includes all parameters, variables, constraints, and the objective function directly within one file.
    *   **Value:** Medium (historical/reference). While functional, its monolithic nature makes it harder to maintain and debug compared to the modular approach. It serves as a good reference for the complete problem definition.

*   **`embedding_sphere_final_dummy_v6.mzn`**:
    *   **Purpose:** A dummy MiniZinc model.
    *   **Value:** Low. Likely a placeholder or for very basic testing.

*   **`embedding_sphere_final_v5.mzn`**:
    *   **Purpose:** A composed final model that includes specific versions of parameter compositions (`v3` core, kappa, other, relations, vector), scaled coefficients, simplified relations, variables, helpers, and `v3` objective.
    *   **Value:** High. Represents a working, modular composition of the embedding model, likely a stable checkpoint.

*   **`embedding_sphere_final_v6.mzn`**:
    *   **Purpose:** This file is currently empty. It was likely intended as the next iteration after `v5` but is not yet implemented or is a placeholder for the `v6` reconstruction effort.
    *   **Value:** Low (as an active model). Its value is in its potential future use as the target for the `v6` reconstruction SOP.

*   **`embedding_sphere_simplified_test.mzn`**:
    *   **Purpose:** A simplified test model that includes basic parameters, scaled coefficients, simplified relations, variables, helpers, and a simplified objective.
    *   **Value:** Medium. Useful for quick, lightweight testing of core components.

*   **`embedding_sphere_test_master.mzn`**:
    *   **Purpose:** A consolidated MiniZinc model for testing runtime, including `church_embeddings`, `embedding_params_composed_v4_decorated`, `sieve_results`, `embedding_backpack_content`, `embedding_coeffs_scaled`, `partitioned_manifold_params`, `embedding_variables`, `embedding_helpers`, `embedding_output`, `embedding_prime_constraints`, `embedding_constraints`, and `embedding_objective_v4`. This is a comprehensive test model.
    *   **Value:** High. Serves as a robust integration test for many components of the embedding system.

*   **`embedding_sphere_test_v0.mzn` to `embedding_sphere_test_v12.mzn`**:
    *   **Purpose:** A series of incremental test models. Each `vX` file likely adds one or more components to the previous version, following the stepwise reconstruction approach outlined in the `v6_reconstruction_sop.md`.
    *   **Value:** High. These files are crucial for the `v6` reconstruction effort, allowing for systematic debugging and performance analysis. They embody the "Monotonic Epic Idea" by building complexity incrementally.

*   **`embedding_relations_v3.mzn`**:
    *   **Purpose:** A simplified version of relations, with all declarations commented out.
    *   **Value:** Low (as an active model). Similar to `embedding_params_relations_v4.mzn`, likely a historical or commented-out version.

#### 5. Manifold and Space Models

*   **`partitioned_manifold.mzn`**:
    *   **Purpose:** A basic model demonstrating a partitioned manifold, using `p_idx` to index into `partition_values`.
    *   **Value:** Medium. Illustrates the concept of partitioning a space and mapping indices to values.

*   **`partitioned_manifold_params.mzn`**:
    *   **Purpose:** Defines parameters for the partitioned manifold, specifically `scaled_partition_values`.
    *   **Value:** Medium. Provides the underlying data for the partitioned manifold.

*   **`simple_manifold.mzn`**:
    *   **Purpose:** A very basic model of a 1-bit manifold with two points, demonstrating a simple constraint that points must be in different halves.
    *   **Value:** Low. Primarily for conceptual illustration of a minimal manifold.

*   **`test_unit_norm.mzn`**:
    *   **Purpose:** Specifically tests the unit norm constraint on a single point in an 8-dimensional space, using partitioned values.
    *   **Value:** High. Crucial for debugging and understanding the behavior of the unit norm constraint, which was a known issue.

*   **`univalent_manifold.mzn`**:
    *   **Purpose:** A model for a "univalent manifold," which appears to be an exploration of mapping indices to reciprocals of primes and minimizing a "fudge factor" to achieve a sum of 1.0.
    *   **Value:** Medium. Represents an experimental model exploring alternative embedding strategies, possibly related to univalent foundations or number theory.

*   **`univalent_params.mzn`**:
    *   **Purpose:** Defines parameters for the univalent manifold, including `RECIPROCAL_SCALE` and `scaled_reciprocals` (based on prime numbers).
    *   **Value:** Medium. Provides the data for the univalent manifold model.

#### 6. Lambda Calculus AST and Embedding

*   **`lambda_embedding_v1.mzn`**:
    *   **Purpose:** Version 1 of a MiniZinc model for lambda calculus embedding, focusing on representing a simple lambda term (lambda x. x) as an AST and assigning it a hyperspace location.
    *   **Value:** Medium. Represents an early attempt at directly embedding lambda terms, providing a conceptual starting point for the more complex embedding models.

*   **`lambda_variable_embedding_v1.mzn`**:
    *   **Purpose:** Similar to `lambda_embedding_v1.mzn`, but specifically focuses on embedding a single variable (`x`) and assigning it an arbitrary hyperspace location.
    *   **Value:** Low. A very basic conceptual model, likely for initial exploration.

#### 7. Parameter Generation and Parsing

*   **`generate_vector_params.mzn`**:
    *   **Purpose:** Generates dummy vector parameters (alpha_coeff, beta_coeff, m_idx, n_idx, t_idx) with all values set to zero.
    *   **Value:** Medium. Useful for creating placeholder data files for testing when actual data is not yet available.

*   **`generate_vector_params_v2.mzn`**:
    *   **Purpose:** Generates parameterized vector parameters, allowing for more varied dummy data based on `num_vec` and `base_size`.
    *   **Value:** High. More flexible than `v1` for generating test data.

*   **`parse_vector_params.mzn`**:
    *   **Purpose:** Parses and validates vector parameters from a DZN file. It simply declares the expected parameters and solves for satisfaction.
    *   **Value:** Medium. Useful for ensuring that generated or manually created DZN files conform to the expected structure.

#### 8. Simple Test Models

*   **`test.mzn`**:
    *   **Purpose:** A very basic MiniZinc model that declares an integer variable `x`, constrains it to be greater than 0, and solves for satisfaction.
    *   **Value:** Low. Primarily for testing MiniZinc setup or basic syntax.

*   **`test_func.mzn`**:
    *   **Purpose:** A simple model demonstrating a MiniZinc function `test_func` and its usage.
    *   **Value:** Low. For basic MiniZinc function syntax testing.

---

**Summary and Value Assessment:**

The `minizinc_models/` directory reflects a highly modular and iterative development process.

*   **High Value:** Files related to `embedding_params_core`, `embedding_params_kappa`, `embedding_params_relations`, `embedding_params_vector` (especially the `_composed_` and `_vX` versions), `embedding_constraints`, `embedding_objective_v4`, `embedding_helpers`, `embedding_variables`, `embedding_output`, `church_embeddings`, `sieve_results`, `embedding_sphere_test_master`, and the `embedding_sphere_test_vX` series are of high value. These form the core of the lambda calculus embedding project, defining its structure, parameters, constraints, objectives, and testing methodology. The modular `_LXX_` files are valuable for their clarity and fine-grained control.

*   **Medium Value:** Files like `embedding_backpack_content` (for its conceptual role), `embedding_objective_v3`, `embedding_sphere_simplified_test`, `partitioned_manifold` and its params, `univalent_manifold` and its params, `generate_vector_params` (both versions), and `parse_vector_params` are of medium value. They serve specific purposes, provide alternative objectives, or are useful for testing and data generation.

*   **Low Value:** Files like `embedding_sphere_final_dummy_v6`, `embedding_sphere_final_v6` (due to being empty), `embedding_params_relations_v4` (commented out), `embedding_params_vector_L3_num_vec_v2` (redundant), `lambda_variable_embedding_v1`, `simple_manifold`, `test.mzn`, and `test_func.mzn` are of low value in their current state. They are either placeholders, historical artifacts, or very basic test cases.

**Comparison and Contrast:**

*   **Monolithic vs. Modular:** `embedding_sphere.mzn` stands in stark contrast to the `embedding_sphere_final_v5.mzn` and `embedding_sphere_test_master.mzn` models. The former is a single, large file, while the latter two demonstrate the project's commitment to modularity through extensive use of `include` statements. The modular approach is clearly preferred for maintainability and reusability.
*   **Objective Functions:** `embedding_objective_v3.mzn` and `embedding_objective_v4.mzn` show an evolution in the project's optimization goals, from a simplified objective to one that explicitly maximizes diversity.
*   **Parameter Definitions:** The `_LXX_` files versus the consolidated `_vX.mzn` files for parameters (e.g., `embedding_params_core_LXX_*.mzn` vs. `embedding_params_core_v2.mzn`) illustrate a trade-off between extreme modularity (for fine-grained control and documentation) and consolidation (for conciseness). The `_decorated.mzn` files attempt to combine both.
*   **Relations:** The `embedding_params_relations_v3.mzn` and `embedding_params_relations_v3_decorated.mzn` show how the same set of parameters can be presented with or without extensive inline documentation and modular includes. `embedding_params_relations_v4.mzn` serves as a cautionary example of commented-out code that should ideally be removed or properly versioned if no longer active.
*   **Test Progression:** The `embedding_sphere_test_v0.mzn` to `v12.mzn` series is a clear example of incremental development and testing, directly supporting the `v6_reconstruction_sop.md`.
