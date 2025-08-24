include "thought_type_enum.mzn";

% Represents a single thought
type Thought = record(
    int: id,
    string: timestamp,
    ThoughtType: kind,
    string: content,
    string: related_project_concept,
    int: godel_number
);
