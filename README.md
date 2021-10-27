# Wasmer-Exports

Utility that uses wasmer to show exported function names from wasm files.

## Install

To build the binary and add it to `~/.cargo/bin/`:

```console
$ git clone https://github.com/chikai-io/wasmer-exports.git
$ cd wasmer-exports
$ cargo install --path .
```

## Usage

From a wasm file:

```console
$ wasmer-exports /path/to/file.wasm
file: /home/thiago-keyko/r/near/examples/rust-fungible-token/res/fungible_token.wasm
new
set_allowance
transfer_from
transfer
get_total_supply
get_balance
get_allowance
```

Recursively show from wasm files from the current directory:

```console
$ wasmer-exports -r .
```

## Related Issues on the Near Ecossystem

- [near-cli#849](https://github.com/near/near-cli/issues/849)
- [near-cli-rs#70](https://github.com/near/near-cli-rs/issues/70)
- [near-explorer#757](https://github.com/near/near-explorer/issues/757)
