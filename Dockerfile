
## Builder
FROM rust:latest AS builder

WORKDIR /server

RUN rustup component add rustfmt

COPY dummy.rs .

COPY Cargo.toml .

RUN sed -i 's#src/server.rs#dummy.rs#' Cargo.toml

RUN cargo build --release

RUN sed -i 's#dummy.rs#src/server.rs#' Cargo.toml

RUN update-ca-certificates

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release --bin server

# --------------------------------------
# --------------------------------------

FROM debian:buster-slim as runner

RUN apt-get update && apt-get -y install ca-certificates

WORKDIR /server

USER root

# Copy our build
COPY --from=builder /server/target/release/server ./

CMD ["/server/server"]