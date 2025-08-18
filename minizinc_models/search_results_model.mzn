% Define a MiniZinc record for a single search result
type search_result_t = record (
    file_path: string,
  line_number: int,
  line_content: string
);

% Declare an array of search_result_t to hold all results
array[int] of search_result_t: search_results;

% Declare the pattern that was searched for
string: pattern_searched;

% You can add constraints or solve statements here to process the data
% For example, to print all results:
output [
  "Search Results for pattern: " ++ pattern_searched ++ "\n",
  "--------------------------------------------------\n"
] ++ [
  "  File: " ++ r.file_path ++ "\n" ++
  "  Line: " ++ int2str(r.line_number) ++ "\n" ++
  "  Content: " ++ r.line_content ++ "\n" ++
  "--------------------------------------------------\n"
  | r in search_results
];

