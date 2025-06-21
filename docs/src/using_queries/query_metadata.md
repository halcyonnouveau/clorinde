# Query metadata

## Query metadata
This is an opt-in feature. If you want to use it in your code, you must set the `enable_query_metadata` variable to `true` in your `clorinde.toml` file.

```toml
generate-field-metadata = true
```

This will add the `metadata` field to your `Query` object. You can use it to get field and type hints from your generated code.

For example you can get a list of the column names returned by your query via

```rust
            if let Some(rows) = &app.lock_info_rows {
                // Dynamically generate headers from field metadata
                let header = Row::new(
                    clorinde::queries::lock_info::LockInfo::field_metadata()
                        .iter()
                        .map(|meta| Cell::from(meta.name))
                        .collect::<Vec<_>>()
                );

```

## BUGS?

I might be holding it wrong, but if I have more than one query in the same file I get build errors with the generated code.

