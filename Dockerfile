FROM rust:1-alpine3.13 AS build

WORKDIR /usr/src
RUN mkdir src && echo "fn main() {}" > src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo install --locked --path . && rm -rf target/build

COPY src ./src
RUN cargo build --release

FROM alpine:3 AS user

RUN echo "rugrep:x:10001:rugrep" >> /tmp/group
RUN echo "rugrep:x:10001:10001::/:/dev/null" >> /tmp/passwd

FROM scratch

COPY --from=user /tmp/passwd /etc/passwd
COPY --from=user /tmp/group /etc/group
COPY --from=build --chown=rugrep:rugrep /usr/src/target/release/rugrep /usr/local/bin/rugrep

USER rugrep:rugrep
ENTRYPOINT [ "/usr/local/bin/rugrep" ]
