# Rust/WASM on AWS Lambda@Edge - nodejs wrapper

Very small and simple TypeScript project which acts as a glue between the node
runtime of AWS Lambda and the WebAssembly module created from the Rust code.

## Building

```sh
npm install
npm run build
```

Note: the WASM package should have been build first, so this part can pick it up
as a dependency.

Check the `Makefile` in the parent directory for how this is achieved.
