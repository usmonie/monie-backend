FROM rust:1.64.0-alpine as build

RUN apk add --no-cache musl-dev
WORKDIR /opt/monie

# copy over your manifests
#COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

RUN rm ./target/release/deps/monie*
RUN cargo build --release

FROM rust:1.64.0-alpine
COPY --from=build /monie/target/release/monie .


CMD ["./monie"]