include "globals.mzn";
int: num_words;
array[1..num_words] of string: word_map;
array[1..num_words, 1..8] of var float: embeddings;
int: num_fixed_words;
array[1..num_fixed_words] of int: fixed_word_indices;
array[1..num_fixed_words, 1..8] of float: fixed_word_embeddings;
int: num_relations;
array[1..num_relations] of tuple(int, int): relation_pairs;
array[1..num_relations] of float: desired_distances;

constraint forall (i in 1..num_fixed_words) (
    forall (j in 1..8) (
        embeddings[fixed_word_indices[i], j] = fixed_word_embeddings[i, j]
    )
);
include "minizinc_models/functions.mzn";
include "minizinc_models/constraints.mzn";

array[1..num_relations] of float: actual_distances = [
    euclidean_distance(embeddings[relation_pairs[k].1], embeddings[relation_pairs[k].2])
    | k in 1..num_relations
];

solve minimize sum(k in 1..num_relations) (pow(actual_distances[k] - desired_distances[k], 2));

include "minizinc_models/solve_output.mzn";
