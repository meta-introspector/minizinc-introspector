% Data declarations for word embeddings, logical relationships, and co-occurrence
int: num_words;
array[1..num_words] of string: word_map;
array[1..num_words, 1..8] of var float: embeddings; % Make embeddings variables to be optimized

int: num_relations;
array[1..num_relations, 1..2] of string: relation_pairs;
array[1..num_relations] of float: desired_distances;

int: num_bigrams;
array[1..num_bigrams, 1..2] of string: bigram_pairs;
array[1..num_bigrams] of int: bigram_counts;
int: num_trigrams;
array[1..num_trigrams, 1..3] of string: trigram_triples;
array[1..num_trigrams] of int: trigram_counts;