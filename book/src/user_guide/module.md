# Create your Module

The file module.toml defines the name, the version and the dependencies of your module.. The sections name and versions are already defines. 

## Find a depedency

To find the dependency you need you can use the command `edifice` `search` as follow:

```shell
edifice search <name>
```

or

```shell
edifice search <description>
```

The full list of the registry modules is available with the command `edifice` `list`

```shell
edifice list
```

To list locker module only add the option `--locked`

```shell
edifice list --locked
```

## Add depedency

To add a dependency from the registry you need to use the command `edifice` `add` as follow:

```shell
edifice add <dependency_name>
```
