# Add a Registry

## Description

The registry is used to index all released versions of the modules. It may be a local directory or a remote git repository. A remote registry may be updated in the cache using the command `edifice` `fetch`.

## Use a registry

### Update your environement variables

```bash
export EDIFICE_REGISTRY="/project/registry:${EDIFICE_REGISTRY}"
export EDIFICE_REGISTRY="git@github.com:Annrtl/Edifice_registry.git:${EDIFICE_REGISTRY}"
```

### Add a edifice.toml configuration locally

```toml
# edifice.toml
[config]
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
```

### Add a edifice.toml user configuration

```toml
# ~/.config/edifice/edifice.toml
[config]
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
```

### Add a edifice.toml global configuration

```toml
# /etc/edifice/edifice.toml
[config]
EDIFICE_REGISTRY = 'git@github.com:Annrtl/Edifice_registry.git:/project/registry'
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