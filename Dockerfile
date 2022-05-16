FROM node:17.8-bullseye as frontend

WORKDIR /usr/src/app
COPY frontend/package*.json ./
COPY frontend/pnpm-lock.yaml ./

RUN corepack enable && corepack prepare pnpm@7.1.0 --activate && pnpm i
COPY frontend/ .
RUN pnpm run build


FROM rust:latest as builder

RUN USER=root cargo new --bin PersonalBlogEngine
WORKDIR /PersonalBlogEngine
COPY ./Cargo.* .
RUN mkdir -p frontend/dist
COPY --from=frontend /usr/src/app/dist frontend/dist
# RUN ls -la && cat Cargo.toml && ls frontend/dist
RUN cargo build --release
RUN rm src/*.rs

COPY . ./
RUN rm ./target/release/deps/PersonalBlogEngine*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app
#RUN apt-get update \
#    && apt-get install -y ca-certificates tzdata \
#    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

#ENV TZ=Etc/UTC \
#    APP_USER=appuser

#RUN groupadd $APP_USER \
#    && useradd -g $APP_USER $APP_USER \
#    && mkdir -p ${APP}

COPY --from=builder /PersonalBlogEngine/target/release/PersonalBlogEngine /PersonalBlogEngine

#RUN chown -R $APP_USER:$APP_USER ${APP}

#USER $APP_USER
RUN apt-get update \
    && apt-get install -y libpq-dev \
    && rm -rf /var/lib/apt/lists/*
CMD ["/PersonalBlogEngine"]
