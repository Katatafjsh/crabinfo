# Builder stage
FROM rust:1.67-slim as builder
WORKDIR /usr/src/crabinfo
COPY . .
RUN cargo install --path . 

# Runtime stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/crabinfo /usr/local/bin/crabinfo

CMD ["crabinfo"]