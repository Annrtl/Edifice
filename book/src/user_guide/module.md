# Create your Module

The file module.toml defines the name, the version and the dependencies of your module.. The sections name and versions are already defines. 

## Find a depedency

To find the dependency you need you can use the command `hydra` `search` as follow:

```shell
hydra search <name>
```

or

```shell
hydra search <description>
```

The full list of the registry modules is available with the command `hydra` `list`

```shell
hydra list
```

To list locker module only add the option `--locked`

```shell
hydra list --locked
```

## Add depedency

To add a dependency from the registry you need to use the command `hydra` `add` as follow:

```shell
hydra add <dependency_name>
```
