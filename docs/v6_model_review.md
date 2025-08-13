# V6 MiniZinc Model Review, Testing, and QA

This document provides a detailed review of each MiniZinc file comprising the `v6` embedding model. For each file, we will explain its purpose, describe its contents, outline how it can be tested or used, and highlight key Quality Assurance (QA) considerations.

---

## 1. `embedding_sphere_final_v6.mzn`

*   **Purpose:** This is the main entry point for the `v6` embedding model. It orchestrates the entire model by including all necessary parameter definitions, variable declarations, helper functions, constraints, objective functions, and output specifications. It defines the overall structure and behavior of the hyperspace embedding problem for the `v6` iteration.

*   **Contents:**
    *   **Includes:** It primarily consists of `include` statements that pull in various modular components of the model. These components define:
        *   Composed parameters (core, kappa, other, relations, vector).
        *   Backpack content (for dynamic knowledge injection).
        *   Scaled coefficients.
        *   Partitioned manifold parameters.
        *   Embedding variables (the actual points in hyperspace).
        *   Helper functions.
        *   Core constraints.
        *   The objective function.
        *   Output formatting.
    *   **No direct declarations:** This file itself does not contain direct variable declarations, constraints, or solve statements. Its role is purely compositional.

*   **How to Test/Use It:**
    *   **Primary Execution:** This file is designed to be run via the `run_embedding_model_v6.sh` script, which handles the dynamic loading of data parameters.
        ```bash
        ./scripts/run_embedding_model_v6.sh v6 v1 v1 v1 v1 v1
        ```
    *   **Expected Behavior:** When run, it should either find a solution (if satisfiable) or report `=====UNSATISFIABLE=====`. The output will be formatted according to `embedding_output.mzn`.
    *   **Debugging:** The `run_embedding_model_v6.sh` script has built-in debugging features that display the MiniZinc command and the head of the `stdout.log` and `stderr.log`, which are crucial for diagnosing issues with this main model.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all required included files exist and are correctly referenced. A missing include will cause a compilation error.
    *   **Parameter Compatibility:** Verify that the versions of included parameter files (`v4_decorated`, `v3`, etc.) are compatible with the variables and constraints defined in other parts of the model. Incompatibilities can lead to "undefined identifier" errors or unexpected behavior.
    *   **Overall Model Coherence:** The successful execution of this file (even if unsatisfiable) indicates that the entire model compiles and links correctly. Any errors at this stage often point to issues in included sub-models or parameter definitions.
    *   **Output Consistency:** Check that the output format matches expectations defined in `embedding_output.mzn`.

---

## 2. `embedding_params_composed_v4_decorated.mzn`

*   **Purpose:** This file serves as a high-level composition of all parameter definitions for the `v4` iteration of the composed parameters, specifically designed for the `v6` main model. It acts as a central hub for pulling in various categories of parameters (core, kappa, other, relations, vector) into a single, organized structure. The "decorated" in its name suggests it might include additional metadata or formatting.

*   **Contents:**
    *   **Includes:** This file primarily includes other `.mzn` files, each responsible for defining a specific set of parameters:
        *   `embedding_params_core_composed_v3.mzn`: Core model parameters.
        *   `embedding_params_kappa_composed.mzn`: Parameters related to kappa forces/weights.
        *   `embedding_params_other_composed.mzn`: Auxiliary or miscellaneous parameters.
        *   `embedding_params_relations_v3.mzn`: Parameters defining relationships within the model.
        *   `embedding_params_vector_composed_v3.mzn`: Parameters related to vector axioms.
    *   **Comments:** Contains rich, descriptive comments and emojis that explain the purpose and "vibe" of each included parameter set, aligning with the project's "Muse Protocol."

*   **How to Test/Use It:**
    *   **Indirectly Tested:** This file is not typically run in isolation. Its correctness is verified when `embedding_sphere_final_v6.mzn` (which includes it) compiles and runs without "undefined identifier" errors related to parameters.
    *   **Parameter Inspection:** To understand the parameters defined, one would recursively examine the files it includes.
    *   **Dependency Check:** Can be used to quickly see which major parameter groups are being used by the `v6` model.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all included parameter files exist and are correctly referenced.
    *   **Parameter Overlap/Conflict:** Verify that there are no conflicting definitions of the same parameter across different included files. While MiniZinc allows re-definition, it can lead to unexpected behavior if not intentional.
    *   **Completeness:** Ensure that all parameters required by the main model (`embedding_sphere_final_v6.mzn`) and its other components are eventually defined through this composition. Missing parameters will result in "undefined identifier" errors.
    *   **Version Compatibility:** Confirm that the versions of the included parameter files (`v3`, `v3`, etc.) are compatible with the expectations of the `v6` model.

