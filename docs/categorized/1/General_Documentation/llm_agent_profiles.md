### What are the defined "profiles" for LLM agents, and how do they influence agent behavior and task execution?

LLM agents are "onboarded" by defining their precise role, for example, as an **"expert MiniZinc developer"** or a "programming assistant," within the system prompt. This guides the LLM on the type of output and focus required for the task.

**Influence on Agent Behavior and Task Execution**:

*   **Role Definition (System Prompts)**: This is the primary mechanism for defining an LLM agent's "profile." By setting the system prompt, the LLM is guided on the type of output and focus required for the task. For instance, an "expert MiniZinc developer" profile would lead the LLM to generate MiniZinc code, adhere to MiniZinc best practices, and focus on constraint programming concepts.
*   **Contextual Data Provision**: The profile also implicitly influences how the LLM processes and utilizes contextual data provided in prompts. An agent profiled as a "programming assistant" might prioritize code generation, while one focused on "semantic resonance mapping" might emphasize extracting and structuring conceptual relationships.
*   **Prompt Engineering Strategies**: The choice of prompt engineering strategies (e.g., Chain-of-Thought, Decomposition-based, Grammar Prompting) can be tailored to the agent's profile to optimize its performance for specific tasks.
*   **Iterative Refinement**: The feedback loops and iterative refinement process are also influenced by the agent's profile. An agent focused on code generation will receive and process feedback related to syntax, correctness, and efficiency of the generated code, while an agent focused on documentation might receive feedback on clarity, completeness, and adherence to documentation standards.

In essence, the "profile" acts as a guiding persona and set of constraints that shape the LLM agent's interpretation of tasks, its reasoning process, and the nature of its output, ensuring alignment with the project's specialized domains and objectives.