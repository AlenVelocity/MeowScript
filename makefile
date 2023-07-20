.PHONY:
wasm:
	cargo build --release --target wasm32-unknown-emscripten -Z build-std=panic_abort,std
	cp target/wasm32-unknown-emscripten/release/meowscript.js site/public/meowscript.js
	cp target/wasm32-unknown-emscripten/release/meowscript.wasm site/public/meowscript.wasm