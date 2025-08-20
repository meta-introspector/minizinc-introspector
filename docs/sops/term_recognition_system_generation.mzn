% MiniZinc Specification: Term Recognition System Generation
% Conceptual Model based on SOP Poem

% --- 1. System Parameters and Configuration ---
int: num_raw_terms; % Total terms in hierarchical_term_index.json
int: max_total_terms; % MAX_TOTAL_TERMS (from term_loader.rs)
int: max_terms_per_chunk; % MAX_TERMS_PER_CHUNK (from chunk_generator.rs)
int: max_terms_per_dfa_file; % MAX_TERMS_PER_FILE (from vocabulary_dfa_generator/src/main.rs)

% --- 2. Data Flow and Entities ---
set of int: TermID = 1..num_raw_terms;
set of int: FilteredTermID; % Subset of TermID after filtering
set of int: ChunkID;
set of int: GeneratedFileID;
set of int: DFAFileID;

% Variables representing properties of terms/chunks/files
array[TermID] of bool: is_junk_term; % True if term is junk
array[FilteredTermID] of string: filtered_term_content; % Actual term strings
array[ChunkID] of int: terms_in_chunk; % Number of terms in each chunk
array[GeneratedFileID] of string: generated_file_name; % Name of generated JSON file
array[DFAFileID] of string: dfa_file_name; % Name of generated DFA .rs file

% --- 3. Constraints (from SOP logic) ---

% Filtering constraint: Filtered terms are a subset of raw terms
constraint forall (t in FilteredTermID) (not is_junk_term[t]);

% Max total terms constraint
constraint card(FilteredTermID) <= max_total_terms;

% Chunking constraints:
% Each chunk's size must respect max_terms_per_chunk
constraint forall (c in ChunkID) (terms_in_chunk[c] <= max_terms_per_chunk);
% Total filtered terms must be distributed among chunks
constraint sum(c in ChunkID)(terms_in_chunk[c]) = card(FilteredTermID);

% DFA file generation constraint:
% Each DFA file's size must respect max_terms_per_dfa_file
% (Conceptual: terms are grouped into DFA files)

% Path configuration (conceptual, as MiniZinc doesn't handle file paths directly)
% string: home_dir_path;
% string: github_root_path;
% string: project_root_path;
% string: hierarchical_term_index_path;

% --- 4. Objectives (Conceptual) ---
% This part is more about what we *want* to achieve, rather than what MiniZinc can directly optimize.
% For example, minimizing memory usage could be conceptualized as minimizing max(terms_in_chunk)
% or minimizing the number of chunks for a given max_terms_per_chunk.

% solve satisfy; % Or solve minimize/maximize if an objective is defined

% --- 5. Output (Conceptual) ---
% output [
%   "Number of filtered terms: ", show(card(FilteredTermID)), "\n",
%   "Number of chunks: ", show(card(ChunkID)), "\n",
%   "Max terms in any chunk: ", show(max(terms_in_chunk)), "\n"
% ];
