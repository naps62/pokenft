FROM node:15.11.0-alpine3.13

WORKDIR /app

ENV PATH /app/node_modules/.bin:$PATH

# install app dependencies
COPY package.json ./
COPY yarn.lock ./
RUN yarn install

COPY . ./

CMD ["npm", "start"]
