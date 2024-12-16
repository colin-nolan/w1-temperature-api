##################################################
# Build stage
##################################################
FROM rust:alpine3.21 AS build

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release


##################################################
# Production service stage
##################################################
FROM alpine:3.21

COPY --from=build /app/target/release/w1-temperature-api /app/target/release/w1-temperature /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/w1-temperature-api"]
