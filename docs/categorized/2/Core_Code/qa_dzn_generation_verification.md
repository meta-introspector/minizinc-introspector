# QA Procedure: MiniZinc DZN Data Generation and Verification

## 1. Purpose
This procedure outlines the steps for generating MiniZinc Data (DZN) files programmatically using a MiniZinc model and verifying their correctness. This ensures that generated data files are syntactically valid and can be successfully parsed by the main MiniZinc model.

## 2. Scope
This procedure applies to all MiniZinc models designed to generate `.dzn` data files within the project.

## 3. Procedure

### 3.1. Data Generation Model (`generate_vector_params_v2.mzn`)
The `generate_vector_params_v2.mzn` model is responsible for generating the data. It takes `num_vec` and `base_size` as input and outputs the required arrays in DZN format.

**Key Requirement:** The `output` section of the data generation model MUST NOT include the `----------` separator, as this is a solution separator and not part of the DZN data file syntax.

### 3.2. Generating the DZN File
To generate a DZN file, execute the `minizinc` command with the data generation model and the desired input parameters.

**Command:**
```bash
source /data/data/com.termux/files/home/storage/github/libminizinc/.env && \
${LIBMINIZINC_BUILD_DIR}/minizinc \
  "${MINIZINC_MODELS_DIR}/generate_vector_params_v2.mzn" \
  -D "num_vec=<NUM_VEC_VALUE>;" \
  -D "base_size=<BASE_SIZE_VALUE>;" \
  -o "<OUTPUT_DZN_FILE_PATH>"
```
Replace `<NUM_VEC_VALUE>`, `<BASE_SIZE_VALUE>`, and `<OUTPUT_DZN_FILE_PATH>` with the desired values.

### 3.3. Verifying the Generated DZN File
To verify the generated DZN file, use a simple MiniZinc model that declares the expected variables and attempts to parse the DZN file.

**3.3.1. Verification Model (`parse_vector_params.mzn`)
**The `parse_vector_params.mzn` model declares the variables expected in the generated DZN file.

**Content:**
```minizinc
% File: parse_vector_params.mzn
% Purpose: MiniZinc model to parse and validate vector parameters DZN files.

int: num_vec;

array[1..num_vec] of float: alpha_coeff;
array[1..num_vec] of float: beta_coeff;
array[1..num_vec] of int: m_idx;
array[1..num_vec] of int: n_idx;
array[1..num_vec] of int: t_idx;

solve satisfy; % Just try to parse and satisfy, no complex solving
```

**3.3.2. Running the Verification**
Execute the `minizinc` command with the verification model and the generated DZN file.

**Command:**
```bash
source /data/data/com.termux/files/home/storage/github/libminizinc/.env && \
${LIBMINIZINC_BUILD_DIR}/minizinc \
  "${MINIZINC_MODELS_DIR}/parse_vector_params.mzn" \
  "<GENERATED_DZN_FILE_PATH>" \
  -D "num_vec=<NUM_VEC_VALUE>;"
```
Replace `<GENERATED_DZN_FILE_PATH>` and `<NUM_VEC_VALUE>` with the appropriate values.

## 4. Automation Script (`scripts/qa_dzn_generation_verification.sh`)

An automation script can combine these steps for efficient testing.

**Usage:**
```bash
scripts/qa_dzn_generation_verification.sh <num_vec> <base_size>
```

**Script Logic:**
1.  Parse `num_vec` and `base_size` from command-line arguments.
2.  Define output DZN file path (e.g., `example_vector_params_nv<num_vec>_bs<base_size>.dzn`).
3.  Execute the DZN generation command (Section 3.2).
4.  Check the exit code of the generation command. If it fails, report an error and exit.
5.  Execute the DZN verification command (Section 3.3.2).
6.  Check the exit code of the verification command. If it fails, report an error and exit.
7.  If both steps succeed, report success.

## 5. Records
All generated DZN files and verification logs should be stored in the `proof_tapes/` directory or a dedicated `qa_logs/` directory for future reference and auditing.
