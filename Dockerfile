FROM alpine:3.12
COPY ./target/release/hackbook-server .
CMD ["./hackbook-server"]