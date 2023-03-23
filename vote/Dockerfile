FROM node:18-alpine

WORKDIR /app

RUN apk --no-cache add curl

RUN npm install -g pnpm

COPY package.json /app/package.json
RUN pnpm install

COPY . .

EXPOSE 5151

CMD ["pnpm", "dev", "-p", "5151"]