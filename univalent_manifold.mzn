include "univalent_params.mzn";

% Parameters
int: n = 1; % One point
int: d = 8; % Eight dimensions

% Variables: position as an index into the reciprocals array
array[1..n, 1..d] of var 1..8: p_idx; % Index into primes/reciprocals array

var int: fudge_factor;

% Get the actual value from the reciprocals array
function var int: get_univalent_value(var int: idx) = scaled_reciprocals[idx];

% Constraint: sum of reciprocals for the first point must be 1.0
constraint sum(k in 1..d) (get_univalent_value(p_idx[1,k])) + fudge_factor = RECIPROCAL_SCALE;

% Objective: Minimize the absolute value of the fudge factor
solve minimize abs(fudge_factor);

output [ "Position (indices): ", show(p_idx[1,..]), "\n",         "Position (scaled values): ", show([get_univalent_value(p_idx[1,k]) | k in 1..d]), "\n",         "Position (actual values): ", show([get_univalent_value(p_idx[1,k]) / int2float(RECIPROCAL_SCALE) | k in 1..d]), "\n",         "Fudge Factor: ", show(fudge_factor), "\n" ];




