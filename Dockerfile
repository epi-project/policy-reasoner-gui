# DOCKERFILE for the POLICY REASONER GUI
#   by Tim Müller
# 
# Builds both the client webserver and Rust backend as containers.
# 


##### BUILD CONTAINER #####
FROM rust:1 AS backend-build

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    cmake \
 && rm -rf /var/lib/apt/lists/*

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
    cargo build --release \
 && cp /build/target/release/policy-reasoner-client-backend /policy-reasoner-client-backend \
 && chmod +x /policy-reasoner-client-backend





##### BACKEND #####
FROM alpine:3.19 AS backend

# Define some build args
ARG UID=1000
ARG GID=1000

# Install additional dependencies
RUN apk add --no-cache libc6-compat libgcc

# Setup a user mirroring the main one
RUN addgroup -g $GID amy
RUN adduser -u $UID -G amy -g Amy -D amy

# Copy the binary
COPY --from=backend-build /policy-reasoner-client-backend /policy-reasoner-client-backend
RUN chown amy:amy /policy-reasoner-client-backend

# Run it
USER amy
WORKDIR /home/amy
ENTRYPOINT [ "/policy-reasoner-client-backend" ]





##### CLIENT #####
FROM alpine:3.19 AS client

# Define some build args
ARG UID=1000
ARG GID=1000

# Install additional dependencies
RUN apk add --no-cache npm

# Setup a user mirroring the main one
RUN addgroup -g $GID amy
RUN adduser -u $UID -G amy -g Amy -D amy

# Copy the source files
COPY client /home/amy/client
RUN chown -R amy:amy /home/amy/client

# Install node packages
USER amy
WORKDIR /home/amy/client
RUN npm i parcel \
 && npm cache clean --force

# Run the thing
ENTRYPOINT [ "npx", "parcel" ]
