# SOP: Gemini Batch Interface for Multi-Turn Conversations

This Standard Operating Procedure (SOP) describes the design of a batch interface for interacting with the Gemini CLI agent in multi-turn conversations. This interface enables an external orchestrator (e.g., a Rust program) to provide Gemini with a batch of instructions or queries and to poll for structured responses, facilitating automated OODA loop engagement and complex task execution.

## 1. Purpose

To define a standardized, machine-readable interface for batch communication with the Gemini CLI agent, enabling efficient execution of multiple tasks, structured data exchange, and integration into automated workflows for the `libminizinc` project.

## 2. Scope

This SOP applies to any external system or program that intends to interact with the Gemini CLI agent in a programmatic, batch-oriented manner for tasks related to code analysis, test generation, documentation, and project introspection.

## 3. Interface Design: JSON-Based Communication

The batch interface utilizes JSON for both input and output, ensuring structured and easily parsable data exchange.

### 3.1. Input Format (to Gemini)

Input to Gemini is a JSON array of "tasks." Each task is an object with a unique `task_id`, a `type` specifying the operation, and a `parameters` object containing task-specific arguments.

```json
[
  {
    "task_id": "task_123",
    "type": "query_coverage",
    "parameters": {
      "scope": "all",
      "format": "minizinc_dzn"
    }
  },
  {
    "task_id": "task_124",
    "type": "propose_test",
    "parameters": {
      "target_cpp_file": "lib/parser.cpp",
      "target_line_range": "100-150",
      "semantic_context": "parsing of MiniZinc variables"
    }
  },
  {
    "task_id": "task_125",
    "type": "get_memory_summary",
    "parameters": {
      "keywords": ["OODA", "oxidation"]
    }
  },
  {
    "task_id": "task_126",
    "type": "execute_shell_command",
    "parameters": {
      "command": "ls -la",
      "description": "List files in current directory"
    }
  }
]
```

**Task Types and Parameters (Examples)**:

*   `query_coverage`:
    *   `scope`: `"all"` (for combined C++/Rust), `"cpp"`, `"rust"`
    *   `format`: `"minizinc_dzn"`, `"json"`, `"text_summary"`
*   `propose_test`:
    *   `target_cpp_file`: Path to the C++ file to target.
    *   `target_line_range`: Specific line range (e.g., `"100-150"`) or `"uncovered"`.
    *   `semantic_context`: Natural language description of the semantic area to test.
*   `get_memory_summary`:
    *   `keywords`: Array of strings to filter memory entries.
*   `execute_shell_command`:
    *   `command`: The shell command to execute.
    *   `description`: A brief description of the command's purpose.

### 3.2. Output Format (from Gemini - for Polling)

Output from Gemini is a JSON object that provides the status and results of the processed tasks. This format is designed for polling by an external orchestrator.

```json
{
  "response_id": "batch_response_456",
  "status": "completed", // "pending", "completed", "failed"
  "results": [
    {
      "task_id": "task_123",
      "status": "completed",
      "output": {
        "coverage_data": "..." // MiniZinc DZN content or other requested format
      },
      "error": null // null if no error, otherwise error message
    },
    {
      "task_id": "task_124",
      "status": "pending", // Task might take longer to process
      "output": {},
      "error": null
    },
    {
      "task_id": "task_125",
      "status": "completed",
      "output": {
        "memory_summary": "..."
      },
      "error": null
    },
    {
      "task_id": "task_126",
      "status": "completed",
      "output": {
        "stdout": "...",
        "stderr": "...",
        "exit_code": 0
      },
      "error": null
    }
  ],
  "next_poll_interval_seconds": 5 // Suggested interval for the orchestrator to poll again
}
```

**Output Statuses**:

*   `pending`: The task is being processed and results are not yet available.
*   `completed`: The task has finished successfully.
*   `failed`: The task encountered an error during processing. The `error` field will contain details.

## 4. Orchestrator Interaction Model

An external orchestrator (e.g., a Rust program) will interact with Gemini using the following model:

1.  **Send Batch Request**: The orchestrator sends a single JSON input (as described in 3.1) to Gemini.
2.  **Receive Initial Response**: Gemini immediately returns a JSON output (as described in 3.2) with the initial status of tasks (likely `pending` for most).
3.  **Poll for Updates**: The orchestrator periodically polls Gemini (using the `next_poll_interval_seconds` as a hint) with the `response_id` from the initial response.
4.  **Process Results**: Once a task's `status` changes to `completed` or `failed`, the orchestrator processes its `output` or `error` field.
5.  **Multi-Turn Continuation**: For tasks that require multi-turn interaction (e.g., `propose_test` might lead to a `refine_test` task), the orchestrator can send new batch requests based on the results of previous tasks.

## 5. OODA Loop Engagement

This batch interface facilitates Gemini's role within the project's OODA (Observe, Orient, Decide, Act) loop:

*   **Observe**: The orchestrator collects data (e.g., C++/Rust coverage reports) and sends `query_coverage` tasks to Gemini.
*   **Orient**: Gemini processes the coverage data, analyzes it, and potentially uses its semantic understanding to identify gaps or propose strategies. This might involve `propose_strategy` tasks (a future task type).
*   **Decide**: The orchestrator (or a human operator) reviews Gemini's proposed strategies or test cases (e.g., from `propose_test` tasks) and decides on the next action.
*   **Act**: The orchestrator executes the chosen actions (e.g., runs tests, generates new tests based on Gemini's proposals), and feeds the new data back into the "Observe" phase.

This structured interaction model allows for a more automated and intelligent feedback loop, moving towards a truly self-aware system.