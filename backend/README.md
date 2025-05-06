# Hummanta Backend Template

This is a template for when you want to create a new backend.

The template will create the following project structure:

```
.
├── .github               # GitHub-specific configurations, such as workflows.
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
    └── main.rs           # Main entry point of the Rust application.
```
