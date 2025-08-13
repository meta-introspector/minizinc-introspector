% Parameters
int: n = 2; % Two points
int: d = 1; % One dimension

% Variables: positions on a 1-bit manifold
array[1..n, 1..d] of var {-1, 1}: p; % Two halves: -1 or 1

% Constraint: points must be in different halves
constraint p[1,1] != p[2,1];

% Objective: Minimize a constant to just find a feasible solution
solve minimize 0;

output [ "Position 1: ", show(p[1,1]), "\n",
         "Position 2: ", show(p[2,1]), "\n" ];


