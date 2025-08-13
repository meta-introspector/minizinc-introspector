include "embedding_params_core_composed_v3.mzn"; % Use new composed parameters
include "embedding_params_kappa_composed.mzn";
include "embedding_params_other_composed.mzn";
include "embedding_params_relations_v3.mzn";
include "embedding_params_vector_composed_v3.mzn";
include "embedding_coeffs_scaled.mzn";
include "embedding_relations_v3.mzn"; % Use simplified relations
include "embedding_variables.mzn";
include "embedding_helpers.mzn";
include "embedding_constraints.mzn";
include "embedding_objective_v3.mzn"; % Use simplified objective

include "embedding_output.mzn";