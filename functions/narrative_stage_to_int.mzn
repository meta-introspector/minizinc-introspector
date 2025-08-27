function int: narrative_stage_to_int(NarrativeStage: stage) =
    if stage = CallToAdventure then 2
    else if stage = RefusalOfTheCall then 3
    else if stage = MeetingTheMentor then 5
    else if stage = CrossingTheThreshold then 7
    else if stage = TestsAlliesEnemies then 11
    else if stage = ApproachToTheInmostCave then 13
    else if stage = Ordeal then 17
    else if stage = RewardSeizingTheSword then 19
    else if stage = TheRoadBack then 23
    else if stage = Resurrection then 29
    else if stage = ReturnWithTheElixir then 31
    else 0 % Should not happen
    endif endif endif endif endif endif endif endif endif endif endif;