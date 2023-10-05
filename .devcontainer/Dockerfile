
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
#ARG SOURCE_SOCKET=/var/run/docker-host.sock
#ARG TARGET_SOCKET=/var/run/docker.sock


# Do stuff as root here

# Switch to non-root user
USER ${USERNAME}
#RUN echo $HOME
# this adds a default flag to cargo so that it intiates rust proejcts with no
# version control (since we are inside a GitHub repo already)
# "cargo new <name>" will not init a default repo.  Just a clean base project.
# RUN touch $HOME/.cargo/config.toml
RUN ["/bin/bash", "-c", "mkdir $HOME/.cargo"]
RUN ["/bin/bash", "-c", "touch $HOME/.cargo/config.toml"]  
RUN ["/bin/bash", "-c", "echo \"[cargo-new]\" >> $HOME/.cargo/config.toml"]
RUN ["/bin/bash", "-c", "echo 'vcs = \"none\"' >> $HOME/.cargo/config.toml"]