---

## 3. `embedding_backpack_content.mzn`

*   **Purpose:** This file is designed for "dynamic knowledge injection" into the model. It's intended to hold parameters or data that can be changed or updated frequently without altering the core model structure. The "backpack" metaphor suggests it carries essential, portable content.

*   **Contents:**
    *   **Parameters/Data:** This file would typically contain declarations of parameters or initial data values that are meant to be flexible and easily swapped. The specific content would depend on the "knowledge" being injected.
    *   **Comments:** Likely contains comments explaining the nature of the injected knowledge and its role in the model.

*   **How to Test/Use It:**
    *   **Inspection:** Review the contents of this file to understand what dynamic knowledge is being provided to the model.
    *   **Modification:** To test different knowledge injections, one would modify the parameter values within this file.
    *   **Integration Test:** Its effect is observed when the main model (`embedding_sphere_final_v6.mzn`) is run with different versions or contents of this file.

*   **QA Considerations:**
    *   **Parameter Validity:** Ensure that any parameters declared or defined here are within expected ranges and types required by the rest of the model. Invalid values can lead to runtime errors or unexpected model behavior.
    *   **Impact on Model:** Understand how changes in this file affect the overall model's behavior, satisfiability, or objective function.
    *   **Documentation:** Ensure that the purpose and expected content of this "backpack" are clearly documented, especially if it's intended for frequent modification.

---

## 4. `embedding_coeffs_scaled.mzn`

*   **Purpose:** This file likely defines scaled coefficients used within the embedding model. In MiniZinc, especially when dealing with integer-based solvers, floating-point numbers are often scaled up to integers to maintain precision and allow for efficient constraint propagation. This file would contain the declarations and definitions of these scaled coefficients.

*   **Contents:**
    *   **Scaled Parameters:** Declarations of `int` or `array of int` parameters representing scaled versions of what might conceptually be floating-point coefficients.
    *   **Scaling Logic (Implicit):** The scaling factor (e.g., `KAPPA_SCALE`, `PARTITION_SCALE`) would be defined elsewhere (e.g., in core parameters), and the values in this file would be the result of applying that scaling.

*   **How to Test/Use It:**
    *   **Inspection:** Review the declarations and values to understand the scaled coefficients being used.
    *   **Consistency Check:** Verify that the scaling applied here is consistent with the `KAPPA_SCALE` and `PARTITION_SCALE` values defined in the core parameters.
    *   **Impact Analysis:** Analyze how changes to these scaled coefficients affect the model's behavior and solution space.

*   **QA Considerations:**
    *   **Scaling Accuracy:** Ensure that the scaling process (conceptual or explicit) maintains sufficient precision for the problem's requirements. Rounding errors can lead to unsatisfiability or incorrect solutions.
    *   **Range & Domain:** Verify that the scaled integer values remain within the valid range for MiniZinc integers and the intended domain of the variables they influence.
    *   **Consistency with Floating-Point Intent:** If these coefficients represent scaled versions of real-world floating-point values, ensure that the integer representation accurately reflects the original intent.

---

## 5. `partitioned_manifold_params.mzn`

*   **Purpose:** This file defines parameters related to the "partitioned manifold," which is a conceptual grid or discretization of the hyperspace. These parameters are crucial for defining the structure and granularity of the space in which the lambda terms are embedded.

*   **Contents:**
    *   **Grid Dimensions:** Parameters defining the number of partitions, the size of each partition, or the overall dimensions of the manifold.
    *   **Scaling Factors:** Parameters that might influence how values are mapped to and from this partitioned space.
    *   **Derived Parameters:** Could include parameters derived from the basic grid dimensions.

*   **How to Test/Use It:**
    *   **Inspection:** Review the parameters to understand the current partitioning scheme of the hyperspace.
    *   **Impact Analysis:** Analyze how changes to these parameters would affect the density, resolution, and overall structure of the embedding space.
    *   **Visualization (Conceptual):** These parameters would directly influence how the hyperspace is visualized, allowing for different "views" of the manifold.

