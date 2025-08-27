refactor(tmux_interface): Document prelude and cfg refactoring; debug gemini_eprintln

This commit documents a significant refactoring of the `tmux_interface` crate, focusing on the introduction of a `prelude` module and the strategic re-evaluation of `#[cfg]` attributes on constant declarations to improve code organization and resolve compilation ambiguities.

Additionally, it includes minor debugging adjustments to `gemini_eprintln!` macro's internal logging and a temporary change in `mini-act`'s logging.