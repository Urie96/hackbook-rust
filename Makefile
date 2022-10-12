.PHONY: image setup migration pb

setup:
	diesel setup
	diesel migration generate create_article

migration:
	diesel migration run

pb:
	cargo run --bin proto

image:
	docker build -t hub.lubui.com/hackbook-rust .
	docker push hub.lubui.com/hackbook-rust
