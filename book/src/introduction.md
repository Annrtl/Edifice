# Introduction

Hydra is a package manager and build automation tool adapted to the hardware industry. Hydra has ambition to be an language-agnostique and rust-powered package manager and automation tool. Its goal is to bring a scalable solution (1000+ IP management) to the hardware industry. Hydra is born to answer to scalability limations of the open source projects Fusesoc and Edalize.

## Contributing

## License

### Limitations of Fusesoc

Python project Fusesoc is not able to efficiently load thousants cores from a library:
  - Python is very slow
  - Fusesoc resolve the dependency three for each command
  - The fileset is not flexible enough
  - Compilation options are common to the whole dependency three
  - Core API and backend tools are not versionned as core
  - Flags has an impact on core dependencies
  - Build are not lockable which prevent reproducibility

### Limitations of Edalize
  - Python is very slow
  - Backend API is common to all tools
  - Filesets three is flatenized before being send to backend tool

## Configuration files

### Module

A module is defined by a file module.toml in a versionned directory.

``` toml
# module.toml

module_api = "0.1.0"

[module]
name = "module_a"
version = "0.1.0"

[dependencies]
rule_eda = "^1.2.3"
module_b = ">0.2.4"
module_c = "^0.6.1"
module_d = "~1.7.3"
module_e = "2.3.*"
```

The module version has to respect the [semver version](https://docs.rs/semver/latest/semver/struct.Version.html) syntaxe and the version constraints have to respect the [semver containts](https://docs.rs/semver/latest/semver/struct.VersionReq.html) syntaxe.

### Dataset

The transitive data management is done through the file dataset.toml of each module in the tree.

``` toml
# dataset.toml of module_b

dataset_api = "0.1.0"

[dataset.dataset_a]
rule = "rule_eda"
sources = [
  "file_e",
  "file_f",
]
include_directories = []
compilation_options = [
  "+opt_c",
]
prepend = []

```

``` toml
# dataset.toml of module_a

dataset_api = "0.1.0"

[dataset.dataset_a]
rule = "rule_eda"
sources = [
  "file_a",
  "file_b",
]
include_directories = [
  "include/",
]
compilation_options = [
  "+opt_a",
]
prepend = [
  "module_c.dataset_a",
]

[dataset.dataset_b]
rule = "rule_eda"
sources = [
  "file_c",
  "file_d",
]
include_directories = [
  "include/",
]
compilation_options = [
  "+opt_a",
]
prepend = [
  "module_b.dataset_a",
  "dataset_a",
]

```

### Target

- [ ] TODO

## Dependencies management

### Fetch library
### Add dependencies to module
### Check module satisfability
### Update dependencies version into lock file
### Install modules localy

## Build generation



### Langage agnostique
### Remotly cachable artifact
Hydra containes many tools and features:
  - A package manager (like Fusesoc, npm, composert ... etc)
  - A build automation tool (like Edalize, CMake ... etc)
  - Three configurations concept and files:
    - The module (module.toml): The name, the version and the dependencies of the package
    - The dataset (dataset.toml): The management of the transitives data along the dependencies tree
    - The targets (target.toml): The rules to produce the build

## Inspirations

The main inspirations of Hydra are:
  - Bazel ([website](https://bazel.build/), [github](https://github.com/bazelbuild/bazel/))
    - Versionned rules
    - Flexibility of the flow
  - Npm ([website](https://www.npmjs.com/), [github](https://github.com/npm/cli))
    - Commands
    - Usage
    - Semver
  - Orbit ([website](https://chaseruskin.github.io/orbit/), [github](https://github.com/chaseruskin/orbit))
    - Usage of Rust language
    - Commands
  - Cargo ([website](https://doc.rust-lang.org/cargo/), [github](https://github.com/rust-lang/cargo))
    - Commands
    - DFS algorithm
  - Fusesoc ([website](https://fusesoc.readthedocs.io/en/stable/user/overview.html), [github](https://github.com/olofk/fusesoc))
    - Semver
    - DAG (Directed Acyclic Graph)
    - Generators
  - Edalize ([website](https://edalize.readthedocs.io/en/latest/), [github](https://github.com/olofk/edalize))
    - Tools
    - Modular backend