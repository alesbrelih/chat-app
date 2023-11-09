FROM rust:1.73.0-bullseye as builder

WORKDIR /app

COPY ./code/backend/chat-app .

RUN cargo build

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/debug/chat-app /app

EXPOSE 8080

CMD ["/app"]
