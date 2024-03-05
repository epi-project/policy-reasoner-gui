# DOCKERFILE for the POLICY REASONER GUI
#   by Tim Müller
# 
# Builds both the client webserver and Rust backend as containers.
# 


##### BUILD CONTAINER #####
FROM rust:1-alpine3.19 AS backend-build

# Install additional dependencies
RUN apk add --no-cache \
    g++ make cmake \
    musl-dev pkgconf openssl-dev

# Copy the source files
RUN mkdir -p /build
COPY src /build/src
COPY build.rs /build/build.rs
COPY Cargo.toml /build/Cargo.toml
COPY Cargo.lock /build/Cargo.lock

# Run cargo, with caches
WORKDIR /build
RUN --mount=type=cache,id=cargoidx,target=/usr/local/cargo/registry \
    --mount=type=cache,id=policyreasonerguicache,target=/build/target \
    cargo build --release --target x86_64-unknown-linux-musl \
 && cp /build/target/x86_64-unknown-linux-musl/release/policy-reasoner-client-backend /policy-reasoner-client-backend





##### BACKEND #####
FROM alpine:3.19 AS backend

# Define some build args
ARG UID=1000
ARG GID=1000

# # Install additional dependencies
# RUN apk add --no-cache libc6-compat libgcc

# Setup a user mirroring the main one
RUN addgroup -g $GID amy
RUN adduser -u $UID -G amy -g Amy -D amy

# Copy the binary
COPY --chown=amy:amy --from=backend-build /policy-reasoner-client-backend /policy-reasoner-client-backend

# Run it
USER amy
WORKDIR /home/amy
ENTRYPOINT [ "/policy-reasoner-client-backend" ]





##### CLIENT #####
FROM alpine:3.19 AS client

# Define some build args
ARG UID=1000
ARG GID=1000

# Setup a user mirroring the main one
RUN addgroup -g $GID amy
RUN adduser -u $UID -G amy -g Amy -D amy

# Install additional dependencies
RUN apk add --no-cache npm

# Copy the source files
COPY --chown=amy:amy client /home/amy/client

# Install node packages
USER amy
WORKDIR /home/amy/client
RUN npm i parcel @parcel/transformer-sass --save-dev \
 && npm cache clean --force

# Run the thing
ENTRYPOINT [ "npx", "parcel" ]
