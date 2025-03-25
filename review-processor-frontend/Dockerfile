FROM node:22-alpine AS build

WORKDIR /app

RUN npm install -g @quasar/cli

COPY . .

RUN npm install

RUN quasar build

FROM nginx:stable AS runner

COPY --from=build /app/dist/spa /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80 443

CMD ["nginx", "-g", "daemon off;"]