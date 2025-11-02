################################################################################
# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
################################################################################

FROM mcr.microsoft.com/devcontainers/rust:latest
ARG USERNAME=vscode
ARG TARGETARCH

RUN apt-get update && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install \
    curl \
    gcc \
    git \
    zip \
    build-essential \
    protobuf-compiler \
    && \
    rm -rf /var/lib/apt/lists/*

# docker in docker
# also make sure the /var/run/docker.sock:/var/run/docker.sock mount is set in devcontainer.json
RUN curl -fsSL https://get.docker.com | sh

USER ${USERNAME}
