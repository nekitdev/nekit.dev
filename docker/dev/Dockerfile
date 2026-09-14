FROM rust:slim AS chef

# install `cargo-chef` and `cargo-binstall` for caching dependencies and installing binaries
RUN cargo install cargo-chef
RUN cargo install cargo-binstall

# install `dioxus-cli` for building later
RUN cargo binstall dioxus-cli --no-confirm

WORKDIR /app

FROM chef AS planner

COPY . .

# generate the recipe for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# build dependencies based on the recipe
RUN cargo chef cook --release --package nekit-dev --recipe-path recipe.json

COPY . .

# bundle the web application
RUN dx bundle --web --release --package nekit-dev --debug-symbols false

FROM debian:trixie-slim AS runtime

# install `ca-certificates` for requests to work, and clean apt cache to reduce image size
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/dx/nekit-dev/release/web ./nekit-dev

# listen to all interfaces and the given port
ENV IP=0.0.0.0
ENV PORT=6942

EXPOSE 6942

ENTRYPOINT ["./nekit-dev/server"]
