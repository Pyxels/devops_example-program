FROM rust as builder
# Target wird für das statische kompilieren benötigt
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/devops-webserver /server
COPY --from=builder /app/Rocket.toml .

EXPOSE 8000
VOLUME /data/

CMD ["/server"]
