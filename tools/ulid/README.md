
# ULID tool

This tool is used so that we don't have to rely on a call to a webservice that may not be available.  It is implemented in [Rust](https://www.rust-lang.org/).

To build the tool, you should `cd` into the `ulid` directory

```
cd /workspaces/tools/tools/ulid
```

Then run the build command for either a debug build or a release build:
```
# debug build
cargo build

# release build
cargo build --release
```

This tool is meant to be access in this devcontainer in an alias.  I have had no luck in creating an alias that works to access it in this directory path.  So, I have created a two line build script that will do a release build and copy the ulid executable into the `.devcontainer/library-scripts` directory where the Dockerfile will be able to copy it to a location that will work for an alias mapping (in the `vscode` user's directory).



