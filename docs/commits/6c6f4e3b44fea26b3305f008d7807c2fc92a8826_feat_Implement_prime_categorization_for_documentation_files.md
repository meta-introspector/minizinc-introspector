feat: Implement prime categorization for documentation files

This commit introduces a new Rust program (`src/main.rs`) designed to categorize markdown files within the project based on a system of "prime categories" (e.g., Duality/Foundation, Workflow/SOPs/OODA).

The program analyzes file names and content to assign categories and proposes moving the files into a structured `docs/prime_categorized` directory. It also generates `README.md` files for each category and sense, providing an organized overview of the documentation.

This feature aims to improve knowledge management and discoverability within the project's extensive documentation.