# Hummanta Frontend Template

This is a template for when you want to create a new frontend.

The template will create the following project structure:

```
.
├── .github               # GitHub-specific configurations, such as workflows.
├── build.rs              # Build script for custom compilation processes.
├── CONTRIBUTING.md       # Guidelines for contributing to the project.
├── Cargo.toml            # Manifest file for Rust project dependencies and metadata.
├── LICENSE               # License file specifying the terms of use for the project.
├── README.md             # Main documentation file providing an overview of the project.
├── hmt-package.toml      # Configuration file specific to the Hummanta package.
├── justfile              # Task runner file for defining and automating project tasks.
├── rustfmt.toml          # Configuration file for Rust code formatting rules.
├── samples               # Directory containing sample data or examples for the project.
└── src                   # Source code directory for the project.
    ├── args.rs           # Command-line argument parsing implementation.
    ├── ast.rs            # Abstract Syntax Tree (AST) definitions and operations.
    ├── codegen.rs        # High-level code generation orchestration.
    ├── emit.rs           # Code generation logic (e.g., compiling AST to target output).
    ├── error.rs          # Error handling definitions and utilities.
    ├── grammar.lalrpop   # LALRPOP grammar definition file for parser generation.
    ├── lexer.rs          # Lexical analysis/tokenization implementation.
    ├── lib.rs            # Library root defining the public API for crate consumers.
    ├── main.rs           # Main entry point of the Rust application.
    ├── parser.rs         # Parser implementation (likely generated from grammar.lalrpop).
    └── token.rs          # Token definitions and related functionality.
```
