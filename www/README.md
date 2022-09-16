# **Development**

## **Run the development environment localy -require full instalation- with:**

```
make dev
```

Make will run a script that will serve the current dev version of this repo

---

### **How to prepare a production environment**

<br>

Within WWW folder, Initialize a Node project with:
<br>

```
npm init -y
```
<br>

Install necessary packages:
```
npm install --save webpack
npm install --save webpack-cli
npm install --save-dev webpack-dev-server
npm install --save copy-webpack-plugin
npm install --save typescript ts-loader

```
<br>

---

## **[WebPack](https://webpack.js.org/concepts/)**

At its core, webpack is a static module bundler for modern JavaScript applications. When webpack processes your application, it internally builds a dependency graph from one or more entry points and then combines every module your project needs into one or more bundles, which are static assets to serve your content from.

<br>

---
## **Documentation for [webpack with TS](https://webpack.js.org/guides/typescript/)**

<br>

---

## **Node scripts**
<br>

run development server:
```
npm run dev
```

build files:
```
npm run build
```
