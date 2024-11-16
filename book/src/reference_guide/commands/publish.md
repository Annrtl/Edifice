# Publish

## Description

Generate a copy of your module file with a provider section. 

Two provider type are available:
- Git

```toml
# $HYDRA_REGISTRY/module/module.toml with git provider
[provider]
uri = "git@github.com:hydra/module.git"
commit = "fad83baa880db7b3ba61b88fdafdde2a666da05f"
```

- Local

```toml
# $HYDRA_REGISTRY/module/module.toml with local provider
[provider]
uri = "/absolute/path/to/local/module/directory"
```

If `.git` directory doesn't exists in the module directory, local provider is used.

## Synopsys

`hydra` `publish` [*option*]

## Options

`--type`: Type of provider:
- `git`: Git uri
- `local`: Local path

`--help`: Display command description and options.

## Examples

`hydra` `publish`

`hydra` `publish` --type local