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

  - [`detector`](./detector/README.md): Detecting a project's programming language.
  - [`frontend`](./frontend/README.md): Parsing the source code and generating the Cranelift IR.

## Contributing

To keep the generated code up to date, install [`just`](https://github.com/casey/just)
and run `just generate-all` (or a specific template -e.g. `just generate-package`).

## License

Copyright (c) The Hummanta Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
