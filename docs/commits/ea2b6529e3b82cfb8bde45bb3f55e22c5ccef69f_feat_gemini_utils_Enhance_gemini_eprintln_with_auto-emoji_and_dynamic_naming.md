feat(gemini_utils): Enhance gemini_eprintln with auto-emoji and dynamic naming
Implement automatic population of emoji variables and dynamic emoji naming in `gemini_eprintln!`.

- **Auto-emoji population**: If an emoji character is used in the format string without a corresponding named argument, the macro now automatically generates a named argument for it using its canonical name (e.g., `sparkles` for `✨`).
- **Dynamic emoji naming**: Supports `::some-emoji::` placeholders where `some_emoji` is a variable whose string literal value (e.g., `":rocket:"`) is interpreted as the actual emoji to be used.

Refactor `gemini_utils_test` to split tests into separate files for better organization and maintainability.