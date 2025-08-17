array[int] of string: all_extracted_strings;
int: num_strings = length(all_extracted_strings);

% Input parameter: indices of strings that are manually considered needed
set of int: manual_needed_indices;

% Decision variable: whether each string is needed as a constant
array[1..num_strings] of var bool: is_needed;

% Constraint: A string is needed if its index is in manual_needed_indices
constraint forall(i in 1..num_strings) (
    is_needed[i] = (i in manual_needed_indices)
);

% The list of selected constants
array[int] of string: selected_constants = [
    all_extracted_strings[i] | i in 1..num_strings where is_needed[i]
];

solve satisfy;

output [
    "selected_constants = ", show(selected_constants), ";\n"
];


