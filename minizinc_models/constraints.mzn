

% Define the loss function (Mean Squared Error)
var float: loss = sum(r in 1..num_relations) (pow(actual_distances[r] - desired_distances[r], 2));

% Add co-occurrence loss component
% Add co-occurrence loss component
% constraint loss = loss + sum(b in 1..num_bigrams) (int2float(bigram_counts[b]) * euclidean_distance(get_embedding(bigram_pairs[b,1]), get_embedding(bigram_pairs[b,2])));
% constraint loss = loss + sum(t in 1..num_trigrams) (int2float(trigram_counts[t]) * euclidean_distance(get_embedding(trigram_triples[t,1]), get_embedding(trigram_triples[t,2])));