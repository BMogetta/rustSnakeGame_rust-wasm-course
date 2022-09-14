rust:
	rustc main.rs
	./main

run:
	cargo run

wasm: 
	wasm-pack build --target web

.PHONY: rust cargo wasm