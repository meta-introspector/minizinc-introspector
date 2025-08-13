include "embedding_params.mzn";
include "embedding_coeffs_scaled.mzn";
include "embedding_relations.mzn";
include "embedding_variables.mzn";
include "embedding_helpers.mzn";
include "embedding_constraints.mzn";
include "embedding_objective.mzn";

solve satisfy; % Or solve satisfy; if objective is uncommented

include "embedding_output.mzn";
