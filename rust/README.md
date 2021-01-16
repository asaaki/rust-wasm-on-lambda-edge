# Rust/WASM on AWS Lambda@Edge - Rust code

The business logic lives here.

## Building

Note that you will not call `cargo build` directly, instead [wasm-pack][wp] CLI is used.

If you have Rust on the machines building the project:

```sh
cargo install wasm-pack
```

For other enviroments and needs, [consult the documentation][install].

Check the `Makefile` in the parent directory for how it is called.



[wp]: https://rustwasm.github.io/wasm-pack/
[install]: https://rustwasm.github.io/wasm-pack/installer/
