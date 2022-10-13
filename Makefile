.PHONY: image setup migration pb

setup:
	diesel setup
	diesel migration generate create_article
	diesel migration run
	rm -rf ./migrations

pb:
	cargo run --bin proto

image:
	cargo build --release
	docker build -t hub.lubui.com/hackbook-rust .
	docker push hub.lubui.com/hackbook-rust
