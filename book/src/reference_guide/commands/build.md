# Build

## Description

Generate the build directory with a Makefile to run target. This generation depends on the backend rules used by the module.

## Synopsys

`hydra` `build` [*option*]

## Options

`--build-dir`: Location of the generate build directory. By default the location is the current directory. This option overwrite the environment variable `HYDRA_BUILD_DIR`.

`--help`: Display command description and options.

## Examples

`hydra` `build`

`hydra` `build` --build-dir /local/hydra