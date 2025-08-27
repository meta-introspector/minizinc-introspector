# C4 Model for Launchpad and Tmux Controller

This document provides a C4 model overview of the `launchpad` and `tmux_controller` components, illustrating their context, containers, and key components.

## 1. System Context Diagram

The System Context diagram shows the `launchpad` and `tmux_controller` systems in their environment, interacting with users and other external systems.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

Person(user, "Developer/User", "Operates the CLI tools")

System(launchpad, "Launchpad CLI", "Orchestrates Gemini CLI execution and development workflows")
System(tmux_controller, "Tmux Controller CLI", "Manages tmux sessions and windows programmatically")
System(gemini_cli, "Gemini CLI", "The core Gemini command-line interface")
System(tmux, "Tmux", "Terminal multiplexer")
System(asciinema, "Asciinema", "Terminal session recorder")

Rel(user, launchpad, "Uses")
Rel(user, tmux_controller, "Uses")
Rel(launchpad, gemini_cli, "Launches and controls")
Rel(launchpad, tmux, "Manages sessions via")
Rel(launchpad, asciinema, "Initiates recording via")
Rel(tmux_controller, tmux, "Controls")

@enduml
```

## 2. Container Diagram

The Container diagram shows the main executable/crate containers within the `launchpad` and `tmux_controller` systems.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

System_Boundary(launchpad_boundary, "Launchpad CLI") {
    Container(launchpad_exe, "launchpad Executable", "Rust Executable", "Main entry point for orchestrating workflows")
    Container(orchestrator_lib, "Orchestrator Library", "Rust Crate", "Handles command execution, Gemini CLI management, and workflow stages")
    Container(gemini_cli_options_lib, "Gemini CLI Options Library", "Rust Crate", "Defines and parses Gemini CLI arguments")
    Container(dum_wrappers_lib, "Dum Wrappers Library", "Rust Crate", "Integrates with the 'dum' task runner")
}

System_Boundary(tmux_controller_boundary, "Tmux Controller CLI") {
    Container(tmux_controller_exe, "tmux_controller Executable", "Rust Executable", "Main entry point for tmux operations")
    Container(tmux_commands_lib, "Tmux Commands Library", "Rust Crate", "Implements specific tmux commands (e.g., split-horizontal, split-vertical)")
    Container(output_formatter_lib, "Output Formatter Library", "Rust Crate", "Formats CLI output")
}

Rel(launchpad_exe, orchestrator_lib, "Uses")
Rel(launchpad_exe, gemini_cli_options_lib, "Uses")
Rel(launchpad_exe, dum_wrappers_lib, "Uses")

Rel(tmux_controller_exe, tmux_commands_lib, "Uses")
Rel(tmux_controller_exe, output_formatter_lib, "Uses")

@enduml
```

## 3. Component Diagram (Launchpad)

The Component diagram for `launchpad` details its key internal components.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Container_Boundary(launchpad_exe, "launchpad Executable") {
    Component(main_fn, "main()", "Function", "CLI entry point, parses arguments and dispatches commands")
    Component(launchpad_args, "LaunchpadArgs", "Struct", "Defines and holds all command-line arguments")
    Component(run_launchpad_fn, "run_launchpad()", "Function", "Main logic for orchestrating workflows")

    Component(command_executor, "Command Executor", "Module", "Executes shell commands")
    Component(gemini_cli_manager, "Gemini CLI Manager", "Module", "Manages Gemini CLI installation and execution")
    Component(orchestrator_utils, "Orchestrator Utilities", "Module", "Provides utilities for dum, cargo, tmux, and sandboxed commands")

    Rel(main_fn, launchpad_args, "Parses")
    Rel(main_fn, run_launchpad_fn, "Calls")
    Rel(run_launchpad_fn, command_executor, "Uses")
    Rel(run_launchpad_fn, gemini_cli_manager, "Uses")
    Rel(run_launchpad_fn, orchestrator_utils, "Uses")
}

@enduml
```

## 4. Component Diagram (Tmux Controller)

The Component diagram for `tmux_controller` details its key internal components.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Container_Boundary(tmux_controller_exe, "tmux_controller Executable") {
    Component(main_fn_tc, "main()", "Function", "CLI entry point, parses arguments and dispatches commands")
    Component(commands_enum, "Commands Enum", "Enum", "Defines available tmux commands (e.g., SplitVertical, SplitHorizontal)")

    Component(split_horizontal_cmd, "handle_split_horizontal_command()", "Function", "Handles horizontal window splitting")
    Component(split_vertical_cmd, "handle_split_vertical_command()", "Function", "Handles vertical window splitting")
    Component(output_formatter_mod, "output_formatter", "Module", "Formats command output")

    Rel(main_fn_tc, commands_enum, "Parses and dispatches based on")
    Rel(main_fn_tc, split_horizontal_cmd, "Calls")
    Rel(main_fn_tc, split_vertical_cmd, "Calls")
    Rel(split_horizontal_cmd, output_formatter_mod, "Uses")
    Rel(split_vertical_cmd, output_formatter_mod, "Uses")
}

@enduml
```