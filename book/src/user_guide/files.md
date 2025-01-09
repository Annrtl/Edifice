# Workspace files

## Principle

The modules definition is split in tree files: `module.toml`, `dataset.toml` and `target.toml`. This methode optimize the execution time of Edifice commands.

### module.toml

The registries index only `module.toml`. This file required only to have a name, version, description and dependencies constraints. When the registry is fetched and analysed, Edifice has to read little files. The `dataset.toml` has to be known only at build time and `target.toml` only when the specified module is built.

### dataset.toml

This file is accessible in the cache after the resolution of the dependencies tree. Edifice has to read this file only to get transitive data, to create the build.

### target.toml

This file is only used when developer work on the specified module (E.g. running test). Edifice doesn't need to read the content of each dependencies's `target.toml`.

## Commands depending on the files

| Command | module.toml | dataset.toml | target.toml |
| ------- | ----------- | ------------ | ----------- |
| add     | x           |              |             |
| build   |             | x            | x           |
| check   | x           |              |             |
| clean   |             |              |             |
| export  | x           |              |             |
| fetch   |             |              |             |
| help    |             |              |             |
| info    | x           |              |             |
| install |             |              |             |
| list    | x           |              |             |
| new     |             |              |             |
| prune   |             |              |             |
| publish | x           |              |             |
| run     | x           |              | x           |
| search  | x           |              |             |
| update  | x           |              |             |
| version |             |              |             |