*   **QA Considerations:**
    *   **Consistency with `PARTITION_SCALE`:** Ensure that any scaling factors or partition sizes defined here are consistent with `PARTITION_SCALE` from the core parameters.
    *   **Feasibility:** Verify that the defined partitioning parameters are feasible for the solver (e.g., not too large, leading to combinatorial explosion).
    *   **Coverage:** Ensure that the partitioned manifold adequately covers the intended range of the embedding space.

---

## 6. `embedding_variables.mzn`

*   **Purpose:** This file declares the core decision variables of the embedding model. These variables represent the actual hyperspace coordinates (or indices into a partitioned space) where the lambda terms are placed. They are the unknowns that the MiniZinc solver will determine.

*   **Contents:**
    *   **`p_idx`:** An array of `var int` representing indices into the `partition_values` array (which is likely defined in `partitioned_manifold_params.mzn` or a related file). This indicates which specific partition each dimension of a lambda term's embedding occupies.
    *   **`p_actual_values`:** An array of `var int` representing the actual numerical coordinates in the hyperspace.
    *   **Constraint:** A `forall` constraint that links `p_actual_values` to `p_idx` using a `get_partition_value` function (likely defined in `embedding_helpers.mzn`). This ensures that the actual coordinates are derived from the chosen partition indices.

*   **How to Test/Use It:**
    *   **Inspection:** Review the declarations to understand the dimensionality (`n` and `d`) and the types of the variables being solved for.
    *   **Output Analysis:** After a successful model run, examine the output values of `p_idx` and `p_actual_values` to see the computed embedding locations.
    *   **Domain Testing:** Experiment with different domains for `p_idx` (e.g., `1..P`) to see how it affects the solution space and solver performance.

*   **QA Considerations:**
    *   **Variable Domains:** Ensure that the domains of `p_idx` and `p_actual_values` are correctly defined and consistent with the `P` (number of partitions) and `PARTITION_SCALE` parameters. Incorrect domains can lead to unsatisfiability or incorrect solutions.
    *   **Constraint Correctness:** Verify that the constraint linking `p_actual_values` to `p_idx` accurately reflects the intended mapping from partition indices to actual coordinates.
    *   **Solvability:** The number and domain of these variables directly impact the complexity and solvability of the model. Large `n` or `d` values, or wide domains for `p_idx`, can lead to long solve times or memory issues.

---

## 7. `embedding_helpers.mzn`

*   **Purpose:** This file contains helper functions and predicates that simplify the modeling of complex relationships within the embedding. These functions encapsulate reusable logic, making the main model more readable and maintainable.

*   **Contents:**
    *   **`get_partition_value` function:** This is a crucial function that maps an index (`p_idx`) to an actual numerical value (`p_actual_values`) within the partitioned hyperspace. It likely uses `PARTITION_SCALE` and `P` (number of partitions) to calculate the coordinate.
    *   **Other Helper Functions:** Could include functions for distance calculations, vector operations, or other common mathematical utilities used throughout the model.

*   **How to Test/Use It:**
    *   **Unit Testing (Conceptual):** While MiniZinc doesn't have a direct unit testing framework, one could create small `.mzn` files that call these helper functions with various inputs and assert the expected outputs.
    *   **Inspection:** Review the logic of these functions to ensure they correctly implement the intended mathematical or logical operations.
    *   **Debugging Aid:** When debugging the main model, understanding the behavior of these helper functions is critical.

*   **QA Considerations:**
    *   **Correctness:** Ensure that all helper functions and predicates correctly implement their specified logic. Errors here can propagate throughout the entire model.
    *   **Edge Cases:** Test helper functions with edge cases (e.g., boundary values, zero inputs, maximum/minimum indices) to ensure robust behavior.
    *   **Performance:** For frequently called helper functions, consider their computational complexity. Inefficient helper functions can significantly impact solver performance.
    *   **Consistency:** Verify that the helper functions are consistent with the overall scaling and partitioning schemes defined in other parameter files.

---

## 8. `embedding_constraints.mzn`

*   **Purpose:** This file defines the core constraints that shape the hyperspace embedding. These constraints are crucial for ensuring that the assigned locations of lambda terms adhere to specific mathematical or logical properties. This is where the "rules" of the hyperspace are enforced.

