array[int] of string: all_extracted_strings;
int: num_strings = length(all_extracted_strings);

% Decision variable: whether each string is needed as a constant
array[1..num_strings] of var bool: is_needed;

% Constraint: For now, all extracted strings are considered needed.
% This can be extended with more complex logic later.
constraint forall(i in 1..num_strings) (
    is_needed[i] = true
);

% The list of selected constants
array[int] of string: selected_constants = [
    all_extracted_strings[i] | i in 1..num_strings where is_needed[i]
];

solve satisfy;

output [
    "selected_constants = ", show(selected_constants), ";\n"
];

