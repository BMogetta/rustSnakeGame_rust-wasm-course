FROM rust:alpine3.16 as Builder
WORKDIR /usr/app
RUN cargo install wasm-pack
COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ["src/lib.rs", "./"]
RUN wasm-pack build --target web

FROM node:16-alpine3.16 as base

WORKDIR /usr/app
COPY --from=builder /usr/app/pkg /usr/app/pkg
COPY ["www", "server", "/usr/app/"]
RUN ["npm", "install"]
RUN ["wasm-pack", "build", "--target", "web"]
RUN ["npm", "run", "heroku-prebuild"]
RUN ["npm," "run", "build"]
CMD ["npm", "run", "start"]
