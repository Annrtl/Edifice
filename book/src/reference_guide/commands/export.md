# Export

## Description

Create a compressed directory allowing anyone to run the targets without edifice tool. This file is generated in the module directory next to module.toml. If no build directory exists yet, `edifice` `build` is implicitly ran and then compressed. 

The compressed directory contains:
- Makefile / Script
- Sources
- Artifacts (If some targets was already ran in build directory before the export)

## Synopsys

`edifice` `export` [*option*]

## Options

`--format`: Compression format. Default format is `gz`
- `gz`: *module*_*version*.tar.gz
- `zip`: *module*_*version*.zip

`--name` *name*: Name of the compressed file: *name*.tar.gz

`--help`: Display command description and options.

## Examples

`edifice` `export`

`edifice` `export` --format zip

`edifice` `export` --name issue_1234