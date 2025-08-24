% clamp01.mzn
% Clamps a real number to the [0,1] interval
function int: clamp01_int(x: int) = max(0, min(1, x));