*   **Contents:**
    *   **Unit Norm Constraint (Commented Out):** The primary constraint in this file is a `forall` constraint that aims to enforce a "unit norm" property on the `p_actual_values`. It requires the sum of squares of the coordinates for each lambda term to equal `PARTITION_SCALE * PARTITION_SCALE`.
        *   **Note:** This constraint was identified as the cause of unsatisfiability in the `v6` model with current parameters and is currently commented out for debugging purposes.
    *   **Other Constraints (Potential):** This file could contain other constraints related to distance, relationships between terms, or properties of the embedding.

*   **How to Test/Use It:**
    *   **Unsatisfiability Testing:** This file is critical for testing the satisfiability of the model. If the model is unsatisfiable, this file is often the first place to investigate.
    *   **Constraint Relaxation/Modification:** To achieve satisfiability or explore different embedding properties, one would modify or relax the constraints within this file.
    *   **Impact Analysis:** Analyze how each constraint restricts the solution space and influences the resulting embedding.

*   **QA Considerations:**
    *   **Correctness:** Ensure that the constraints accurately reflect the intended mathematical or logical properties.
    *   **Satisfiability:** Test the constraints with various parameter sets to understand their impact on model satisfiability. A constraint that always leads to unsatisfiability might be too restrictive or incorrectly formulated.
    *   **Redundancy/Conflict:** Check for redundant or conflicting constraints that might unnecessarily complicate the model or lead to unsatisfiability.
    *   **Performance:** Complex constraints can significantly impact solver performance. Consider alternative formulations if performance is an issue.
    *   **Documentation:** Clearly document the purpose and implications of each constraint, especially if they are known to cause unsatisfiability under certain conditions.

---

## 9. `embedding_objective_v3.mzn`

*   **Purpose:** This file defines the objective function for the `v3` iteration of the embedding model. The objective function is what the MiniZinc solver attempts to optimize (minimize or maximize) while satisfying all constraints. In embedding problems, this often relates to minimizing "distortion" or maximizing "coherence" in the hyperspace.

*   **Contents:**
    *   **`solve minimize ...` or `solve maximize ...`:** The core of this file will be a `solve` statement with an `objective` expression.
    *   **Objective Expression:** This expression will typically be a mathematical formula involving the decision variables (e.g., `p_actual_values`) and parameters, designed to quantify the "quality" of an embedding. For `v3`, it might be a simplified objective.

*   **How to Test/Use It:**
    *   **Optimization Test:** Run the model and observe the value of the objective function in the solution.
    *   **Sensitivity Analysis:** Modify parameters or constraints and observe how the optimal objective value changes.
    *   **Alternative Objectives:** Experiment with different objective functions to explore various embedding criteria.

*   **QA Considerations:**
    *   **Correctness:** Ensure that the objective function accurately reflects the intended optimization goal.
    *   **Monotonicity/Continuity:** For continuous optimization problems, ensure the objective function behaves as expected (e.g., no sudden jumps or plateaus that could trap the solver).
    *   **Computational Complexity:** Complex objective functions can significantly increase solve time. Consider simpler formulations if performance is an issue.
    *   **Relationship to Constraints:** The objective function should be well-defined within the feasible region established by the constraints. If the objective pushes variables to extreme values that violate constraints, it can lead to unsatisfiability.

---

## 10. `embedding_output.mzn`

*   **Purpose:** This file defines how the solution found by the MiniZinc solver is presented. It specifies the format and content of the output, making the results human-readable and interpretable.

*   **Contents:**
    *   **`output` statement:** The core of this file is the `output` statement, which uses MiniZinc's string formatting capabilities to construct the desired output.
    *   **Output Variables:** It references the decision variables (e.g., `p_actual_values`) and potentially other parameters or derived values that are relevant for understanding the solution.

*   **How to Test/Use It:**
    *   **Visual Inspection:** After a model run, visually inspect the output to ensure it is correctly formatted and contains all the expected information.
    *   **Debugging Aid:** Modify the output statement to print additional variables or intermediate calculations for debugging purposes.
    *   **Integration with External Tools:** The output format can be designed to be easily parsed by external tools for visualization or further analysis.

*   **QA Considerations:**
    *   **Completeness:** Ensure that all critical information from the solution is included in the output.
    *   **Clarity & Readability:** The output should be easy to understand and interpret by a human.
    *   **Consistency:** Maintain a consistent output format across different model runs or versions.
    *   **Error Handling:** Consider how the output behaves in cases of unsatisfiability or errors (though MiniZinc typically handles this by reporting the error directly).

