call cargo build --profile wasm-release --target wasm32-unknown-unknown

call wasm-bindgen --out-name voxels --out-dir wasm/webassembly --target web target/wasm32-unknown-unknown/wasm-release/bevy-voxels.wasm

::call wasm-opt -Os -o output.wasm input.wasm

start "" http://localhost:8000/

call python -m http.server