wasm:
	wasm-pack build --target web

dev:
	wasm-pack build --target web
	npm run heroku-prebuild
	npm run --prefix www dev

prod:
	npm install
	wasm-pack build --target web
	npm run prebuild
	npm run build
	npm run start

.PHONY: wasm dev prod