# Run

## Description

Run targets of the build specified in `target.toml`

## Synopsys

`hydra` `run` [*option*] [*target name regex*]

## Options

`--help`: Display command description and options.

## Examples

`hydra` `run`

`hydra` `run` '.*-lint'

`hydra` `run` 'test_.*'