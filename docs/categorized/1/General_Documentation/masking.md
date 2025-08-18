# Masking

In the context of MiniZinc-based code generation and analysis, "masking" refers to the technique of intentionally leaving certain parts of a structured representation (such as a sequence of tokens or an Abstract Syntax Tree - AST) unspecified or partially constrained. These unspecified parts are represented by MiniZinc variables that the solver is then tasked with determining.

Masking allows for:
*   **Completion**: Filling in missing elements in a partial code snippet or AST.
*   **Prediction**: Inferring the most probable elements based on surrounding context and constraints.
*   **Exploration**: Discovering various valid ways to complete a structure under given conditions.

The MiniZinc solver, guided by the model's constraints, will find values for these masked variables that result in a valid and consistent overall structure.