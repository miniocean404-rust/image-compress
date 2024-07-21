set CC=emcc
set AR=emar
wasm-pack build .\crates\wasm --dev --out-dir ..\..\dist\wasm
