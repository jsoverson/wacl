# wasmcloud actor cli (wacl)

This is a light wrapper around `wasmcloud-host` to easily test individual wasm actors.

## Install

```bash
cargo install --git git@github.com:jsoverson/wacl.git
```

## Usage

```
wacl <path to signed wasm file> <command to run> <JSON data>
```

## Limitations

This doesn't connect to a lattice nor does it support linking to any other actors or providers.
