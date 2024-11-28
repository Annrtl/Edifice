# Add a Registry

## Description

The registry is used to index all released versions of the modules. It may be a local directory or a remote git repository. A remote registry may be updated in the cache using the command `hydra` `fetch`.

## Use a registry

### Update your environement variables

```bash
export HYDRA_REGISTRY="/project/registry:${HYDRA_REGISTRY}"
export HYDRA_REGISTRY="git@github.com:Annrtl/Hydra_registry.git:${HYDRA_REGISTRY}"
```

### Add a hydra.toml configuration locally

```toml
# hydra.toml
[config]
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```

### Add a hydra.toml user configuration

```toml
# ~/.config/hydra/hydra.toml
[config]
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```

### Add a hydra.toml global configuration

```toml
# /etc/hydra/hydra.toml
[config]
HYDRA_REGISTRY = 'git@github.com:Annrtl/Hydra_registry.git:/project/registry'
```

## Create your own registry

### Remote repository

1. Create a new repository
2. For each module to index add a `module.toml` anywhere in the repository.

It is recommanded to put a file `module.toml` in a a folder `name/version/module.toml`

### Local directory

1. Create a new directory
2. For each module to index add a `module.toml` anywhere in the repository.

It is recommanded to put a file `module.toml` in a a folder `name/version/module.toml`