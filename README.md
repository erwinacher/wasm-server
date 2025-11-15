![License](https://img.shields.io/badge/license-MIT-blue.svg)


## WASM Graph Demo

### Modular WebAssembly execution with Rust (Axum) + Next.js

This project is a minimal, open-source demonstration of a graph-driven WebAssembly execution system.

Each WASM module contains:
- A tiny piece of logic (in Rust)
- A tiny asset (like a string or color)
- A simple ABI (get_text, get_color)

The browser (Next.js) loads a graph.json from a Rust/Axum server.
Users can step through the graph manually, or auto-run the whole graph.

#### This repo demonstrates:
- WebAssembly modules as isolated logic/asset units
- Dynamic module loading over HTTP
- Streaming .wasm responses from Rust/Axum
- A clean Next.js client executing modules in sequence
- Graph-based orchestration logic in TypeScript

#### Features

- Three example WASM modules written in Rust  
- Axum server that streams .wasm files incrementally  
- Graph-based execution (node1 → node2 → node3)  
- Next.js UI showing:  
   - Colored box updated by each module
   - Module text logs
   - “Step Manually” button
   - “Auto Run Graph” button
- Minimal, easy-to-understand codebase
- Great starting point for modular WASM architectures

### Project structure

```bash
/wasm-modules/
   module1/
   module2/
   module3/
   module4/

/wasm-server/   (Axum)
   src/
   static/      <-- compiled .wasm copied here
   Cargo.toml

/next-app/      (Next.js)
   src/app/page.tsx
   package.json
```

#### 1. Install prerequisites
You’ll need:
- Rust stable
- WASM target
- Node.js 18+
- PNPM / Yarn / npm
 
#### 2. Install Rust target:
```
rustup target add wasm32-unknown-unknown
```

#### 3. Build the WASM modules

From each module folder:
```
cd wasm-modules/module1
cargo build --target wasm32-unknown-unknown --release
```

Do the same for module2, and module3 and module4

Each module produces:
```
target/wasm32-unknown-unknown/release/moduleX.wasm
```

Copy each .wasm into the Axum server:
```
cp module1/target/wasm32-unknown-unknown/release/module1.wasm ../wasm-server/static/
cp module2/target/wasm32-unknown-unknown/release/module2.wasm ../wasm-server/static/
cp module3/target/wasm32-unknown-unknown/release/module3.wasm ../wasm-server/static/
cp module4/target/wasm32-unknown-unknown/release/module4.wasm ../wasm-server/static/
```

#### 4. Run the Axum server

Inside /wasm-server:
```
RUST_LOG=debug cargo run
```

This starts the backend on:
```
http://localhost:3002
```

Endpoints:

| Path         | Description          |
|--------------|----------------------|
| /graph.json  | Execution graph      |
| /wasm/{file} | Streams WASM modules |
| /hello       | Test route           |
 

#### 4. Run the Next.js app

Inside /next-app:
```
npm install  # or yarn / pnpm
npm run dev
```

This starts the client at:
```
http://localhost:3000
```

Open this in your browser.

#### How It Works (Architecture)
##### 1. Axum streams WASM modules

Using tokio::fs::File + ReaderStream, the server streams .wasm chunk-by-chunk.

This allows:

- Immediate compilation while downloading
- Large WASM files
- Module independence

##### 2. Next.js fetches graph.json

Example graph:
```
{
  "start": "node1",
  "nodes": {
    "node1": { "wasm": "/wasm/module1.wasm", "next": "node2" },
    "node2": { "wasm": "/wasm/module2.wasm", "next": "node3" },
    "node3": { "wasm": "/wasm/module3.wasm", "next": null }
  }
}
```

##### 3. Next.js loads modules dynamically

Using:
```
WebAssembly.instantiateStreaming(fetch(url), {})
```

Each WASM module exports:
```
#[no_mangle]
pub extern "C" fn get_color() -> u32;

#[no_mangle]
pub extern "C" fn get_text() -> *const u8;
```

The UI then:
- Reads the RGB color
- Reads the text from memory
- Displays them

##### 4. Two execution modes

Manual

Click "Step Manually"
-> loads exactly one WASM module and executes it.

Auto-Run

Click "Auto Run Graph"
-> walks through the full graph automatically with time delay.

#### Example Output
```
[node1] Hello from module1
[node2] Hello from module2
[node3] Hello from module3
[node4] Hello from module4
```

UI shows:

- Blue box from module1
- Red box from module2

etc.

#### This project aims to show how WebAssembly modules can be:

- Modular
- Dynamic
- Streamed
- Loaded based on graphs
- Executed safely

#### License

MIT License


#### Thanks

This project exists to demonstrate the power of modular WebAssembly architectures and graph-based execution models.

