FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Planner
# Caches dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin server

# TODO: Use main repo / convert to install binaries
RUN cargo install --git https://github.com/DogeDark/dioxus --branch cli-raw-out dioxus-cli --root /.cargo

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
ENV PORT=3000
ENV BUILD_TEMPLATE_PATH="/usr/local/bin/template"
EXPOSE 3000

ENTRYPOINT [ "/usr/local/bin/server" ]