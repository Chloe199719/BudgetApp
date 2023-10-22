FROM lukemathwalker/cargo-chef:latest-rust-latest as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build our project
ENV SQLX_OFFLINE true
RUN cargo build --release --bin discord-backend
FROM debian:bookworm AS runtime
WORKDIR /app
RUN apt-get update -y \
    &&  apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord-backend discord-backend
COPY settings settings
COPY templates templates
ENV APP_ENVIRONMENT production
ENV APP_DEBUG false
ENTRYPOINT ["./discord-backend"]