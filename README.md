# templates

Starter templates for Hummanta specification projects, with predefined structure and validation.

## Getting Started

1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation):

   ```bash
   cargo install cargo-generate
   ```

2. Generate a new project using one of the templates:

   ```bash
   cargo generate hummanta/templates
   ```

3. Choose one of the following templates:

  - `package`: A hummanta specification package, for example, a detector, compiler, or target.

## Contributing

To keep the generated code up to date, install [`just`](https://github.com/casey/just) and run `just generate-all` (or a specific template -e.g. `just generate-package`).
