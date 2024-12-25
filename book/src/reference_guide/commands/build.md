# Build

## Description

Generate the build directory with a Makefile to run target. This generation depends on the backend rules used by the module.

## Synopsys

`edifice` `build` [*option*]

## Options

`--build-dir`: Location of the generate build directory. By default the location is the current directory. This option overwrite the environment variable `EDIFICE_BUILD_DIR`.

`--help`: Display command description and options.

## Examples

`edifice` `build`

`edifice` `build` --build-dir /local/edifice