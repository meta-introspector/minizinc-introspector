array[int] of string: raw_strings;

array[int] of var string: extracted_constants;

constraint forall(i in index_set(raw_strings)) (
    extracted_constants[i] = raw_strings[i]
);

solve satisfy;

output [
    "extracted_constants = ", show(extracted_constants), ";\n"
];

