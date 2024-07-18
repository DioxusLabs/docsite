FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Planner
# Caches dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin server

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo --no-confirm

# Builder
# Builds binary and imports cached deps from planner
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin server
COPY . .
RUN cargo build --release --bin server

# Finalize with slim runtime
FROM chef AS runtime

COPY --from=builder /app/target/release/server /usr/local/bin
COPY --from=builder /app/server/template /usr/local/bin/template
COPY --from=planner /.cargo/bin/dx /usr/local/bin
#COPY --from=planner /app/dx-debian /usr/local/bin/dx

ENV PATH="${PATH}:/usr/local/bin"
ENV PORT=8080
ENV BUILD_TEMPLATE_PATH="/usr/local/bin/template"
EXPOSE 8080

ENTRYPOINT [ "/usr/local/bin/server" ]
