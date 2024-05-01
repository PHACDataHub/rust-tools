
# ULID tool

This tool is used so that we don't have to rely on a call to a webservice that may not be available.  It is implemented in [Rust](https://www.rust-lang.org/).

## Compiling the tool

This tool is written in Rust.  An assumption is being made that you have rust installed.

Clone a copy of the repo.  If you are only going to be doing a compile of the tool and dicarding the repo you clone afterwards you can do a shallow clone.

```bash
## EXAMPLES:
## Example 1:
# clone the `ulid-v1.0` release (assuming you want to compile against a specific version)
# `--depth 1` cause a shallow clone (no history of the repo and it will run headless)
git clone -b 'ulid-v1.0' --depth 1 https://github.com/PHACDataHub/rust-tools

## Example 2:
# alteratively you can do a clone against the latest:
git clone --depth 1 https://github.com/PHACDataHub/rust-tools

# you can omit `--depth 1` if you want the full repo (i.e. all the history) - if you want to contribute
# to the repo for example.

```

To build the tool, you should `cd` into the `ulid` directory

```bash
cd /workspaces/tools/tools/ulid
```

Then run the build command for either a debug build or a release build:
```bash
# debug build
cargo build

# release build
cargo build --release
```

## Building this tool from a Dockerfile to be used in a container (part of multi-stage build).

You can actually build this ulid tool to be used in a container quite easily using a multi-stage build.

```dockerfile
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye AS ulid_tool
WORKDIR /app
# specify the release tag (-b 'ulid-v1.0') so we aren't running against latest.
# --depth 1 is a shallow clone (no history) because we aren't keeping the clone
RUN git clone -b 'ulid-v1.0' --depth 1 https://github.com/PHACDataHub/rust-tools
WORKDIR /app/rust-tools/tools/ulid
RUN cargo build --release
# At this point the ulid CLI utility exists...
# Later in the Dockerfile we copy it to a location in our final container so it can be used.

# You will have at least one more stage at this point usually - something that
# builds out the container for whatever you are doing...
# eg. FROM mcr.microsoft.com/vscode/devcontainers/base:buster AS main_devcontainer
# More commands here to build out the main_devcontainer

# This will copy the build from the ulid_tool step to the $HOME directory of whatever 
# user context we are running as. The `sudo` command is needed if the user context is
# not running as root. You can change the destination ($HOME/) to anything that makes 
# sense for what you are doing.
COPY --from=ulid_tool /app/rust-tools/tools/ulid/target/release/ulid /tmp/ulid
RUN ["/bin/bash", "-c", "sudo cp /tmp/ulid $HOME/ulid"]
```

