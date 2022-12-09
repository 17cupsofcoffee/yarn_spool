# yarn_spool

yarn_spool is a runtime for [Yarn Spinner](https://yarnspinner.dev) scripts,
written in Rust.

Note that this library does not provide tools for parsing Yarn scripts, or compiling
them down to bytecode - the [official compiler](https://github.com/YarnSpinnerTool/YarnSpinner-Console)
can be used for this.

## Installation

```toml
yarn_spool = { git = "https://github.com/17cupsofcoffee/yarn_spool" }
```

## Usage

See [`examples/cli.rs`](examples/cli.rs) for a full annotated example.

## License

Licensed under the [MIT license](LICENSE).
