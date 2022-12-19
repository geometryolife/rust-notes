# cargo-watch

- [GitHub](https://github.com/watchexec/cargo-watch)
- [Homepage](https://watchexec.github.io/#cargo-watch)

## Install

```shell
cargo install cargo-watch
```

## Use

By default, it runs `check`. You can easily override this, though:

```bash
cargo watch [-x command]...
```

A few examples:

```bash
# Run tests only
cargo watch -x test

# Run check then tests
cargo watch -x check -x test

# Run run with arguments
cargo watch -x 'run -- --some-arg'

# Run an arbitrary command
cargo watch -- echo Hello world

# Run with features passed to cargo
cargo watch --features "foo,bar"
```

There's a lot more you can do! Check out:

- [Usage guide](https://github.com/watchexec/cargo-watch/blob/main/USAGE.md)
- [Troubleshooting](https://github.com/watchexec/cargo-watch/blob/main/TROUBLESHOOT.md)
