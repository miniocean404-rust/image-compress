export CC=emcc
export AR=emar
wasm-pack build ./crates/wasm --dev --out-dir ../../dist/wasm
