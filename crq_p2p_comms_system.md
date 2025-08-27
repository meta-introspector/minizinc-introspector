# Change Request: Virtual P2P Communication and Command System

## 1. Purpose
This Change Request (CRQ) proposes the design and implementation of a robust, virtual peer-to-peer (P2P) communication and command system. The primary goal is to establish a reliable, auditable, and flexible mechanism for inter-agent communication, particularly for orchestrating tasks and sharing public process data among various workers (e.g., Gemini instances, `gitcrq`, `tmux_controller`). This system aims to overcome limitations of direct command sending (like quoting issues) and enable more complex, stateful interactions.

## 2. Scope
The system will encompass:
*   **Command and Data Exchange:** A mechanism for agents to write and read commands, configurations, and public process data.
*   **Shared State:** A conceptual "gossip directory" or shared filesystem-based state for agents to discover and interact with each other's public data.
*   **Worker Coordination:** Facilitate the orchestration and monitoring of tasks executed by different workers.
*   **Auditing and Traceability:** Ensure all communications and state changes are auditable and traceable.

## 3. Key Features / Requirements

### 3.1. Reliable Command Delivery
*   **File-based Commands:** Commands will be written to dedicated files within a designated "comms directory" to avoid shell quoting issues and provide persistence.
*   **Command Acknowledgment:** Mechanisms for workers to acknowledge receipt and completion of commands.

### 3.2. Virtual P2P Gossip Directory
*   **Shared Filesystem:** Utilize a shared directory structure (e.g., `comms/`) where each worker can publish its "public proc data" (e.g., status, capabilities, current task).
*   **Discovery:** Agents can discover other active workers and their published data by monitoring this directory.
*   **Event-driven (Polling/Watchers):** Workers can either poll the directory for new commands/data or utilize filesystem watchers for real-time updates.

### 3.3. Public Process Data (Proc Data)
*   **Standardized Format:** Define a standardized format (e.g., JSON, YAML, MiniZinc data) for public process data published by workers.
*   **Worker Status:** Workers will publish their current status (e.g., idle, busy, error, running specific task).
*   **Task Progress:** Workers can update their proc data to reflect the progress of ongoing tasks.

### 3.4. Composability and Extensibility
*   **Modular Design:** The system should be modular, allowing new types of commands and proc data to be easily integrated.
*   **Tool Agnostic:** Designed to work with various tools and agents (Gemini, `gitcrq`, `tmux_controller`, `asciinema`, LLMs, etc.).

## 4. Potential Technologies / Approach
*   **Core Language:** Rust (for performance, safety, and filesystem interaction).
*   **Filesystem Operations:** Standard Rust `std::fs` for file creation, reading, and writing.
*   **Serialization/Deserialization:** `serde` for handling structured data (JSON/YAML).
*   **Filesystem Watchers:** Crates like `notify` for event-driven updates (if real-time responsiveness is critical).
*   **Concurrency:** Asynchronous Rust for handling multiple workers and non-blocking I/O.
*   **MiniZinc:** Potentially use MiniZinc models to define the state and rules for the gossip directory.

## 5. Relationship to Existing CRQs/SOPs
This system will serve as a foundational communication layer for the "Generic Git and CRQ Management Tool" (CRQ: `crq_generic_git_tool.md`). It will enable more reliable and complex interactions between components of that tool and other agents in the ecosystem.
*   This system will be a critical enabler for the "Launchpad Dynamic Stage Loading from File" (`crq_launchpad_dynamic_stages.md`), providing the communication channel for dynamic stage definitions.
*   This system will be a critical enabler for the "Tmux Watch Workflow for Real-time Monitoring and Logging" (`crq_tmux_watch_workflow.md`), providing the communication channel for streaming and logging.

## 6. Next Steps
*   Detailed design of the directory structure and data formats.
*   Proof-of-concept implementation for basic command exchange.
*   Integration with existing tools.


