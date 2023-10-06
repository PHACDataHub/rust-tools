
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye AS main_devcontainer

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# DO STUFF AS ROOT HERE

# Switch to non-root user
USER ${USERNAME}

# this adds a default flag to cargo so that it intiates rust proejcts with no
# version control (since we are inside a GitHub repo already)
# "cargo new <name>" will not init a default repo.  Just a clean base project.
RUN ["/bin/bash", "-c", "mkdir $HOME/.cargo"]
RUN ["/bin/bash", "-c", "touch $HOME/.cargo/config.toml"]  
RUN ["/bin/bash", "-c", "echo \"[cargo-new]\" >> $HOME/.cargo/config.toml"]
RUN ["/bin/bash", "-c", "echo 'vcs = \"none\"' >> $HOME/.cargo/config.toml"]