---

## 11. `embedding_params_core_composed_v3.mzn`

*   **Purpose:** This file composes the core parameters for the `v3` iteration of the model. It acts as an intermediary, pulling in the fundamental constants and dimensions that define the basic structure of the hyperspace and the problem.

*   **Contents:**
    *   **Includes:** Primarily includes `embedding_params_core_v2.mzn`, which contains the actual declarations of core parameters like `n` (number of subterms), `d` (dimensionality), `P` (number of partitions), `KAPPA_SCALE`, `PARTITION_SCALE`, and `num_vec`.
    *   **Comments:** Contains descriptive comments about its role in composing core parameters.

*   **How to Test/Use It:**
    *   **Indirectly Tested:** Like `embedding_params_composed_v4_decorated.mzn`, this file is not run in isolation. Its correctness is verified when higher-level models that include it compile and run without "undefined identifier" errors related to core parameters.
    *   **Parameter Inspection:** To understand the core parameters, one would examine `embedding_params_core_v2.mzn`.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure `embedding_params_core_v2.mzn` exists and is correctly referenced.
    *   **Parameter Consistency:** Verify that the parameters defined in `embedding_params_core_v2.mzn` are consistent with the expectations of the overall `v6` model.
    *   **Version Control:** Ensure that the correct version (`v2`) of the core parameters is being included.

---

## 12. `embedding_params_core_v2.mzn`

*   **Purpose:** This file declares the fundamental core parameters of the embedding model for its `v2` iteration. These parameters define the basic dimensions and scales of the problem, serving as constants that influence the entire model.

*   **Contents:**
    *   **`n` (int):** The number of subterms or entities being embedded. This is a parameter that needs to be provided by a `.dzn` file.
    *   **`d` (int):** The dimensionality of the hyperspace (fixed at 8 in this version).
    *   **`P` (int):** The number of partitions or discrete values available along each dimension of the hyperspace.
    *   **`KAPPA_SCALE` (int):** A scaling factor for kappa-related parameters, used to convert conceptual floating-point values to integers.
    *   **`PARTITION_SCALE` (int):** A scaling factor for partition values, used to convert conceptual floating-point values to integers.
    *   **`num_vec` (int):** The count of vector axioms, another parameter that needs to be provided.

*   **How to Test/Use It:**
    *   **Direct Inspection:** The values of `d`, `P`, `KAPPA_SCALE`, and `PARTITION_SCALE` are directly visible and can be inspected.
    *   **Parameter Provision:** To use this file, `n` and `num_vec` must be provided via a `.dzn` file (e.g., `example_core_params_v1.dzn` or `lambda_params.dzn`).
    *   **Impact Analysis:** Experiment with different values for `n`, `P`, `KAPPA_SCALE`, and `PARTITION_SCALE` (by modifying this file or providing them in a `.dzn` file) to observe their impact on model size, complexity, and solution characteristics.

*   **QA Considerations:**
    *   **Parameter Ranges:** Ensure that the default or provided values for these parameters are within reasonable and expected ranges for the problem.
    *   **Consistency:** Verify that these core parameters are consistent with the assumptions and requirements of other parts of the model (e.g., `d=8` should match the expected dimensionality everywhere).
    *   **Scaling Correctness:** The `KAPPA_SCALE` and `PARTITION_SCALE` values are critical for the integer discretization strategy. Ensure they are chosen appropriately to maintain precision and avoid overflow.
    *   **Documentation:** Clearly document the meaning and intended use of each parameter.

---

## 13. `embedding_params_kappa_composed.mzn`

*   **Purpose:** This file composes various "kappa" parameters, which represent different types of forces or influences within the hyperspace embedding. These forces likely govern relationships, attractions, or repulsions between embedded lambda terms.

*   **Contents:**
    *   **Includes:** This file includes a series of `embedding_params_kappa_L*.mzn` files. Each `L` file defines a specific kappa parameter (e.g., `kappa_global`, `kappa_bind`, `kappa_free`, `kappa_app`, `kappa_hier`, `kappa_alpha`, `kappa_beta`, `kappa_vec`, `kappa_unit_norm`).
    *   **Scaled Versions:** It also includes scaled versions of these kappa parameters (e.g., `kappa_global_scaled`), which are used for integer-based solvers.
    *   **Comments:** Contains rich, descriptive comments and emojis explaining the conceptual meaning of each kappa parameter.

