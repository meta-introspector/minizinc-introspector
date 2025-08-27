include "types/narrative_stage_enum.mzn"; % Needed for NarrativeStage
include "minizinc_models/narrative_journey/hero_journey_sequence.mzn";
include "minizinc_models/narrative_journey/stage_milestone.mzn";
include "minizinc_models/narrative_journey/current_stage_constraint.mzn";
include "minizinc_models/narrative_journey/output_narrative.mzn";

NarrativeStage: current_project_stage = CallToAdventure;

solve satisfy;