# Multi-stage Dockerfile for building and running the Review Processor backend
#
# This Dockerfile uses multiple stages to:
# 1. Build the application
# 2. Run tests
# 3. Run clippy checks
# 4. Create the final production image

# Build stage - compiles the application
FROM rust:1.85.0-slim-bullseye AS build

# Install required system dependencies
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
