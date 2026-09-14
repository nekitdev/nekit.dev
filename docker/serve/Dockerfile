FROM rust:slim AS chef

# install `cargo-chef` for caching dependencies
RUN cargo install cargo-chef

WORKDIR /app

FROM chef AS planner

COPY . .

# generate the recipe for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# build dependencies based on the recipe
RUN cargo chef cook --release --bin nekit-serve --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin nekit-serve

FROM debian:trixie-slim

COPY --from=builder /app/target/release/nekit-serve .

# copy the static directory to be served
COPY --from=builder /app/static ./static

EXPOSE 6913

ENTRYPOINT ["./nekit-serve"]
