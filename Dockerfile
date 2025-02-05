FROM rust:1-bookworm AS chef

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef --no-confirm

WORKDIR /app

# Planner
# Caches dependencies
FROM chef AS planner
COPY . .

RUN cargo binstall dioxus-cli --root /.cargo --no-confirm
RUN cargo chef prepare --recipe-path recipe.json --bin server

# Builder
# Builds binary and imports cached deps from planner
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin server
COPY . .
RUN cargo build --release --bin server

# Pre-slim runtime
FROM rust:1-slim-bookworm AS pre-runtime

# Install openssl
RUN set -ex; \
    apt-get update -y ; \
    apt-get install -y --no-install-recommends \
    openssl

# Finalize as runner
FROM pre-runtime AS runtime

COPY --from=builder /app/target/release/server /usr/local/bin
COPY --from=builder /app/packages/playground/server/template /usr/local/bin/template
COPY --from=planner /.cargo/bin/dx /usr/local/bin
RUN MKDIR /app/temp

ENV PATH="${PATH}:/usr/local/bin"
ENV PORT=8080
ENV BUILD_TEMPLATE_PATH="/usr/local/bin/template"
EXPOSE 8080

ENTRYPOINT [ "/usr/local/bin/server" ]
