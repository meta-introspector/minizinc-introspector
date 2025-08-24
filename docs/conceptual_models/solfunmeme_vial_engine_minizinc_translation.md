# SOLFUNMEME Vial Engine: Lean4 to MiniZinc Translation

This document outlines the translation of core concepts from the Lean4 sketch of the SOLFUNMEME Vial Engine (from Codeberg issue #231) into MiniZinc. The goal is to integrate these conceptual models into the `libminizinc` project's self-awareness framework, enabling reasoning about memetic dynamics and project evolution using MiniZinc.

## 1. Core Data Structures

The fundamental data structures from the Lean4 sketch have been mapped to their MiniZinc equivalents:

*   **`Meme`**: Represented as `array[int] of float` in MiniZinc, reflecting the Lean4 `List ℝ` where values are clamped to the `[0,1]` interval.
*   **`Provenance`**: Translated into a MiniZinc `record` type, capturing key provenance metrics.
*   **`emojiCodex` and `primeCodex`**: Mapped to MiniZinc `array[int] of string` and `array[int] of int` respectively, preserving their role in prime-mapped trait lattices.

**MiniZinc Snippet (from `solfunmeme_vial_engine.mzn`):**

```minizinc
% A Meme is a list of real numbers in [0,1]
array[int] of float: Meme;

% Provenance structure (simplified for MiniZinc)
record Provenance = {
    int pins, % IPFS pin count
    int gitCommits,
    int gitForks,
    int archiveMirrors,
    int cidAgeDays,
    int contractAgeDays,
    int supplyCap,
    int minted,
    int royaltyBps % 0..10000
};

% Emoji encoding as prime-mapped state
array[int] of string: emojiCodex = [
"🍄", "🌿", "🧬", "🤖", "🌐", "🔍", "🧠", "🔮", "🕉️", "💻",
"🧪", "📊", "🏭", "📜", "🚀", "🪐", "🌌", "🔬", "🖥️", "⚡",
"🧩", "🦾", "🔄", "📝", "📈", "🌟", "🧴", "📖", "🔣", "🗣️"
];

% Prime codex
array[int] of int: primeCodex = [
2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
73, 79, 83, 89, 97, 101, 103, 107, 109, 113
];
```

## 2. Helper Functions

Essential helper functions like `clamp01`, `mean`, and `variance` have been implemented in MiniZinc to support trait calculations.

**MiniZinc Snippet (from `solfunmeme_vial_engine.mzn`):**

```minizinc
% Clamps a real number to the [0,1] interval
function float: clamp01(x: float) = max(0.0, min(1.0, x));

% Computes the mean of a list of real numbers
function float: mean(xs: array[int] of float) = sum(xs) / int2float(card(xs));

% Computes the variance of a list of real numbers (simplified)
function float: variance(xs: array[int] of float) =
    let {
        float_mean = mean(xs)
    } in
    sum(i in index_set(xs)) (pow(xs[i] - float_mean, 2)) / int2float(card(xs));
```

## 3. Trait Modeling

A representative subset of the Lean4 trait functions has been translated into MiniZinc functions. These traits capture different aspects of memetic dynamics, from introspection to provenance.

**Translated Traits (from `solfunmeme_vial_engine.mzn`):**

*   **`traitEb` (Blue Eye: Introspection)**: Models the introspection aspect of a meme.
*   **`traitNE` (Nutrient Exchange Efficiency)**: Represents the efficiency of nutrient exchange, peaking at a balanced state.
*   **`traitSV` (Secretome Vibe)**: Captures the "vibe" of the secretome, peaking when the meme's mean state is around 0.5.
*   **`traitRedundancy` (Provenance trait)**: Reflects the redundancy of provenance, based on IPFS pins and archive mirrors.

**MiniZinc Snippet (from `solfunmeme_vial_engine.mzn`):**

```minizinc
% Blue Eye: Introspection
function float: traitEb(m: Meme) = clamp01(0.4 * (sum(m) / 20.0));

% Nutrient Exchange Efficiency: Peaks at balanced state
function float: traitNE(m: Meme) =
    let {
        float_sum = sum(m),
        float_len = int2float(card(m))
    } in
    if float_len = 0.0 then 0.0
    else (
        if float_sum <= float_len * 0.5 then clamp01(float_sum / float_len)
        else clamp01((float_len - float_sum) / float_len)
    );

% Secretome Vibe: Peaks at mean state ~0.5
function float: traitSV(m: Meme) = clamp01(0.6 * (1.0 - abs(mean(m) - 0.5)));

% Helper for traitRedundancy
function float: sat(x: float) = x / (1.0 + x);

% Provenance trait: Redundancy
function float: traitRedundancy(p: Provenance) = clamp01(sat(int2float(p.pins + p.archiveMirrors) / 8.0));
```

## 4. Example Data (`solfunmeme_vial_engine.dzn`)

An example data file `solfunmeme_vial_engine.dzn` has been created to provide sample `Meme` and `Provenance` data for testing and demonstration purposes.

**MiniZinc Data Snippet (from `solfunmeme_vial_engine.dzn`):**

```minizinc
% Example Meme (List of real numbers in [0,1])
Meme = [0.8, 0.9, 0.6];

% Example Provenance data
Provenance = (
    pins: 6,
    gitCommits: 420,
    gitForks: 21,
    archiveMirrors: 3,
    cidAgeDays: 180,
    contractAgeDays: 365,
    supplyCap: 777,
    minted: 128,
    royaltyBps: 500
);
```

## 5. Future Work

This initial translation lays the groundwork for further integration. Future work could include:

*   Translating more complex traits and the `transformStep` evolution operator into MiniZinc constraints.
*   Developing MiniZinc models to optimize meme evolution based on desired trait profiles.
*   Integrating these models with the project's self-modeling framework to reason about the evolution of the `libminizinc` project itself as a "meme."

This integration represents a significant step towards achieving the project's vision of a quasi-meta computationally self-aware system.
