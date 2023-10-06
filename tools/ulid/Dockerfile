FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye AS ulid_tool
WORKDIR /app
RUN git clone https://github.com/PHACDataHub/rust-tools
WORKDIR /app/rust-tools/tools/ulid
RUN cargo build --release



# This will copy the build from the ulid_tool step to the $HOME directory of whatever 
# user context we are running as. The `sudo` command is needed if the user context is
# not running as root. You can change the 
COPY --from=ulid_tool /app/rust-tools/tools/ulid/target/release/ulid /tmp/ulid
RUN ["/bin/bash", "-c", "sudo cp /tmp/ulid $HOME/ulid"]