*   **How to Test/Use It:**
    *   **Indirectly Tested:** Like other composed parameter files, its correctness is verified when higher-level models that include it compile and run without "undefined identifier" errors related to kappa parameters.
    *   **Parameter Inspection:** To understand the individual kappa parameters, one would examine the specific `embedding_params_kappa_L*.mzn` files it includes.
    *   **Impact Analysis:** Experiment with different values for these kappa parameters (by modifying the `L` files or providing them in a `.dzn` file) to observe their impact on the model's behavior and the resulting embedding.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all included `embedding_params_kappa_L*.mzn` files exist and are correctly referenced.
    *   **Parameter Consistency:** Verify that the kappa parameters are consistent with the overall model's design and the intended "forces" they represent.
    *   **Scaling Correctness:** For the scaled versions, ensure the scaling is applied correctly and maintains precision.
    *   **Documentation:** Clearly document the meaning and intended use of each kappa parameter, especially their conceptual role in the embedding.

---

## 14. `embedding_params_other_composed.mzn`

*   **Purpose:** This file composes various "other" or auxiliary parameters that are not categorized as core, kappa, relations, or vector parameters. These parameters often represent fine-tuning constants, thresholds, or specific values that influence the model's behavior in a more specialized way.

*   **Contents:**
    *   **Includes:** This file includes a series of `embedding_params_other_L*.mzn` files. Each `L` file defines a specific "other" parameter (e.g., `epsilon`, `theta_vec`).
    *   **Scaled Versions:** It also includes scaled versions of these parameters (e.g., `epsilon_scaled`, `theta_vec_scaled`), used for integer-based solvers.
    *   **Comments:** Contains descriptive comments and emojis explaining the conceptual meaning of each parameter.

*   **How to Test/Use It:**
    *   **Indirectly Tested:** Like other composed parameter files, its correctness is verified when higher-level models that include it compile and run without "undefined identifier" errors related to these parameters.
    *   **Parameter Inspection:** To understand the individual "other" parameters, one would examine the specific `embedding_params_other_L*.mzn` files it includes.
    *   **Impact Analysis:** Experiment with different values for these parameters (by modifying the `L` files or providing them in a `.dzn` file) to observe their impact on the model's behavior and the resulting embedding.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all included `embedding_params_other_L*.mzn` files exist and are correctly referenced.
    *   **Parameter Consistency:** Verify that these parameters are consistent with the overall model's design and their intended influence.
    *   **Scaling Correctness:** For the scaled versions, ensure the scaling is applied correctly and maintains precision.
    *   **Documentation:** Clearly document the meaning and intended use of each parameter.

---

## 15. `embedding_params_relations_v3.mzn`

*   **Purpose:** This file defines parameters related to various types of relationships within the lambda calculus and its embedding. These relationships are crucial for modeling how lambda terms interact, bind, apply, and reduce. The `v3` iteration suggests a refined or expanded set of relations.

*   **Contents:**
    *   **Includes:** This file includes a series of `embedding_params_relations_L*.mzn` files. Each `L` file defines parameters for a specific type of relation (e.g., `num_bindings`, `binder_idx`, `bound_idx`, `num_free`, `free_idx`, `num_apps`, `func_idx`, `arg_idx`, `num_hier`, `parent_idx`, `child_idx`, `num_alpha`, `eq1_idx`, `eq2_idx`, `num_beta`, `redex_idx`, `reduced_idx`, `num_vec`).
    *   **Comments:** Contains descriptive comments and emojis explaining the conceptual meaning of each relation parameter.

*   **How to Test/Use It:**
    *   **Indirectly Tested:** Like other composed parameter files, its correctness is verified when higher-level models that include it compile and run without "undefined identifier" errors related to relation parameters.
    *   **Parameter Inspection:** To understand the individual relation parameters, one would examine the specific `embedding_params_relations_L*.mzn` files it includes.
    *   **Impact Analysis:** Experiment with different values for these parameters (by modifying the `L` files or providing them in a `.dzn` file) to observe their impact on the model's ability to represent and reason about lambda calculus relationships.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all included `embedding_params_relations_L*.mzn` files exist and are correctly referenced.
    *   **Parameter Consistency:** Verify that these parameters are consistent with the overall model's design and the intended relationships they represent.
    *   **Completeness:** Ensure that all necessary relation parameters are defined for the model to accurately capture lambda calculus semantics.
    *   **Documentation:** Clearly document the meaning and intended use of each relation parameter, especially their role in defining the structure and dynamics of lambda terms.

