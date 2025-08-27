# Launchpad Refactoring: From Hardcoded Stages to a Modular Stage System

## Introduction
This narrative documents the refactoring effort undertaken to transform the `launchpad` application from a tightly coupled, hardcoded stage dispatcher into a more modular and extensible system. The primary goal was to improve maintainability, facilitate the addition of new functionalities, and align with the project's principles of structured and composable development.

## Initial Problem Identification
Initially, `launchpad_app.rs` (formerly `launchpad_main.rs`) relied on a large `match` statement to dispatch control based on a `stage_identifier` string. This approach led to several issues:
*   **Tight Coupling:** Any new stage required direct modification of `launchpad_app.rs`, making it a central point of change and increasing its complexity.
*   **Limited Extensibility:** Adding new stages was cumbersome and prone to errors, as it involved modifying a critical, monolithic piece of code.
*   **Lack of Abstraction:** The `launchpad_app` module had direct knowledge of the implementation details of each stage, hindering separation of concerns.

## Design Decisions
To address these issues, a modular stage system was designed based on the following principles:

### The `Stage` Trait
A `Stage` trait was introduced to define a common interface for all operational stages. This trait encapsulates the essential behavior of a stage:
*   `name(&self) -> &str`: Provides a unique identifier for the stage.
*   `run(&self, repo: &Repository, args: &[&str]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>`: Executes the asynchronous logic of the stage, taking a Git repository reference (for context) and command-line arguments.

### Dynamic Stage Registry
Instead of a hardcoded `match` statement, a `HashMap` was implemented in `launchpad_app.rs` to serve as a dynamic stage registry. This registry maps stage names (strings) to boxed `Stage` trait objects. This allows `launchpad_app` to look up and execute stages dynamically at runtime.

### Separation of Concerns
Each stage is now implemented in its own dedicated module within the `crates/launchpad/src/stages/` directory. This promotes a clear separation of concerns, where each module is responsible only for its specific stage's logic.

## Implementation Steps

### 1. Setting up the `stages/` Module
*   A new directory `crates/launchpad/src/stages/` was created.
*   `crates/launchpad/src/stages/stage.rs` was created to define the `Stage` trait.
*   `crates/launchpad/src/stages/mod.rs` was created to declare and re-export the `Stage` trait and individual stage modules.
*   `crates/launchpad/src/main.rs` was updated to declare the new `stages` module and to rename `launchpad_main.rs` to `launchpad_app.rs` for clarity, reflecting its role as the core application logic.

### 2. Implementing `TmuxStage`
`TmuxStage` was the first concrete stage implemented. Its purpose is to directly execute raw `tmux` commands. During its implementation, a critical Rust language feature interaction was encountered:
*   **Challenge: `async fn` in `dyn Trait`:** Rust's `async fn` desugars into an anonymous `Future` type whose size is not known at compile time. This makes it incompatible with `dyn Trait` objects, which require a known size. Attempting to use `async fn` directly in the `Stage` trait's `run` method resulted in a `the trait `Stage` is not dyn compatible` error.
*   **Resolution:** The `run` method's signature was changed to return `Pin<Box<dyn Future<Output = Result<(), String>> + Send>>`. This pattern allows for heap-allocating the `Future` and boxing it, making its size known and enabling `dyn Trait` compatibility. The `Send` bound was added to ensure the `Future` can be safely moved across `async` task boundaries.

### 3. Implementing `TmuxControllerCmdStage`
Recognizing that `tmux_controller` provides a more structured way to interact with `tmux` (rather than raw commands), `TmuxControllerCmdStage` was introduced. This stage acts as a bridge, allowing `launchpad` to execute `tmux_controller`'s subcommands (like `create-layout`) by running `cargo run --package tmux_controller -- <subcommand> [args...]`.

### 4. Refactoring `launchpad_app.rs`
`launchpad_app.rs` was significantly refactored:
*   It now initializes a `HashMap` (`stage_registry`) to store `Box<dyn Stage + Send>` instances.
*   Each concrete stage (e.g., `TmuxStage`, `TmuxControllerCmdStage`) is instantiated and inserted into this registry.
*   The large `match` statement was replaced with a lookup in the `stage_registry`. If a stage is found, its `run` method is called. The fallback logic for `zos-stage-*` binaries was retained for now.

## Adding `CreateLayout` to `tmux_controller`
As part of enhancing `tmux` orchestration, a new `CreateLayout` command was added to the `tmux_controller` crate. This command is designed to create a predefined `tmux` pane layout.
*   **Functionality:**
    *   Aggressively kills all other panes in the current window to ensure a clean slate (`kill-pane -a`).
    *   Creates a three-pane vertical split layout:
        *   **Top Pane (Status):** 2 lines high.
        *   **Middle Pane (Gemini):** Occupies 50% of the remaining height.
        *   **Bottom Pane (Work/Data):** 3 lines high.
    *   Selects the top (status) pane.
*   **Initial Iteration:** The initial implementation used a horizontal split, but was later corrected to a vertical split based on user feedback. The pane sizing has been iteratively refined based on user requirements.

## Lessons Learned
*   **Importance of Modular Design:** The refactoring clearly demonstrated how a modular design (using traits and registries) significantly improves code organization, testability, and extensibility compared to monolithic `match` statements.
*   **Challenges with `async fn` in `dyn Trait`:** This common Rust pattern requires careful handling with `Pin<Box<dyn Future + Send>>` to ensure `dyn` compatibility.
*   **Value of `tmux_controller`:** The `tmux_controller` crate provides a robust, pure-Rust abstraction over raw `tmux` commands, making complex `tmux` orchestrations more manageable and less error-prone.
*   **Iterative Development and Refactoring:** This entire process highlighted the iterative nature of software development, where initial implementations are refined and refactored as requirements and understanding evolve.

## Future Work
*   Implement other stages (e.g., `InstallGeminiStage`, `RunGeminiStage`, `DumTestStage`) as `Stage` trait implementations.
*   Further refine `tmux_controller` with more layout options or advanced session management features.
*   Integrate the `zos-stage-*` binary execution into a proper `Stage` implementation.
