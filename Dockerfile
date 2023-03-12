# Builder stage
FROM rust:1.67-slim as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev make

# https://github.com/rust-secure-code/cargo-auditable/
RUN cargo install cargo-auditable cargo-audit
ENV cargo='cargo auditable'

WORKDIR /usr/src/crabinfo
COPY . .

RUN $cargo install --path . 

# Runtime stage
FROM debian:bullseye-slim

ARG USERNAME=ferris
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

USER $USERNAME

COPY --from=builder /usr/local/cargo/bin/crabinfo /usr/local/bin/crabinfo

HEALTHCHECK NONE

CMD ["crabinfo"]