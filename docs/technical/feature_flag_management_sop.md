# Standard Operating Procedure: Feature Flag Management

## 1. Purpose
This SOP outlines the process for effectively utilizing Rust's feature flags to manage conditional compilation within the `libminizinc` project. This approach supports the "Monotonic Epic Idea" by allowing additive inclusion of functionality without altering core logic, enabling flexible builds for different environments, debugging, or experimental features.

## 2. Scope
This SOP applies to all Rust crates within the `libminizinc` project that require conditional compilation of code or dependencies.

## 3. Principles
*   **Additive Development:** Features are added as optional components, not by modifying existing, stable code paths.
*   **Modularity:** Encapsulate specific functionalities or dependencies behind feature flags.
*   **Clarity:** Feature flags should have clear, descriptive names.
*   **Testability:** Ensure that each feature can be independently tested.
*   **Performance:** Use feature flags to optimize builds by excluding unnecessary code or dependencies.

## 4. Procedure

### 4.1. Defining Feature Flags in `Cargo.toml`
1.  **Locate `Cargo.toml`:** Navigate to the `Cargo.toml` of the crate where the feature flag is to be defined.
2.  **Add `[features]` Section:** If not already present, add a `[features]` section.
3.  **Define Feature:** Define the new feature. If the feature enables optional dependencies, list them within the feature definition.
    *   Example for a simple feature:
        ```toml
        [features]
        my_new_feature = []
        ```
    *   Example for a feature enabling optional dependencies:
        ```toml
        [features]
        default = [] # Define a default set of features if applicable
        profile = ["puffin", "puffin_egui", "puffin_http"] # 'profile' feature enables puffin-related crates
        ```
4.  **Declare Optional Dependencies:** For dependencies that are only required when a specific feature is enabled, declare them as `optional = true` in their respective `[dependencies]` or `[dependencies.<crate-name>]` sections.
    *   Example:
        ```toml
        [dependencies.puffin]
        version = "0.19.1"
        optional = true

        [dependencies.puffin_egui]
        version = "0.29.0"
        optional = true

        [dependencies.puffin_http]
        version = "0.19.1"
        optional = true
        ```

### 4.2. Conditional Compilation in Rust Code
1.  **Use `#[cfg(feature = "...")]`:** Apply the `#[cfg(feature = "<feature_name>")]` attribute to code blocks, modules, functions, or even `use` statements that should only be compiled when the feature is enabled.
    *   Example for a function:
        ```rust
        #[cfg(feature = "my_new_feature")]
        fn experimental_function() {
            // ... code for the experimental feature ...
        }
        ```
    *   Example for a code block:
        ```rust
        fn main() {
            // ... common code ...

            #[cfg(feature = "profile")]
            {
                puffin::set_scopes_on(true);
                if let Ok(guard) = puffin_http::start_puffin_server() {
                    eprintln!("Started puffin server on {}", guard.url());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                }
            }
        }
        ```
2.  **Conditional `use` statements:** If a `use` statement depends on a feature, it should also be conditionally compiled.
    *   Example:
        ```rust
        #[cfg(feature = "profile")]
        use puffin;
        #[cfg(feature = "profile")]
        use puffin_http;
        ```

### 4.3. Building with Feature Flags
1.  **Enable a feature:** To build with a specific feature enabled, use the `--features` flag with `cargo build` or `cargo run`.
    *   Example:
        ```bash
        cargo build --features profile
        cargo run --features profile
        ```
2.  **Enable multiple features:** Separate multiple features with a comma.
    *   Example:
        ```bash
        cargo build --features "feature_a,feature_b"
        ```
3.  **Disable default features:** If a crate has default features that you want to disable, use `--no-default-features`.
    *   Example:
        ```bash
        cargo build --no-default-features --features my_new_feature
        ```

## 5. Expected Outcome
*   Flexible and optimized builds tailored to specific needs.
*   Clear separation of concerns within the codebase.
*   Reduced compilation times when optional features are not required.
*   Adherence to the "Monotonic Epic Idea" by enabling additive feature integration.
