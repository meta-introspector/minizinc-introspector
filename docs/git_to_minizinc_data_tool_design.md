## Conceptual Design for `git-to-minizinc-data` Rust Tool

### 1. Purpose
To parse Git repository history and project metrics to generate a `.dzn` data file for the `development_path_optimizer.mzn` MiniZinc model.

### 2. Input
*   Path to a Git repository (e.g., `/data/data/com.termux/files/home/storage/github/libminizinc`).
*   Optional: Start and end commit hashes/dates to limit the analysis scope.
*   Optional: Configuration file (`git_data_extractor.toml`) for heuristics (e.g., how to define a "task," keywords for LLM agent tasks).

### 3. Core Functionality
*   **Git Log Parsing:** Use the `git2` crate to read commit history.
    *   Identify commits as potential "tasks."
    *   Calculate `task_duration` based on commit timestamps (e.g., time between a commit and its parent, or time spent on a branch).
    *   Infer `depends` relationships from commit parentage.
*   **Code Metric Extraction (Conceptual):**
    *   For more advanced models, analyze code changes within commits (e.g., lines added/deleted, file types modified) to estimate complexity or resource costs. This might involve integrating with existing code analysis crates or external tools.
*   **LLM Agent Action Log Integration (Future):**
    *   If LLM agent actions are logged with timestamps and resource usage, this tool could parse those logs to provide more accurate `llm_agent_time_cost` data.
*   **Output Generation:** Format the extracted data into the `development_path_optimizer.dzn` structure.

### 4. Output
*   A `.dzn` file (e.g., `generated_dev_path_data.dzn`) containing `num_tasks`, `task_duration`, `depends`, `llm_agent_time_cost`, and `cpu_time_cost`.

### 5. Technical Considerations (Rust):
*   Use `git2` for Git repository interaction.
*   Use `clap` for CLI arguments.
*   Modular design for different data extraction heuristics.
