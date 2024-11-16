# List

## Description

Display the list of available modules and their versions. If no regex pattern is specificed in argument, list all modules.

## Synopsys

`hydra` `list` [*option*] [*regex pattern*]

## Options

`--locked`: List only modules of the lockfile

`--help`: Display command description and options.

## Examples

`hydra` `list`

`hydra` `list` '.\*_model'

`hydra` `list` --locked

`hydra` `list` --locked '.\*_model' 