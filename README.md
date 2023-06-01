# 🏗️ FlowCrafter

FlowCrafter is a command-line tool to create and manage workflows for
[GitHub Actions].

## Installation

FlowCrafter can be installed using `cargo`. Running the following command will
download the latest version of FlowCrafter, compile it, and make it available as
a command-line tool.

```shell
cargo install flowcrafter
```

After the installation, `flowcrafter` is available as a command in the terminal:

```shell
flowcrafter --help
```

## Usage

Using FlowCrafter consists of three different steps:

1. [Creating templates for workflows and jobs](#create-templates)
2. [Initializing FlowCrafter in a repository](#initialize-flowcrafter)
3. [Creating and updating workflows](#create-a-workflow)

### Create Templates

FlowCrafter uses templates stored on GitHub to create workflows and jobs. These
are YAML files that follow a few conventions:

- The repository contains a folder for each workflow (e.g. `rust`).
- Each workflow folder contains a file called `workflow.yml` that defines the
  workflow (e.g. `rust/workflow.yml`).
- Job templates are stored in the same folder as the workflow file (e.g.
  `rust/lint.yml`).

#### Workflow

The workflow template sets the top-level configuration for the workflow. It can
also contain a list of jobs that should always be part of the workflow.

The workflow template is always named `workflow.yml`.

```yaml
---
name: Rust

"on":
  push:
    branches:
      - main
  pull_request:

jobs:
  run-always:
    name: Always include this job
    runs-on: ubuntu-latest

    steps:
      - run: echo "This will always be included"
```

#### Jobs

Jobs are defined in individual YAML files within the workflow folder. Each
represents a single job that can be included in a workflow.

```yaml
style:
  name: Check style
  runs-on: ubuntu-latest

  steps:
    - name: Checkout code
      uses: actions/checkout@v3

    - name: Run Rustfmt
      run: cargo fmt --all -- --check
```

### Initialize FlowCrafter

FlowCrafter manages the workflows for a repository on GitHub. After cloning the
repository to your local machine, open a terminal, change into its directory,
and then generate a configuration file for FlowCrafter:

```shell
flowcrafter init -r <owner>/<repo>
```

This will create a file called `.flowcrafter.yml` in the `.github` directory and
configure the repository `owner/repo` as the source for workflow and job
templates.

### Create a Workflow

With FlowCrafter initialized and templates on GitHub, you can now create a
workflow with FlowCrafter:

```shell
flowcrafter create -w <workflow> -j <job> -j <job>
```

For example, given a repository with the following templates:

```text
rust
├── lint.yml
├── test.yml
└── workflow.yml
```

You can create a workflow for Rust with the following command:

```shell
flowcrafter create -w rust -j lint -j test
```

This will create the file `.github/workflows/rust.yml` and merge `workflow.yml`
and the two jobs `lint.yml` and `test.yml` into it.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[github actions]: https://github.com/features/actions
