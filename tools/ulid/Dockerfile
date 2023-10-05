FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye AS ulid_tool
WORKDIR /app
RUN git clone https://github.com/PHACDataHub/rust-tools
WORKDIR /app/rust-tools/tools/ulid
RUN cargo build --release