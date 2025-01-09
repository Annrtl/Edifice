# Publish

## Description

Generate a copy of your module file with a registry section. 

Two registry type are available:
- Git

```toml
# $EDIFICE_REGISTRY/module/module.toml with git registry
[registry]
uri = "git@github.com:edifice/module.git"
commit = "fad83baa880db7b3ba61b88fdafdde2a666da05f"
```

- Local

```toml
# $EDIFICE_REGISTRY/module/module.toml with local registry
[registry]
uri = "/absolute/path/to/local/module/directory"
```

If `.git` directory doesn't exists in the module directory, local registry is used.

## Synopsys

`edifice` `publish` [*option*]

## Options

`--type`: Type of registry:
- `git`: Git uri
- `local`: Local path

`--help`: Display command description and options.

## Examples

`edifice` `publish`

`edifice` `publish` --type local