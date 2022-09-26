# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.195.0/containers/rust/.devcontainer/base.Dockerfile
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
 RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get upgrade  -y
#     && apt-get -y install --no-install-recommends <your-package-list-here>
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
COPY entrypoint.sh /
RUN chmod +x entrypoint.sh
ENTRYPOINT [ "entrypoint.sh" ]
EXPOSE 8080