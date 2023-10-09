# Build stage.
FROM rust:1.67 as builder

WORKDIR /app

RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .

RUN sed -i 's#src/main_no_shuttle.rs#dummy.rs#' Cargo.toml
RUN cargo build --bin smartfit --release
RUN sed -i 's#dummy.rs#src/main_no_shuttle.rs#' Cargo.toml

COPY src src
COPY templates templates
COPY locations.json locations.json
COPY Cargo.lock .
COPY Cargo.toml .
RUN ls

RUN cargo build --bin smartfit --release

# Production stage.
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/smartfit /app/smartfit
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/locations.json /app/locations.json

WORKDIR /app
CMD ["./smartfit"]