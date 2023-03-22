FROM rust:1-alpine3.16
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/worker

FROM alpine:3.16
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 /app/target/release/worker .
ENTRYPOINT ["/worker"]