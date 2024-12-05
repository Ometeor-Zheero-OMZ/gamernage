FROM node:18-alpine

WORKDIR /frontend

COPY ./frontend/package*.json ./

RUN npm install

COPY ./frontend .

RUN npm run build

RUN npm install -g serve

CMD ["serve", "-s", "out"]
