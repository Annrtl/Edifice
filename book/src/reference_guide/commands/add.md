# Add

## Description

Add a dependency to the current directory module.

## Synopsys

`hydra` `add` [*option*] *name*
- Where *name* is a String.

`hydra` `add` [*option*] *name*@*version_requirement*
- Where *name* is a String.
- Where *version_requirement* is a String with respecting the format of [VersionReq](https://docs.rs/semver/latest/semver/struct.VersionReq.html).

## Options

`--dry-run`: Look for an available version of the module but do not modify module.toml.

`--help`: Display command description and options.

## Examples

`hydra` `add` dff

`hydra` `add` dff@0.1.4

`hydra` `add` dff@^0.1.0