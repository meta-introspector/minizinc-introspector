include "thought_type.mzn";
include "narrative_stage_enum.mzn";

% Represents the Gemini agent
type GeminiAgent = record(
    string: name,
    string: purpose,
    array[int] of string: available_tools,
    array[int] of Thought: thoughts_log, % A log of Gemini's thoughts
    NarrativeStage: current_narrative_stage % Current stage of the Hero's Journey
);

