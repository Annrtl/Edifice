# List

## Description

Display the list of available modules and their versions. If no regex pattern is specificed in argument, list all modules.

## Synopsys

`edifice` `list` [*option*] [*regex pattern*]

## Options

`--locked`: List only modules of the lockfile

`--help`: Display command description and options.

## Examples

`edifice` `list`

`edifice` `list` '.\*_model'

`edifice` `list` --locked

`edifice` `list` --locked '.\*_model' 