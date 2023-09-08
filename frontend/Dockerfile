FROM node:18.16.0-alpine

EXPOSE 3000

WORKDIR /

COPY package.json pnpm-lock.yaml /
COPY . /

ENV NEXT_PUBLIC_SERVER_URL ${SERVER_URL}

RUN npm i -g pnpm

RUN pnpm install
RUN pnpm build
ENTRYPOINT ["pnpm", "start"]
