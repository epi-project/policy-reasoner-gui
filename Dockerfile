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
    openssl-dev openssl-libs-static

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



##### CLIENT BUILD CONTAINER #####
FROM alpine:3.19 AS client-build

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
RUN npm i

# Build the backend files itself
RUN BACKEND_ADDR="/api" npx parcel build



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

# Copy the binary
COPY --chown=amy:amy --from=backend-build /policy-reasoner-client-backend /home/amy/policy-reasoner-client-backend
# # Copy the webapp files
COPY --chown=amy:amy --from=client-build /home/amy/client/dist /home/amy/client

# Run it
USER amy
WORKDIR /home/amy
ENTRYPOINT [ "/home/amy/policy-reasoner-client-backend" ]
