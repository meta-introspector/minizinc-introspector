### How will "structured feedback" (e.g., test failures, linter warnings, human reviews) be provided to LLM agents to enable iterative rewriting?

Establishing robust feedback loops is essential for LLM agents to continuously learn, adapt, and refine their code generation capabilities. Structured feedback ensures that the LLM receives precise and actionable information to enable iterative rewriting.

**Mechanisms for Providing Structured Feedback**:

*   **Precise Error Diagnostics**: When LLM-generated code fails (e.g., compilation errors, runtime exceptions, MiniZinc parsing errors), the system provides highly precise and localized error messages. These messages should pinpoint the exact line, character, and nature of the error, rather than generic failure notifications.
    *   **Example**: Instead of "Compilation failed," the feedback might be "Error: Type mismatch at `model.mzn:15:10`: Expected `int`, found `float` for variable `x`."
*   **Test Failures**: Automated testing frameworks (unit, integration, end-to-end, property-based tests) provide detailed reports on test failures. These reports should include:
    *   **Failed Test Case**: The specific input that caused the failure.
    *   **Expected vs. Actual Output**: The discrepancy between the desired and generated output.
    *   **Stack Traces/Execution Logs**: Relevant logs to help the LLM trace the execution path leading to the error.
*   **Linter Warnings and Static Analysis Reports**: Tools like `clippy` for Rust or MiniZinc's own analysis tools provide warnings and suggestions for code quality, style, and potential bugs. These reports are structured to indicate the rule violated, the location, and often a suggestion for correction.
*   **Human Reviews**: Human developers can provide explicit, structured feedback on LLM-generated code. This feedback can be in the form of:
    *   **Code Review Comments**: Specific comments on lines or blocks of code, explaining issues or suggesting improvements.
    *   **Rubrics/Checklists**: Standardized evaluation rubrics or checklists that assess various aspects of the code (e.g., correctness, efficiency, readability, adherence to best practices).
    *   **Corrected Code Examples**: Providing the LLM with the corrected version of its own code, allowing it to learn from the human's solution.
*   **Performance Metrics**: Feedback on performance profiling (e.g., execution time, memory usage, solver statistics) can be provided in a structured format, highlighting bottlenecks or areas for optimization.
*   **Formal Verification Results**: If formal verification tools are used, their output (e.g., proof success/failure, counterexamples) provides highly precise feedback on the logical correctness of the code or model.

**Enabling Iterative Rewriting**:

*   **Structured Input to LLM**: The structured feedback is then fed back to the LLM as part of the prompt for the next iteration. This allows the LLM to parse the feedback programmatically and incorporate it into its reasoning process.
*   **Adaptive Prompts**: The system can dynamically adapt the prompts based on the type and frequency of errors. For example, if an LLM frequently makes type errors, the prompt for subsequent tasks might include more explicit type definitions or examples.
*   **Reinforcement Learning from Human Feedback (RLHF)**: While not explicitly stated, the human review process can be seen as a form of RLHF, where human preferences and corrections are used to fine-tune the LLM's behavior over time.
*   **Self-Correction Mechanisms**: The LLM itself is designed to observe these errors and attempt corrections, often by modifying or replacing parts of the code. This creates a real-time feedback loop, enabling continuous refinement of the AI-generated models.