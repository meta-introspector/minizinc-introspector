# Cargo2HF Dataset: unknown-project

This dataset contains comprehensive analysis data extracted from the Cargo project `.` using the cargo2hf tool.

## Dataset Overview

- **Source Project**: .
- **Include Dependencies**: false
- **Extraction Tool**: cargo2hf (part of hf-dataset-validator-rust)
- **Format**: Apache Parquet files optimized for machine learning
- **Compression**: Snappy compression for fast loading

## Dataset Structure

### Phase-Based Organization

The dataset is organized into multiple phases, each capturing different aspects of the Cargo project:

#### 1. Project Metadata (`project_metadata-phase/`)
- Basic project information from Cargo.toml
- Authors, license, description, keywords, categories
- Repository and documentation URLs
- Project versioning information

#### 2. Dependency Analysis (`dependency_analysis-phase/`)
- Direct and transitive dependency graphs
- Version constraints and resolution
- Feature flags and optional dependencies
- Dependency source analysis (crates.io, git, path)

#### 3. Source Code Analysis (`source_code_analysis-phase/`)
- Lines of code metrics by file type
- Function, struct, enum, trait counts
- Code complexity measurements
- Documentation coverage analysis
- Public API surface analysis

#### 4. Build Analysis (`build_analysis-phase/`)
- Build script analysis (build.rs)
- Target platform configurations
- Feature flag combinations
- Compilation profiles and settings

#### 5. Ecosystem Analysis (`ecosystem_analysis-phase/`)
- Crates.io metadata and download statistics
- GitHub repository metrics (stars, forks, issues)
- Community engagement indicators
- Popularity and adoption metrics

#### 6. Version History (`version_history-phase/`)
- Git commit history analysis
- Contributor statistics
- Release patterns and frequency
- Project evolution tracking

## Schema

Each record contains:

- **Identification**: Unique ID, project path, name, version
- **Phase Information**: Which analysis phase generated the data
- **Project Metadata**: Description, authors, license, repository info
- **Code Metrics**: Lines of code, file counts, complexity scores
- **Dependency Data**: Dependency counts and detailed dependency information
- **Build Configuration**: Features, targets, build script complexity
- **Ecosystem Metrics**: Download counts, GitHub stats, community metrics
- **Version History**: Commit counts, contributor info, project age
- **Processing Metadata**: Timestamps, tool versions, processing times

## Applications

This dataset is valuable for:

- **Dependency Analysis**: Understanding Rust ecosystem patterns
- **Code Quality Research**: Analyzing code metrics and best practices  
- **Build System Studies**: Understanding Cargo build configurations
- **Ecosystem Evolution**: Tracking how Rust projects develop over time
- **Machine Learning**: Training models on Rust project patterns

## Complementary Datasets

This cargo2hf dataset complements:
- **rust-analyzer datasets**: Semantic analysis and compiler internals
- **crates.io datasets**: Registry-wide ecosystem analysis
- **GitHub datasets**: Repository and community metrics

## License

This dataset is generated from open source Rust projects and inherits their respective licenses.
The extraction tool and dataset format are licensed under AGPL-3.0.

## Generation Details

- **Generated**: 2025-08-07
- **Tool Version**: cargo2hf (hf-dataset-validator-rust)
- **Source Project**: .
- **Dependencies Included**: false
- **Total Phases**: 6 analysis phases
