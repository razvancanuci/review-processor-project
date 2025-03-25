FROM rust:1.85.0-slim-bullseye AS build

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates
    
COPY . .

RUN cargo build --release

FROM build AS test
RUN cargo test --release

FROM build AS clippy
RUN cargo clippy -- -D warnings

FROM build AS run

EXPOSE 8000
CMD ["./target/release/review-processor-backend"]
