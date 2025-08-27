include "../../types/narrative_stage_enum.mzn";
include "hero_journey_sequence.mzn";
include "stage_milestone.mzn";
include "current_project_stage.mzn";

output [
    "--- Project Narrative Journey ---",
    "Current Project Stage: ", show(current_project_stage), "\n",
    "Associated Milestone: ", stage_milestone[current_project_stage], "\n",


    "\n--- Hero's Journey Sequence ---"
] ++ [
    ( "  Stage " ++ show(i) ++ ": " ++ show(hero_journey_sequence[i]) ++ ": " ++ stage_milestone[hero_journey_sequence[i]] ++ "\n" )
    | i in 1..length(hero_journey_sequence)
];
