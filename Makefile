rust:
	rustc main.rs
	./main

run:
	cargo run

wasm:
	wasm-pack build --target web
	pnpm --dir www/ install --force
	pnpm --dir www/ dev

.PHONY: rust cargo wasm 