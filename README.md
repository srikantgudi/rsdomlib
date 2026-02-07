# rsdomlib

## RUST WASM DOM Handling App

### I wanted to try something unique in Rust Wasm—beyond just calling WASM functions from JS

**Rust WASM DOM Control** -for now it is Real-time timezone clock + live DOM updates from Rust.


### 1. Build

```bash
wasm-pack build --target web --dev
# Production: wasm-pack build --target web --release
```

### 2. Serve

```bash
npx serve .
# or python3 -m http.server 3000
# or any static file server
```

### 3. Open

```text
http://localhost:3000
```