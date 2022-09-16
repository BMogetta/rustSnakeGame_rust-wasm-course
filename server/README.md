# **Production**

## **Run the production environment localy -require full instalation- with:**

```
make prod
```

Make will run a script that will serve the current production version of this repo

---

### **How to prepare a production environment**
<br>
Within root folder, Initialize a Node project with:

```
npm init -y
```
<br>

Install necessary packages:
```
npm install --save express
npm install --save compression
```
<br>

Prepare the scripts:

```
"scripts": {
    "prebuild": "npm install --prefix www",
    "build": "npm run build --prefix www",
    "start": "node server/index.js"
  },
```

---

## **[Express](https://expressjs.com/en/5x/api.html) documentation**

Fast, unopinionated, minimalist web framework for Node.js

---
## **[Compression](https://github.com/expressjs/compression)**

Node.js compression middleware.