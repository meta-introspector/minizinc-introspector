# Project Overview

This document provides a high-level overview of the `libminizinc` project, its core purpose, and key components.

## Purpose

The `libminizinc` project aims to integrate MiniZinc models with Rust applications, enabling advanced constraint programming and optimization within a Rust ecosystem. It serves as a foundational layer for building intelligent systems that leverage MiniZinc's powerful modeling capabilities.

## Key Components

### `zos-bootstrap`

This crate provides command-line utilities for bootstrapping and managing various aspects of the project. It includes commands for building, testing, running, debugging, and analyzing code.

### `zos-fast-query`

This crate is responsible for fast querying and term recognition within the project's knowledge base. It includes a build script that processes hierarchical term indexes and generates optimized recognizer data.

### `ragit-string-utils`

A utility crate for string manipulation, particularly focused on parsing key-value pairs from text. It provides reusable functions for common string-related tasks.

### `minizinc-output-parser`

This crate is designed to parse and interpret the output from MiniZinc solvers, converting it into a format that can be easily consumed and processed by Rust applications.

### `rust_lattice_project`

This crate serves as a collection of Rust language feature examples, including macro definitions and their usage. It's primarily for understanding and testing various Rust constructs rather than being a core functional component of the main project.

## Development Philosophy

The project emphasizes modularity, clean code, and robust testing. It aims to provide a stable and efficient integration layer between Rust and MiniZinc, facilitating the development of complex optimization and AI-driven solutions.