---

## 16. `embedding_params_vector_composed_v3.mzn`

*   **Purpose:** This file composes various "vector" parameters for the `v3` iteration of the model. These parameters are likely related to the axioms or properties of vectors within the hyperspace, influencing how vector operations are performed or how vector-like entities are represented.

*   **Contents:**
    *   **Includes:** This file includes `embedding_params_vector_v2.mzn` and a series of `embedding_params_vector_L*.mzn` files. These `L` files define specific vector parameters (e.g., `alpha_coeff`, `beta_coeff`, `m_idx`, `n_idx`, `t_idx`).
    *   **Scaled Versions:** It also includes scaled versions of these parameters (e.g., `alpha_coeff_scaled`, `beta_coeff_scaled`), used for integer-based solvers.
    *   **Comments:** Contains descriptive comments and emojis explaining the conceptual meaning of each vector parameter.

*   **How to Test/Use It:**
    *   **Indirectly Tested:** Like other composed parameter files, its correctness is verified when higher-level models that include it compile and run without "undefined identifier" errors related to vector parameters.
    *   **Parameter Inspection:** To understand the individual vector parameters, one would examine the specific `embedding_params_vector_L*.mzn` files it includes.
    *   **Impact Analysis:** Experiment with different values for these parameters (by modifying the `L` files or providing them in a `.dzn` file) to observe their impact on the model's behavior and the resulting embedding.

*   **QA Considerations:**
    *   **Inclusion Integrity:** Ensure all included `embedding_params_vector_L*.mzn` files exist and are correctly referenced.
    *   **Parameter Consistency:** Verify that these parameters are consistent with the overall model's design and the intended vector properties they represent.
    *   **Scaling Correctness:** For the scaled versions, ensure the scaling is applied correctly and maintains precision.
    *   **Documentation:** Clearly document the meaning and intended use of each vector parameter.

---

## 17. Individual Parameter Files (L-Files)

These files represent the atomic declarations of parameters within the model. They are typically included by "composed" parameter files (like `embedding_params_kappa_composed.mzn`, `embedding_params_other_composed.mzn`, etc.) to build up the complete set of parameters used by the model.

### 17.1. Kappa Parameters (from `embedding_params_kappa_composed.mzn`)

*   **Purpose:** These files define individual "kappa" parameters, each representing a specific force or influence within the hyperspace embedding. They are fundamental constants that govern relationships, attractions, or repulsions between embedded lambda terms.

*   **Contents:**
    *   **Single Parameter Declaration:** Each file typically contains a single `float` or `int` parameter declaration, often with an initial value.
    *   **Comments:** Rich, descriptive comments and emojis explain the conceptual meaning and role of the parameter.
    *   **Scaled Versions:** Some files might define a scaled version of the parameter (e.g., `kappa_global_scaled`), derived from the base parameter and a scaling factor (e.g., `KAPPA_SCALE`).

*   **Representative Example: `embedding_params_kappa_L3_kappa_global.mzn`**
    *   **Purpose:** Defines the `kappa_global` parameter, representing a global influence or force.
    *   **Contents:** `float: kappa_global;` (or `float: kappa_global = <value>;`)
    *   **How to Test/Use It:**
        *   **Direct Inspection:** Verify the declared type and initial value.
        *   **Impact Analysis:** Modify its value (if it has a default) or provide it via a `.dzn` file to observe its impact on the model's behavior.
    *   **QA Considerations:**
        *   **Type Correctness:** Ensure the parameter is declared with the correct type (`float` or `int`).
        *   **Value Range:** Verify that the default or provided value is within a reasonable range for the intended purpose.
        *   **Scaling Consistency:** If a scaled version exists, ensure the scaling logic is correctly applied and consistent with `KAPPA_SCALE`.

---

### 17.2. Other Parameters (from `embedding_params_other_composed.mzn`)

*   **Purpose:** These files define individual "other" or auxiliary parameters. These parameters often represent fine-tuning constants, thresholds, or specific values that influence the model's behavior in a more specialized way, outside of the core, kappa, relations, or vector categories.

