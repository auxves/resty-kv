# Resty KV

A simple key-value store based on Sqlite with an HTTP API.

## Usage

```sh
$ resty-kv file.db
```

or

```sh
$ RESTY_KV_FILE=file.db resty-kv
```

## Features

- String keys and values
- Optional Authentication by Bearer Header
- Config from environment (with `RESTY_KV_` prefix)
