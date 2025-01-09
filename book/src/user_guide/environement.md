# Environement

## Variables

- `EDIFICE_BUILD`: The path where the build directories are created. By default, the build is created in the current modude directory next to `module.toml`.
- `EDIFICE_CACHE`: The path where the remote registries and the remote module sources are downloaded. By default, the cache is located in `~/.cache/edifice`.
- `EDIFICE_REGISTRY`: The URI of the registries separated with a colon (`:`).

## Workspace configuration

```toml
# edifice.toml
[config]
EDIFICE_BUILD = '/local/build'
EDIFICE_CACHE = '~/.cache/edifice'
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
```

## User configuration

```toml
# ~/.config/edifice/edifice.toml
EDIFICE_BUILD = '/local/build'
EDIFICE_CACHE = '~/.cache/edifice'
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
```

## Global configuration

```toml
# /etc/edifice/edifice.toml
EDIFICE_BUILD = '/local/build'
EDIFICE_CACHE = '~/.cache/edifice'
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
```