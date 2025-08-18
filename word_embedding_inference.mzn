include "globals.mzn";

% Input data from DZN file for current chunk
int: num_words;
array[1..num_words] of string: word_map;
array[1..num_words, 1..8] of var float: embeddings; % Make embeddings variables to be optimized

% Input data for fixed embeddings from previous chunks
int: fixed_num_words;
array[1..fixed_num_words] of string: fixed_word_map;
array[1..fixed_num_words, 1..8] of float: fixed_embeddings;

% Function to calculate Euclidean distance between two 8D vectors
function var float: euclidean_distance(array[1..8] of var float: vec1, array[1..8] of var float: vec2) = 
    sqrt(sum(i in 1..8) ((vec1[i] - vec2[i]) * (vec1[i] - vec2[i])));

% Helper function to get embedding by word string
function array[1..8] of var float: get_embedding(string: word_str) = 
    let {
        int: current_idx = sum(i in 1..num_words where word_map[i] == word_str) (i);
        int: fixed_idx = sum(i in 1..fixed_num_words where fixed_word_map[i] == word_str) (i);
    } in 
    if current_idx > 0 then
        embeddings[current_idx, ..]
    else
        fixed_embeddings[fixed_idx, ..]
    endif;

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
% Input data for logical relationships
int: num_relations;
array[1..num_relations, 1..2] of string: relation_pairs;
array[1..num_relations] of float: desired_distances;

% Input data for co-occurrence
int: num_bigrams;
array[1..num_bigrams, 1..2] of string: bigram_pairs;
array[1..num_bigrams] of int: bigram_counts;
int: num_trigrams;
array[1..num_trigrams, 1..3] of string: trigram_triples;
array[1..num_trigrams] of int: trigram_counts;

% Calculate actual distances for the defined relationships
array[1..num_relations] of var float: actual_distances;
constraint forall(r in 1..num_relations) (
    actual_distances[r] = euclidean_distance(get_embedding(relation_pairs[r, 1]), get_embedding(relation_pairs[r, 2]))
);

% Define the loss function (Mean Squared Error)
var float: loss = sum(r in 1..num_relations) (pow(actual_distances[r] - desired_distances[r], 2));

% Add co-occurrence loss component
constraint loss = loss + sum(b in 1..num_bigrams) (int2float(bigram_counts[b]) * euclidean_distance(get_embedding(bigram_pairs[b,1]), get_embedding(bigram_pairs[b,2])));
constraint loss = loss + sum(t in 1..num_trigrams) (int2float(trigram_counts[t]) * euclidean_distance(get_embedding(trigram_triples[t,1]), get_embedding(trigram_triples[t,2])));

solve minimize loss;

output [
    "Distance(code, rust) = \(dist_code_rust)\n",
    "Distance(documentation, guide) = \(dist_doc_guide)\n",
    "Distance(build, design) = \(dist_build_design)\n",
    "Loss = \(loss)\n",
    "\n% Optimized Embeddings for this chunk\n",
    "fixed_num_words = " ++ show(num_words) ++ ";\n",
    "fixed_word_map = " ++ show(word_map) ++ ";\n",
    "fixed_embeddings = array2d(1..num_words, 1..8, [\n",
    concat(["        " ++ show(embeddings[i,j]) ++ ",\n" | i in 1..num_words, j in 1..8])
    "]);
