FROM rust:1.64.0-slim-buster as builder

RUN cargo new --bin hackbook-server \
    && sed -i "s@http://deb.debian.org/debian-security@https://mirrors.tuna.tsinghua.edu.cn/debian-security@g" /etc/apt/sources.list \
    && sed -i "s@http://deb.debian.org/debian@https://mirrors.tuna.tsinghua.edu.cn/debian/@g" /etc/apt/sources.list \
    && apt update \
    && apt install -y libssl-dev pkg-config default-libmysqlclient-dev

WORKDIR /hackbook-server
COPY ./.cargo ./.cargo
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm src/*.rs && rm ./target/release/deps/hackbook_server*
COPY ./src ./src
RUN cargo build --release


FROM debian:buster-slim

# COPY --from=builder /usr/lib/x86_64-linux-gnu/ /usr/lib/x86_64-linux-gnu/
RUN sed -i "s@http://deb.debian.org/debian-security@http://mirrors.tuna.tsinghua.edu.cn/debian-security@g" /etc/apt/sources.list \
    && sed -i "s@http://deb.debian.org/debian@http://mirrors.tuna.tsinghua.edu.cn/debian/@g" /etc/apt/sources.list \
    && apt update \
    && apt install -y libssl-dev default-libmysqlclient-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* 

COPY --from=builder /hackbook-server/target/release/hackbook-server .

CMD ["./hackbook-server"]
