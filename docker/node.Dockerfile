FROM node:18.16.0-alpine

COPY .npmrc .npmrc
COPY package.json package.json

RUN npm install

COPY src src

CMD ["node", "src/server.js"]