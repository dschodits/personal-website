# frontend

The frontend to my personal Website/Webblog. A combination of Vue3, Tailwind and Vite. This was jsut a learning project for me so don't judge the code too hard.


## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Compile and Minify for Production

```sh
npm run build
```
### Add dockerization
```sh
docker build . -t website/frontend:1.0
docker run -d -p 80:80 website/frontend:1.0
```

### Credits
Original Template from [Vue3 Docs](https://vuejs.org/guide/quick-start.html).