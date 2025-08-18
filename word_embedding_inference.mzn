include "globals.mzn";

% Input data from DZN file
int: num_words;
array[1..num_words] of string: word_map;
array[1..num_words, 1..8] of float: embeddings;

% Function to calculate Euclidean distance between two 8D vectors
function float: euclidean_distance(array[1..8] of float: vec1, array[1..8] of float: vec2) = 
    sqrt(sum(i in 1..8) ((vec1[i] - vec2[i]) * (vec1[i] - vec2[i])));

% Helper function to get embedding by word string
function array[1..8] of float: get_embedding(string: word_str) = 
    let { int: idx = sum(i in 1..num_words where word_map[i] == word_str) (i) } in 
    embeddings[idx, ..]; % Use '..' to select the entire row

% Select some word pairs and calculate their distances
% We need to ensure these words exist in our 'word_map'

% Example 1: Distance between 'code' and 'rust'
var float: dist_code_rust;
constraint dist_code_rust = euclidean_distance(get_embedding("code"), get_embedding("rust"));

% Example 2: Distance between 'documentation' and 'guide'
var float: dist_doc_guide;
constraint dist_doc_guide = euclidean_distance(get_embedding("documentation"), get_embedding("guide"));

% Example 3: Distance between 'build' and 'design'
var float: dist_build_design;
constraint dist_build_design = euclidean_distance(get_embedding("build"), get_embedding("design"));

% Output the calculated distances
solve satisfy;

output [
    "Distance(code, rust) = \(dist_code_rust)\n",
    "Distance(documentation, guide) = \(dist_doc_guide)\n",
    "Distance(build, design) = \(dist_build_design)\n"
];