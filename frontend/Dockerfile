FROM node:latest as build-stage
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY ./ .
RUN npm run build

FROM caddy as production-stage
RUN mkdir /app
COPY --from=build-stage /app/dist /app
COPY Caddyfile /etc/caddy/Caddyfile
#COPY nginx/default.conf /etc/nginx/nginx.conf
