# syntax=docker/dockerfile:1

FROM rust AS build-stage
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {println!(\"Preparing dependency cache...\")}" > src/main.rs && \
    cargo build --locked --release
RUN rm -rf src/
COPY ./src ./src
RUN cargo build --locked --release

FROM gcr.io/distroless/cc
WORKDIR /
COPY --from=build-stage /app/target/release/chinesewriter /chinesewriter
COPY ./vol ./vol
EXPOSE 3001
ENTRYPOINT ["/chinesewriter"]
