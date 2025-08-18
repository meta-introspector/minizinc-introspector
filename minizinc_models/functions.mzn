% Function to calculate Euclidean distance between two 8D vectors
function var float: euclidean_distance(array[1..8] of var float: vec1, array[1..8] of var float: vec2) = 
    sqrt(sum(i in 1..8) ((vec1[i] - vec2[i]) * (vec1[i] - vec2[i])));

% Helper function to get embedding by word string
function array[1..8] of var float: get_embedding(string: word_str) = 
    let {
        int: current_idx = sum(i in 1..num_words where word_map[i] == word_str) (i);
    } in 
    % if current_idx > 0 then
        embeddings[current_idx, ..]
    % else
    %    fixed_embeddings[fixed_idx, ..]
    % endif;