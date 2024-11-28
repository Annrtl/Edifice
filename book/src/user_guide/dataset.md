# Create your dataset

The dataset handle the current module files and its children's dataset. The dataset are not used in the tree resolution. We need it at build time to provide the data to the top level module. At each level user sould have as much control as possible on every lower level dataset.

## Dataset containing files

``` toml
# dataset.toml

[eda.dataset.dataset_a]
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