FROM clux/muslrust:nightly AS chef
USER root
RUN cargo install cargo-chef
RUN apt update && apt install mold
WORKDIR /dream

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /dream/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin dream

FROM alpine AS prep
RUN apk add libressl-dev
RUN addgroup -S myuser && adduser -S myuser -G myuser

FROM prep AS runtime
COPY --from=builder /dream/target/x86_64-unknown-linux-musl/release/dream /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/dream"]