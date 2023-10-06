# Rust-based tools repo

The intention of this tools repo is to follow the overall philosophy of *nix tools:  Do one thing and do it well.

If you have a Rust-based tool you would like to add to the repo.  Please submit a PR!

## Tools in this repo.
- [ULID Tool](https://github.com/PHACDataHub/rust-tools/tree/main/tools/ulid)

## Contributing to this repo.

If you have a tool you wish to contribute, please feel free to do so.  You can clone the repo to your system, or you can use a codespace (or devcontainer) here on GitHub for this repo.  The codespace for this repo has been customized and will spin up with the rust toolchain already installed.

Either way, we ask that you follow a couple standards.

### Use `cargo` to create your project.

```bash
cd /tools
cargo new --vcs=none mytoolname
```
Note: The `--vcs=none` argument is not needed if using this repo's codespace as it has been configured with that switch alread set for all `cargo new` calls.

### Please update the README.md that will be default created in your tool's directory.  
Add any relevant information to help the user build or use your tool.

### Try to follow the *nix philosophy for tools
It is simple - a tool should in prinicple do one thing and do it well.
