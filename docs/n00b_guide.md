# Getting Started: A N00b's Guide to libminizinc

Welcome to the `libminizinc` project! This guide is designed for absolute beginners to help you get started with running and understanding the core MiniZinc models in this repository.

## 1. What is this Project About?

At its heart, this project explores how to represent complex ideas and knowledge as numbers, using a special language called MiniZinc. Think of it like translating intricate concepts into a mathematical puzzle that a computer can solve. We're trying to "compress" knowledge and make systems that can evolve themselves!

## 2. Prerequisites: What You Need

Before you begin, make sure you have the following installed:

*   **MiniZinc:** This is the main tool we use. It's a free and open-source constraint modeling language.
    *   **How to Install:** Visit the official MiniZinc website for detailed installation instructions: [https://www.minizinc.org/software.html](https://www.minizinc.org/software.html)
*   **Basic Command-Line Knowledge:** You should be comfortable navigating directories and running basic commands in your terminal (like `cd`, `ls`, `./script.sh`).

## 3. Your First Model Run

The core of this project involves running MiniZinc models. We have a special script that helps you do this, managing different versions of the model and its data.

### Navigating to the Project

First, open your terminal and navigate to the `libminizinc` project directory. If you're reading this guide, you're probably already there!

```bash
cd /data/data/com.termux/files/home/storage/github/libminizinc
```

### Running the Embedding Model Script

We use the `run_embedding_model_v6.sh` script to execute our MiniZinc models. This script takes several "version" arguments to specify which parts of the model and data to use. It also automatically creates a "proof tape" – a record of your run for reproducibility.

Here's the basic command:

```bash
./scripts/run_embedding_model_v6.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
```

For your first run, let's use the example parameters for the `v6` model with `v1` data:

```bash
./scripts/run_embedding_model_v6.sh v6 v1 v1 v1 v1 v1
```

Run this command in your terminal.

### Interpreting the Output

After running the script, you'll see some output directly in your terminal.

*   **Proof Tape Directory:** You'll see a line indicating where the "proof tape" for this run is saved (e.g., `Proof Tape Directory: /data/data/com.termux/files/home/storage/github/libminizinc/proof_tapes/20250813_XXXXXX`). This directory contains all the files used for this specific run, making it fully reproducible.
*   **MiniZinc Command:** The script will print the exact MiniZinc command it's executing. This is useful for advanced debugging.
*   **`--- Head of stdout.log ---` and `--- Head of stderr.log ---`:** These sections will show you the first few lines of the MiniZinc solver's output.
    *   If you see `=====UNSATISFIABLE=====`, it means the model, with the given rules and data, has no solution that satisfies all its conditions. This is a valid outcome in constraint programming!
    *   If you see a solution (e.g., a list of numbers or values), then the model found a way to satisfy all its conditions.
*   **`MiniZinc model run completed.` or `MiniZinc model run failed!`:** This indicates the overall success or failure of the MiniZinc execution. If it failed, check the `stderr.log` for details.

## 4. Basic Debugging: When Things Go Wrong

If your model run fails or gives unexpected results, here's how to start debugging:

*   **Check the Terminal Output:** The `--- Head of stdout.log ---` and `--- Head of stderr.log ---` sections are your first stop. Look for error messages here.
*   **Explore the Proof Tape:** The `proof_tapes/` directory (e.g., `proof_tapes/20250813_XXXXXX`) contains the full `stdout.log` and `stderr.log` files, which will have complete error messages. You can open these files with a text editor.
*   **Common Errors:**
    *   `Error: include error: Cannot open file '...'`: This means a MiniZinc model is trying to include another file that it can't find. Double-check the filename and its location.
    *   `Error: type error: undefined identifier '...'`: This means a variable or parameter is being used without being properly declared in the MiniZinc model.

## 5. Where to Go Next

You've successfully run your first model! Here are some next steps:

*   **Read the `README.md`:** The main `README.md` in the project root has more in-depth information about the project's vision, philosophy, and development guidelines.
*   **Explore the `.mzn` files:** Open some of the MiniZinc model files (e.g., `embedding_sphere_final_v6.mzn`, `embedding_constraints.mzn`) in a text editor to see how the models are defined.
*   **Understand "No Direct Edits":** This project has a strict "add-only, never edit" philosophy. If you plan to contribute, make sure to read the "Development Guidelines" section in the `README.md` carefully.

Happy exploring!