*   **Contents:**
    *   **Single Parameter Declaration:** Each file typically contains a single `float` or `int` parameter declaration, often with an initial value.
    *   **Comments:** Rich, descriptive comments and emojis explain the conceptual meaning and role of the parameter.
    *   **Scaled Versions:** Some files might define a scaled version of the parameter (e.g., `epsilon_scaled`), derived from the base parameter and a scaling factor.

*   **Representative Example: `embedding_params_other_L3_epsilon.mzn`**
    *   **Purpose:** Defines the `epsilon` parameter, often representing a small tolerance or a threshold value.
    *   **Contents:** `float: epsilon;` (or `float: epsilon = <value>;`)
    *   **How to Test/Use It:**
        *   **Direct Inspection:** Verify the declared type and initial value.
        *   **Impact Analysis:** Modify its value (if it has a default) or provide it via a `.dzn` file to observe its impact on the model's behavior, especially in constraints or objective functions where it's used.
    *   **QA Considerations:**
        *   **Type Correctness:** Ensure the parameter is declared with the correct type (`float` or `int`).
        *   **Value Range:** Verify that the default or provided value is within a reasonable range for the intended purpose.
        *   **Scaling Consistency:** If a scaled version exists, ensure the scaling logic is correctly applied and consistent with the relevant scaling factor.

---

### 17.3. Relations Parameters (from `embedding_params_relations_v3.mzn`)

*   **Purpose:** These files define individual parameters related to various types of relationships within the lambda calculus and its embedding. These parameters are crucial for modeling how lambda terms interact, bind, apply, and reduce.

*   **Contents:**
    *   **Single Parameter Declaration:** Each file typically contains a single `int` parameter declaration, often representing counts or indices related to specific relationships (e.g., `num_bindings`, `binder_idx`, `func_idx`).
    *   **Comments:** Rich, descriptive comments and emojis explain the conceptual meaning and role of the parameter.

*   **Representative Example: `embedding_params_relations_L3_num_bindings.mzn`**
    *   **Purpose:** Defines the `num_bindings` parameter, representing the total number of binding relationships in the lambda term.
    *   **Contents:** `int: num_bindings;` (or `int: num_bindings = <value>;`)
    *   **How to Test/Use It:**
        *   **Direct Inspection:** Verify the declared type and initial value.
        *   **Impact Analysis:** Modify its value (if it has a default) or provide it via a `.dzn` file to observe its impact on the model's ability to represent and reason about lambda calculus relationships.
    *   **QA Considerations:**
        *   **Type Correctness:** Ensure the parameter is declared with the correct type (`int`).
        *   **Value Range:** Verify that the default or provided value is within a reasonable range for the intended purpose (e.g., non-negative for counts).
        *   **Consistency:** Ensure consistency with the actual structure of the lambda terms being modeled.

---

### 17.4. Vector Parameters (from `embedding_params_vector_composed_v3.mzn`)

*   **Purpose:** These files define individual "vector" parameters. These parameters are likely related to the axioms or properties of vectors within the hyperspace, influencing how vector operations are performed or how vector-like entities are represented.

*   **Contents:**
    *   **Single Parameter Declaration:** Each file typically contains a single `float` or `int` parameter declaration, often with an initial value.
    *   **Comments:** Rich, descriptive comments and emojis explain the conceptual meaning and role of the parameter.
    *   **Scaled Versions:** Some files might define a scaled version of the parameter (e.g., `alpha_coeff_scaled`), derived from the base parameter and a scaling factor.

*   **Representative Example: `embedding_params_vector_L4_alpha_coeff.mzn`**
    *   **Purpose:** Defines the `alpha_coeff` parameter, likely a coefficient used in vector calculations.
    *   **Contents:** `float: alpha_coeff;` (or `float: alpha_coeff = <value>;`)
    *   **How to Test/Use It:**
        *   **Direct Inspection:** Verify the declared type and initial value.
        *   **Impact Analysis:** Modify its value (if it has a default) or provide it via a `.dzn` file to observe its impact on the model's behavior, especially in vector-related constraints or objectives.
    *   **QA Considerations:**
        *   **Type Correctness:** Ensure the parameter is declared with the correct type (`float` or `int`).
        *   **Value Range:** Verify that the default or provided value is within a reasonable range for the intended purpose.
        *   **Scaling Consistency:** If a scaled version exists, ensure the scaling logic is correctly applied and consistent with the relevant scaling factor.

---
