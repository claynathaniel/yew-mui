# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.195.0/containers/rust/.devcontainer/base.Dockerfile
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
#ARG VARIANT="bullseye"
#FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}
FROM ubuntu:latest
# [Optional] Uncomment this section to install additional packages.
 RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get upgrade  -y \
     && apt-get -y install curl build-essential git
RUN useradd -m ubuntu
USER ubuntu
WORKDIR /home/ubuntu
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
#RUN [ "/bin/bash", "-c", "source /home/ubuntu/.cargo/env" ]
ENV PATH=$PATH:/home/ubuntu/.cargo/bin/
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
ENTRYPOINT ["/bin/bash"]
EXPOSE 8080