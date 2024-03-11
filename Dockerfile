# DOCKERFILE for the POLICY REASONER GUI
#   by Tim Müller
# 
# Builds both the client webserver and Rust backend as containers.
# 


##### BACKEND BUILD CONTAINER #####
FROM rust:1-alpine3.19 AS backend-build

# Install additional dependencies
RUN apk add --no-cache \
    build-base cmake pkgconf \
    openssl-dev openssl-libs-static \
    npm

# Copy the source files
RUN mkdir -p /build
COPY client /build/client
COPY src /build/src
COPY build.rs /build/build.rs
COPY Cargo.toml /build/Cargo.toml
COPY Cargo.lock /build/Cargo.lock

# Run cargo, with caches
WORKDIR /build
RUN --mount=type=cache,id=cargoidx,target=/usr/local/cargo/registry \
    --mount=type=cache,id=policyreasonerguicache,target=/build/target \
    cargo build --release \
 && cp /build/target/release/policy-reasoner-client-backend /policy-reasoner-client-backend \
 && cp -r /build/clientbuild /clientbuild



##### BUILD EFLINT-TO-JSON BINARY #####
FROM alpine:3.19 AS eflint-to-json-build

# Install additional dependencies
RUN apk add --no-cache git go

# Pull the repo & build
RUN git clone https://github.com/epi-project/eflint-server-go /eflint-server-go \
 && cd /eflint-server-go/cmd/eflint-to-json \
 && go build . \
 && mv ./eflint-to-json /eflint-to-json



##### BACKEND #####
FROM alpine:3.19 AS backend

# Define some build args
ARG UID=1000
ARG GID=1000

# Define some environment variables
ENV CLIENT_FILES_PATH="/home/amy/client"

# Setup a user mirroring the main one
RUN addgroup -g $GID amy
RUN adduser -u $UID -G amy -g Amy -D amy

# Copy the backend binary
COPY --chown=amy:amy --from=backend-build /policy-reasoner-client-backend /home/amy/policy-reasoner-client-backend
# Copy the webapp files
COPY --chown=amy:amy --from=backend-build /build/clientbuild /home/amy/client
# Copy the eFLINT -> JSON file
COPY --chown=amy:amy --from=eflint-to-json-build /eflint-to-json /home/amy/bin/eflint-to-json

# Run it
USER amy
WORKDIR /home/amy
ENTRYPOINT [ "/home/amy/policy-reasoner-client-backend" ]
