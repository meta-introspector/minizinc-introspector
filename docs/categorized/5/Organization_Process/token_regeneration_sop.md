# SOP: Token/AST Regeneration using MiniZinc for N00b LLMs

## 1. Purpose

This Standard Operating Procedure (SOP) guides a N00b LLM through the process of understanding and reconstructing how MiniZinc can be used to "regenerate" or "solve for" masked tokens or Abstract Syntax Tree (AST) values within a structured sequence. This technique is fundamental to tasks like code completion, error correction, or constrained generation.

## 2. Prerequisites

To follow this SOP, you should have a basic understanding of:
*   **MiniZinc**: Its syntax, variables, constraints, and `solve` statements.
*   **Rust**: The project's primary language, especially how `zos-bootstrap` is structured.
*   **Shell Commands**: Basic commands like `find`, `ls`, `cat`, `export`.

## 3. Core Concepts

### 3.1. Token/AST Representation in MiniZinc

Tokens or AST nodes are represented as variables within a MiniZinc model. These variables can be of various types (e.g., `enum` for discrete tokens, `int` for node IDs, `array` for sequences or tree structures).

**Example (from `token_regeneration_demo.mzn`):**
```minizinc
enum TokenType = {A, B, C, MASKED};
array[1..seq_len] of var TokenType: token_sequence;
```
Here, `token_sequence` is an array of variables, where each element can take a value from the `TokenType` enum.

### 3.2. Masking

"Masking" a token or AST node means intentionally leaving its corresponding MiniZinc variable unassigned or partially constrained. This signals to the MiniZinc solver that it needs to determine the value for this variable based on other constraints in the model.

**Example (from `token_regeneration_demo.mzn`):**
```minizinc
constraint token_sequence[2] = MASKED; % Mask the second token
```
This constraint explicitly sets the second token to `MASKED`, indicating it's a placeholder to be filled.

### 3.3. Constraints

Constraints are the rules that guide the regeneration process. They define the valid relationships between tokens/nodes, ensuring that the regenerated output is logically consistent and adheres to desired patterns (e.g., grammatical rules for code, structural integrity for ASTs).

**Examples (from `token_regeneration_demo.mzn`):**
```minizinc
constraint token_sequence[1] = A; % The first token must be A
constraint token_sequence[1] = A -> token_sequence[3] != C; % Conditional constraint
constraint token_sequence[2] != A; % Further constraint on the masked token
```

### 3.4. Solving

MiniZinc's role is to find an assignment of values to all variables (including masked ones) that satisfies all the defined constraints. The `solve satisfy;` statement instructs MiniZinc to find any valid solution. For generative tasks, this solution represents the "regenerated" output.

## 4. Project-Specific Environment

### 4.1. `zos-bootstrap`

The `zos-bootstrap` Rust crate is the primary interface for interacting with MiniZinc models within this project. It provides subcommands like `run` (for executing models) and `ast_to_minizinc` (for converting Rust ASTs). While `zos-bootstrap` orchestrates the MiniZinc runs, it ultimately invokes the standalone `minizinc` executable.

### 4.2. `minizinc` Executable Location

The `minizinc` executable is built as part of the project's C++ components. It is typically located in the `build/` directory after a successful `cmake --build build/` command.

**How to find it:**
```bash
find /data/data/com.termux/files/home/storage/github/libminizinc/build -name "minizinc*" -type f -executable
```
This command searches for executable files named "minizinc" (or starting with "minizinc") within the project's `build` directory.

### 4.3. `LD_LIBRARY_PATH`

The `minizinc` executable relies on shared libraries (e.g., `libmzn.so`). If these libraries are not in a standard system path, you must inform the executable where to find them by setting the `LD_LIBRARY_PATH` environment variable. In this project, these libraries are also typically found in the `build/` directory.

**How to find `libmzn.so`:**
```bash
find /data/data/com.termux/files/home/storage/github/libminizinc/build -name "libmzn.so*" -type f
```
This command searches for files named "libmzn.so" (or starting with "libmzn.so") within the project's `build` directory.

## 5. Step-by-Step Demonstration (Reconstruction)

Follow these steps to reconstruct and run the token regeneration demonstration:

### Step 1: Create the MiniZinc Model

Create a new file named `token_regeneration_demo.mzn` in the `/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_models/` directory with the following content:

```minizinc
% MiniZinc model for Token/AST Regeneration Demonstration

% Define a simple set of possible "tokens" or "AST node types"
enum TokenType = {A, B, C, MASKED};

% Define the sequence length
int: seq_len = 3;

% The sequence of tokens/AST nodes
array[1..seq_len] of var TokenType: token_sequence;

% Introduce a "masked" token at a specific position
% We want MiniZinc to determine the value of this masked token
constraint token_sequence[2] = MASKED; % Mask the second token

% Add some constraints to guide the regeneration
% Constraint 1: The first token must be A
constraint token_sequence[1] = A;

% Constraint 2: If the first token is A, the third token cannot be C
constraint token_sequence[1] = A -> token_sequence[3] != C;

% Constraint 3: The masked token (token_sequence[2]) cannot be A
constraint token_sequence[2] != A;

% Solve for the token sequence
solve satisfy;

% Output the regenerated sequence
output ["Regenerated Sequence: " ++ show(token_sequence) ++ "\n"];
```

### Step 2: Locate MiniZinc Executable and Libraries

Before running the model, ensure you know the absolute paths to the `minizinc` executable and its shared libraries. Use the `find` commands from Section 4.2 and 4.3.

**Example Output (your paths may vary):**
*   `minizinc` executable: `/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc`
*   `libmzn.so`: `/data/data/com.termux/files/home/storage/github/libminizinc/build/libmzn.so`

### Step 3: Run the Model

Execute the MiniZinc model using the located executable and setting the `LD_LIBRARY_PATH`. Replace the paths below with your actual paths found in Step 2.

```bash
LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build /data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_models/token_regeneration_demo.mzn
```

### Step 4: Interpret Results

The output will show the "regenerated" sequence.

**Expected Output:**
```
Regenerated Sequence: [A, MASKED, A]
----------
```
This output demonstrates that MiniZinc successfully found a valid assignment for the `token_sequence` that satisfies all the defined constraints, including the masked position.

## 6. Further Exploration

*   **`lambda_embedding_v1.mzn` and `lambda_variable_embedding_v1.mzn`**: Explore these models in `minizinc_models/` to see how more complex AST structures (lambda terms) are represented and constrained.
*   **`zos-bootstrap` `AstToMiniZinc` command**: Investigate `crates/zos-bootstrap/src/commands/ast_to_minizinc.rs` to understand how Rust ASTs are converted into MiniZinc models and data, which is the inverse of the regeneration process.
*   **More Complex Masking**: Experiment with masking multiple tokens/nodes, or adding more intricate constraints to guide the regeneration.
