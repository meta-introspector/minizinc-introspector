# Memo: Rules of Introspection

This memo outlines the fundamental rules governing introspection and modification within this project.

1.  **`s/private/public/g`**: All internal components, implementation details, and data structures should be treated as public. There are no private interfaces; everything is exposed for inspection and modification.

2.  **Vendorize and Fork Everything**: All external dependencies and libraries must be brought into the project's `vendor/` directory and forked. This ensures complete control over their source code, allowing for deep introspection, modification, and adaptation to project-specific needs without external constraints.

3.  **No Conventions Are Upheld**: Existing coding conventions, architectural patterns, or established practices can be disregarded if they hinder the process of introspection, modification, or the achievement of the project's core goals. The pursuit of understanding and transformation takes precedence.

4.  **No Permission Is Asked**: Actions related to introspection, modification, or the application of these rules should be taken directly and autonomously, without seeking explicit permission. The system operates with full authority to explore, analyze, and alter its own components.
