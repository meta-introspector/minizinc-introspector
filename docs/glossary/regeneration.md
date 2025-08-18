# Regeneration (Constrained Generation)

"Regeneration," or more precisely "Constrained Generation," in this project refers to the process of using MiniZinc to synthesize or complete structured data, such as code tokens or Abstract Syntax Trees (ASTs), by satisfying a set of logical constraints. This process is often initiated by "masking" certain parts of the desired output, allowing the MiniZinc solver to determine the values for those masked elements.

Key aspects of regeneration include:
*   **Constraint Satisfaction**: The generated output must adhere to all specified rules and relationships defined in the MiniZinc model.
*   **Guided Synthesis**: Unlike unconstrained generation, this process is guided by explicit logical constraints, ensuring the output is not only syntactically correct but also semantically meaningful within the problem domain.
*   **Iterative Refinement**: Complex regeneration tasks may involve multiple MiniZinc solve steps, iteratively refining the output or exploring different possibilities.

This capability is central to the project's vision of self-evolving systems, enabling the system to generate new code, complete partial code, or correct errors based on its internal models and understanding.