# MiniZinc Sieve Model Performance Report

## 1. Introduction

This report summarizes the performance of the MiniZinc sieve model (`sieve_embedding.mzn`) across a defined parameter lattice. The goal is to understand the relationship between model parameters (`num_vars`, `num_values`, `num_partitions`) and execution time.

## 2. Test Setup

Tests were conducted using the `minizinc_test_runner_rs` Rust program, which dynamically generates DZN data files for each test case and executes the MiniZinc solver. The following parameter ranges were explored:

*   `num_vars`: 1, 2, 3
*   `num_values`: 2, 3
*   `num_partitions`: 2, 3

## 3. Performance Results

The following table presents the execution time for each parameter combination, sorted from fastest to slowest:

| Duration (s) | num_vars | num_values | num_partitions |
|--------------|----------|------------|----------------|

## 4. Analysis and Observations

*(Analysis will go here. This section will be populated with insights after reviewing the data, potentially including plots and regression analysis.)*

## 5. Runtime Prediction (Placeholder)

*(Runtime prediction model and results will go here.)*

