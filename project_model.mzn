enum ComponentType = {CRATE, DOC, VENDOR, TOOL, OTHER};
enum FileType = {RUSTSOURCE, MARKDOWN, JSON, TOML, SHELLSCRIPT, MZNMODEL, DZNDATA, OTHERFILE};

% Declare variables that will be populated by the .dzn file
array[int] of record(string: path, FileType: file_type, int: associated_regex_idx): all_files;
array[int] of string: all_regex_patterns;
array[int] of record(string: name, ComponentType: component_type, set of int: files): components;

% Optional: Add a solve statement or output for verification
solve satisfy;

output [
    "Project Model:\n",
    "  Files: ", show(all_files), "\n",
    "  Regex Patterns: ", show(all_regex_patterns), "\n",
    "  Components: ", show(components), "\n",
    "\nRust Source Files:\n",
    "  ", show([f.path | f in all_files where f.file_type == RUSTSOURCE]), "\n"
];