.PHONY: run image setup migration pb

run:
	cargo run --bin hackbook-server

setup:
	diesel setup
	diesel migration generate create_article
	diesel migration run
	rm -rf ./migrations

pb:
	cargo run --bin proto

build:
	# rustup target add x86_64-unknown-linux-musl
	cargo build --release --target=x86_64-unknown-linux-musl
	# cargo build --release

image: build
	docker build -t hub.lubui.com/hackbook-rust .
	docker push hub.lubui.com/hackbook-rust
