# **rustSnakeGame is the project exercise of the Rust & WebAssembly course by [Filip Jerga](https://www.youtube.com/c/Eincode/playlists)**

This repository contains the source code and my notes about the course of Rust & WebAssembly with TS that I'm taking

<br>

**[Rust](#rust) + [ WebAssembly](#wasm) + [NodeJS](#node) +[TypeScript](#ts)**

<br>

---

<a name="rust"></a>

## **What is [Rust?](https://doc.rust-lang.org/book/)**

Rust is a programing language. Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

[Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

```
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

rustup update

rustc --version
```

<br>

---

<a name="wasm"></a>

## **What is [WebAssembly](https://webassembly.org/)?** 

WebAssembly is a new type of code that can be run in modern web browsers — it is a low-level assembly-like language with a compact binary format that runs with near-native performance and provides languages such as C/C++, C#, and Rust with a compilation target so that they can run on the web. It is also designed to run alongside JavaScript, allowing both to work together.

### Rust to Wasm with [wasm-pack](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

```
cargo install wasm-pack
```

<br>

---

<a name="node"></a>

## **What is [Node.js](https://nodejs.org/en/docs/)?** 

Node.js is an open-source, cross-platform, back-end JavaScript runtime environment that runs on a JavaScript Engine and executes JavaScript code outside a web browser, which was designed to build scalable network applications.

[Install node](https://github.com/nodesource/distributions#installation-instructions)

```
curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -

sudo apt-get install -y nodejs

node -v
```
Install npm

```
sudo apt install -y npm

npm -v
```

<br>

---

<a name="ts"></a>

## **What is [TypeScript](https://www.typescriptlang.org/docs/)?** 

TypeScript is a free and open source programming language developed and maintained by Microsoft. It is a strict syntactical superset of JavaScript and adds optional static typing to the language. It is designed for the development of large applications and transpiles to JavaScript.

<br>

---
## **Usefull extensions**


Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for Rust language support for Visual Studio Code

Install [WebAssembly](https://marketplace.visualstudio.com/items?itemName=dtsvet.vscode-wasm) Toolkit for VSCode

<br>
In VSCode `Ctrl+p`

`ext install rust-lang.rust-analyzer`

`ext install dtsvet.vscode-wasm`

<br>

---