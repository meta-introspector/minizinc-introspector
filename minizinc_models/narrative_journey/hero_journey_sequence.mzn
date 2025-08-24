include "../../types/narrative_stage_enum.mzn";

% Define the sequence of narrative stages
array[1..11] of NarrativeStage: hero_journey_sequence = [
    CallToAdventure,
    RefusalOfTheCall,
    MeetingTheMentor,
    CrossingTheThreshold,
    TestsAlliesEnemies,
    ApproachToTheInmostCave,
    Ordeal,
    RewardSeizingTheSword,
    TheRoadBack,
    Resurrection,
    ReturnWithTheElixir
];