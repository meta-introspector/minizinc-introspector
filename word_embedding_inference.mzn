include "globals.mzn";

% num_words: Total number of words in the current MiniZinc instance.
% This includes both new words from the current chunk and fixed words from previous chunks.
int: num_words;

% word_map: Maps integer indices to word strings for all words (new and fixed).
array[1..num_words] of string: word_map;

% embeddings: The 8-dimensional embedding vectors for all words.
% These are the variables to be optimized by the solver.
array[1..num_words, 1..8] of var float: embeddings;

% num_fixed_words: Number of words whose embeddings are fixed from previous iterations.
int: num_fixed_words;

% fixed_word_indices: Indices within 'word_map' that correspond to fixed words.
array[1..num_fixed_words] of int: fixed_word_indices;

% fixed_word_embeddings: The fixed 8-dimensional embedding vectors for the fixed words.
array[1..num_fixed_words, 1..8] of float: fixed_word_embeddings;

% num_relations: Number of logical relationships (pairs of words with desired distances).
int: num_relations;

% relation_pairs: Pairs of word indices (from 'word_map') for which relationships are defined.
array[1..num_relations] of tuple(int, int): relation_pairs;

% desired_distances: The target Euclidean distances for each 'relation_pair'.
array[1..num_relations] of float: desired_distances;

% Constraint to fix the embeddings of words from previous iterations.
constraint forall (i in 1..num_fixed_words) (
    forall (j in 1..8) (
        embeddings[fixed_word_indices[i], j] = fixed_word_embeddings[i, j]
    )
);

include "minizinc_models/functions.mzn";
include "minizinc_models/constraints.mzn";

% Calculate the actual Euclidean distances between the embeddings of related word pairs.
array[1..num_relations] of float: actual_distances = [
    euclidean_distance(embeddings[relation_pairs[k].1], embeddings[relation_pairs[k].2])
    | k in 1..num_relations
];

% Objective function: Minimize the sum of squared differences between actual and desired distances.
solve minimize sum(k in 1..num_relations) (pow(actual_distances[k] - desired_distances[k], 2));

include "minizinc_models/solve_output.mzn";
