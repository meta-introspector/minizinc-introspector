% MiniZinc Model for Code Coverage Mapping
% This model represents the current state of C++ code coverage in libminizinc
% as exercised by Rust FFI tests.

% --- Data Inputs (from .dzn file) ---
int: num_cpp_files;
array[1..num_cpp_files] of string: cpp_file_names;

int: num_rust_tests;
array[1..num_rust_tests] of string: rust_test_names;

% Simplified: File-level coverage
array[1..num_cpp_files] of bool: cpp_file_covered;

% Simplified: Which Rust test covers which C++ file
% test_covers_file[k, i] is true if rust_test_names[k] covers cpp_file_names[i]
array[1..num_rust_tests, 1..num_cpp_files] of bool: test_covers_file;

% --- Derived Variables ---

% Total number of covered C++ files
var int: total_covered_cpp_files = sum(i in 1..num_cpp_files)(if cpp_file_covered[i] then 1 else 0 endif);

% Percentage of C++ files covered
var float: percentage_cpp_files_covered = (int2float(total_covered_cpp_files) / int2float(num_cpp_files)) * 100.0;

% --- Constraints ---

% A C++ file is covered if at least one Rust test covers it
constraint forall(i in 1..num_cpp_files)(
    cpp_file_covered[i] <-> exists(k in 1..num_rust_tests)(test_covers_file[k, i])
);

% --- Output ---
solve satisfy;

output [
    "--- Code Coverage Map ---\n",
    "Total C++ Files: ", show(num_cpp_files), "\n",
    "Covered C++ Files: ", show(total_covered_cpp_files), "\n",
    "Percentage C++ Files Covered: ", show_float(3, percentage_cpp_files_covered), "%\n",
    "\n",
    "Covered Files Details:\n"
] +
[
    "  File: ", cpp_file_names[i], " - Covered: ", show(cpp_file_covered[i]), "\n"
    | i in 1..num_cpp_files
] +
[
    "\nRust Test Coverage:\n"
] +
[
    "  Test: ", rust_test_names[k], "\n" ++ 
    "    Covers C++ Files: " ++ show({cpp_file_names[i] | i in 1..num_cpp_files where test_covers_file[k, i]}) ++"\n"
    | k in 1..num_rust_tests
];
