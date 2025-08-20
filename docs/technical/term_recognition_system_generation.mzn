% MiniZinc Specification: Term Recognition System Generation ✨
% Conceptual Model based on SOP Poem 📜 and Quality Principles 📈

% This model serves as a declarative specification for the term recognition system,
aligning with ISO 9000 (Quality Management 🎯), GMP (Good Manufacturing Practices 🏭),
Six Sigma (Process Optimization ⚙️), and ITIL (Service Management 🤝) principles.
% It defines the system's parameters, entities, and constraints, ensuring a controlled
% and predictable generation process.

% --- 1. System Parameters and Configuration 📊 ---
int: num_raw_terms; % Total terms in hierarchical_term_index.json 🌳
int: max_total_terms; % MAX_TOTAL_TERMS (from term_loader.rs) - Process Control Limit 🛑
int: max_terms_per_chunk; % MAX_TERMS_PER_CHUNK (from chunk_generator.rs) - Memory Efficiency Constraint 🧠
int: max_terms_per_dfa_file; % MAX_TERMS_PER_FILE (from vocabulary_dfa_generator/src/main.rs) - Modularity Constraint 🧩

% --- 2. Data Flow and Entities ➡️ ---
set of int: TermID = 1..num_raw_terms; % Universe of potential terms 🌌
set of int: FilteredTermID; % Subset of TermID after filtering - Quality Gate ✅
set of int: ChunkID; % Identifiers for generated term data chunks 📦
set of int: GeneratedFileID; % Identifiers for generated JSON files 📄
set of int: DFAFileID; % Identifiers for generated DFA .rs files 🤖

% Variables representing properties of terms/chunks/files
array[TermID] of bool: is_junk_term; % True if term is junk - Defect Identification 🐛
array[FilteredTermID] of string: filtered_term_content; % Actual term strings - Data Integrity 🛡️
array[ChunkID] of int: terms_in_chunk; % Number of terms in each chunk - Resource Allocation ⚖️
array[GeneratedFileID] of string: generated_file_name; % Name of generated JSON file - Traceability 🔗
array[DFAFileID] of string: dfa_file_name; % Name of generated DFA .rs file - Functional Output 🚀

% --- 3. Constraints (from SOP logic) 🔒 ---

% Filtering constraint: Filtered terms are a subset of raw terms
% Ensures only valid, non-junk terms proceed, adhering to quality standards.
constraint forall (t in FilteredTermID) (not is_junk_term[t]);

% Max total terms constraint: Prevents system overload and ensures scalability.
constraint card(FilteredTermID) <= max_total_terms;

% Chunking constraints: Optimizes memory usage and improves processing efficiency.
% Each chunk's size must respect max_terms_per_chunk (Six Sigma: process variation control)
constraint forall (c in ChunkID) (terms_in_chunk[c] <= max_terms_per_chunk);
% Total filtered terms must be distributed among chunks (ITIL: service delivery consistency)
constraint sum(c in ChunkID)(terms_in_chunk[c]) = card(FilteredTermID);

% DFA file generation constraint: Ensures modularity and manageability of generated code.
% Each DFA file's size must respect max_terms_per_dfa_file
% (Conceptual: terms are grouped into DFA files for efficient compilation and loading)

% Path configuration (conceptual, as MiniZinc doesn't handle file paths directly)
% These are external inputs to the system, defining the operational environment.
% string: home_dir_path; 🏠
% string: github_root_path; 🐙
% string: project_root_path; 📁
% string: hierarchical_term_index_path; 📜

% --- 4. Objectives (Conceptual) 🎯 ---
% This section outlines desired outcomes and optimization goals, guiding continuous improvement.
% For example, minimizing memory usage could be conceptualized as minimizing max(terms_in_chunk)
% or minimizing the number of chunks for a given max_terms_per_chunk.
% Other objectives could include:
% - Minimize generation time ⏱️
% - Maximize code reusability (by optimizing chunking for common patterns) ♻️
% - Ensure traceability from raw term to generated file (ISO 9000) 🔍

% solve satisfy; % Or solve minimize/maximize if an objective is defined

% --- 5. Output (Conceptual) 📈 ---
% Defines key metrics for monitoring and quality assurance.
% output [
%   "Number of filtered terms: ", show(card(FilteredTermID)), "\n", % Process Yield
%   "Number of chunks: ", show(card(ChunkID)), "\n", % Modularity Metric
%   "Max terms in any chunk: ", show(max(terms_in_chunk)), "\n" % Peak Memory Load
% ];