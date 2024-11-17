# Environement

## Variables

- `HYDRA_BUILD`: The path where the build directories are created. By default, the build is created in the current modude directory next to `module.toml`.
- `HYDRA_CACHE`: The path where the remote registries and the remote module sources are downloaded. By default, the cache is located in `~/.cache/hydra`.
- `HYDRA_REGISTRY`: The URI of the registries separated with a colon (`:`).

## Workspace configuration

```toml
# hydra.toml
[config]
HYDRA_BUILD = '/local/build'
HYDRA_CACHE = '~/.cache/hydra'
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```

## User configuration

```toml
# ~/.config/hydra/hydra.toml
HYDRA_BUILD = '/local/build'
HYDRA_CACHE = '~/.cache/hydra'
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```

## Global configuration

```toml
# /etc/hydra/hydra.toml
HYDRA_BUILD = '/local/build'
HYDRA_CACHE = '~/.cache/hydra'
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```