# CLI
The CLI exposes three main commands: `schema`, `live`, and `fresh`.

```admonish note
This is only an overview of the CLI. You should read the help message for more complete information (`clorinde --help`)
```

## Generating code
The code generation can be made either against a database that you manage or by letting Clorinde manage an ephemeral database container for you.

### `schema`: Automatic container management
The `clorinde schema` command creates a new container, loads your schema(s), generates your queries and cleanups the container. You will need to provide the path to one or more schema files to build your queries against. This requires `docker` or `podman` to be installed.

### `live`: Manual database management
If you want to manage the database yourself, use the `clorinde live` command to connect to an arbitrary live database. You will need to provide the connection URL.

### `fresh`: Temporary database on existing server
The `clorinde fresh` command provides a middle-ground approach between `schema` and `live`. It connects to an existing PostgreSQL server, creates a temporary database, loads your schema files, generates your queries, and then drops the temporary database. This is useful when you have an existing PostgreSQL server but want the convenience of automatic schema loading without managing containers.

## Example Usage

Here are some examples of using the different commands:

```bash
# Using schema command with container management
clorinde schema schema.sql

# Using live command with existing database
clorinde live postgresql://user:pass@localhost/mydb

# Using fresh command with existing server
clorinde fresh schema.sql --url postgresql://user:pass@localhost

# Using fresh command with custom database name and search path
clorinde fresh schema.sql --url postgresql://user:pass@localhost \
  --db-name my_temp_db \
  --search-path public,custom_schema
```

## Useful flags
### `sync`
By default, Clorinde will generate asynchronous code, but it can also generate synchronous code using the `--sync` flag.

### `serialize` (DEPRECATED)
If you need to serialize the rows returned by your queries, you can use the `--serialize` flag, which will derive `Serialize` on your row types.

~~~admonish warning
This flag is deprecated and may be removed in a future version of Clorinde. Please use the `types.derive-traits` configuration value. For example, a  `clorinde.toml` that includes this will be functionally equivalent as using the flag.

```toml
[types]
derive-traits = ["serde::Serialize"]
```

This will also add `serde` to the manifest dependencies.
~~~

### `podman`
You can use `podman` as a container manager by passing the `-p` or `--podman` flag.
