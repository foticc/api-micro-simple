# syntax=docker/dockerfile:1

# Actix + SeaORM（PostgreSQL / rustls），release 静态链接到 glibc，运行时用 slim 镜像。
FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release

FROM debian:bookworm-slim AS final

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api-micro-simple /app/api-micro-simple

# 与 docker-compose 端口映射一致；数据库与密钥须通过环境变量或编排注入（勿写入镜像）
ENV HOST=0.0.0.0 \
    PORT=3000

USER nobody:nogroup

EXPOSE 3000

CMD ["/app/api-micro-simple"]