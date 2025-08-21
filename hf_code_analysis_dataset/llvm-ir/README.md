# LLVM IR Analysis Dataset: unknown-source

This dataset contains comprehensive LLVM IR analysis data extracted from Rust source `.` using the LLVM IR extractor.

## Dataset Overview

- **Source**: .
- **Optimization Levels**: ["O0", "O1", "O2", "O3"]
- **Extraction Tool**: LLVM IR extractor (part of hf-dataset-validator-rust)
- **Format**: Apache Parquet files optimized for machine learning
- **Compression**: Snappy compression for fast loading

## Dataset Structure

### Phase-Based Organization

The dataset captures the complete Rust → LLVM IR compilation pipeline:

#### 1. IR Generation (`ir_generation-*-phase/`)
- Initial LLVM IR generation from Rust source
- Type system mappings (Rust types → LLVM types)
- Function signature transformations
- Basic block and instruction analysis

#### 2. Optimization Passes (`optimization_passes-*-phase/`)
- LLVM optimization pass applications and effects
- Before/after IR comparisons for each optimization
- Performance impact measurements
- Optimization decision analysis

#### 3. Code Generation (`code_generation-*-phase/`)
- Final IR → machine code generation patterns
- Target-specific optimizations and transformations
- Register allocation and instruction selection
- Assembly code generation analysis

#### 4. Performance Analysis (`performance_analysis-*-phase/`)
- Execution cycle estimates and performance metrics
- Code size and complexity analysis
- Optimization impact correlation
- Performance regression detection

#### 5. Type System Mapping (`type_system_mapping-*-phase/`)
- Detailed Rust type → LLVM type conversions
- Generic parameter handling and monomorphization
- Trait object representation analysis
- Lifetime analysis impact on IR generation

#### 6. Memory Analysis (`memory_analysis-*-phase/`)
- Stack and heap allocation pattern analysis
- Memory safety guarantee preservation
- Reference counting and ownership in IR
- Memory layout optimization analysis

## Optimization Levels

Each phase is analyzed across multiple optimization levels:
- **O0**: No optimization (debug builds)
- **O1**: Basic optimizations
- **O2**: Standard optimizations (release builds)
- **O3**: Aggressive optimizations

## Schema

Each record contains:
- **Source Context**: Original Rust code, line/column, construct type
- **LLVM IR**: Generated IR code, instruction counts, basic blocks
- **Optimization Data**: Passes applied, before/after comparisons, impact scores
- **Code Generation**: Target architecture, assembly code, register usage
- **Performance Metrics**: Cycle estimates, code size, complexity scores
- **Type Mappings**: Rust → LLVM type conversions and analysis
- **Memory Patterns**: Allocation analysis and safety preservation
- **Processing Metadata**: Timestamps, tool versions, processing times

## Applications

This dataset enables research in:
- **Compiler Optimization**: Understanding LLVM optimization effectiveness
- **Performance Prediction**: Predicting performance from source patterns
- **Code Generation**: Learning optimal IR generation strategies
- **Type System Research**: Understanding type system compilation
- **Memory Safety**: Analyzing memory safety preservation in compilation

## Usage

### Loading with Python

```python
import pandas as pd

# Load IR generation data for O2 optimization
ir_gen_df = pd.read_parquet('ir_generation-O2-phase/data.parquet')
print(f"Loaded {len(ir_gen_df)} IR generation records")

# Load optimization pass data
opt_df = pd.read_parquet('optimization_passes-O2-phase/data.parquet')
print(f"Loaded {len(opt_df)} optimization records")
```

### Loading with Rust

```rust
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

// Load LLVM IR data
let file = std::fs::File::open("ir_generation-O2-phase/data.parquet")?;
let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
let reader = builder.build()?;

for batch_result in reader {
    let batch = batch_result?;
    println!("Loaded batch with {} LLVM IR records", batch.num_rows());
}
```

## Generation Details

- **Generated**: 2025-08-21 00:27:23 UTC
- **Tool Version**: LLVM IR extractor (hf-dataset-validator-rust)
- **Source**: .
- **Optimization Levels**: ["O0", "O1", "O2", "O3"]
- **Total Phases**: 6 analysis phases × 4 optimization levels
