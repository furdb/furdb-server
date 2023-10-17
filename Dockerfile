ARG RUST_VERSION=1.72.0
ARG APP_NAME=furdb-server

# Build
FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

# Final
FROM debian:12 AS final
COPY --from=build /bin/server /bin/
RUN mkdir /furdb
EXPOSE 8080

CMD ["/bin/server"]
