# How to Run the Project Model and Regex Analysis System

This guide outlines the steps to generate file path regexes, analyze them with the Rust `regex_file_analyzer` crate, and then run the MiniZinc project model with the generated data.

## Prerequisites

*   Rust toolchain installed.
*   MiniZinc installed and built. Ensure the `minizinc` executable is available and properly configured.
*   The `libminizinc` project cloned and built.

## Step 1: Generate File Path Regexes

First, you need to generate the `generated_path_regexes.json` file, which contains individual regexes for each file path in your project. This is done using the `regex_generator_tool`.

```bash
cargo run --bin regex_generator_tool > generated_path_regexes.json
```

This command will output the JSON content directly to the specified file.

## Step 2: Analyze Regexes and Generate MiniZinc Data

Next, use the `regex_file_analyzer` crate to process the `generated_path_regexes.json` file and generate the `project_model_data.dzn` file. This `dzn` file will contain the data for your MiniZinc model.

Navigate to the `crates/regex_file_analyzer` directory and run the tool:

```bash
cd crates/regex_file_analyzer
cargo run --release > ../../project_model_data.dzn
cd ../..
```

This command compiles and runs the `regex_file_analyzer`, directing its output (the MiniZinc data) to `project_model_data.dzn` in the project root.

## Step 3: Run the MiniZinc Project Model

Finally, run the MiniZinc model (`project_model.mzn`) with the generated data (`project_model_data.dzn`). You need to ensure that the `LD_LIBRARY_PATH` environment variable is correctly set to the MiniZinc build directory.

```bash
export LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build
/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc \
    project_model.mzn \
    project_model_data.dzn
```

This command executes the MiniZinc model, using the data generated in the previous step. The output will be the solution to your MiniZinc model, including queries like the list of Rust source files.
