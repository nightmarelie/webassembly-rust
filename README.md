# Setup
```bash
rustup target add wasm32-unknown-unknown

cargo install wasm-pack
cargo install wasm-gc
cargo install https
```

# Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

# Run
```bash
wasm-gc target/wasm32-unknown-unknown/release/utils.wasm -o utils.gc.wasm 
```

