
setup:
	diesel setup
	diesel migration generate create_article

migration:
	diesel migration run

pb:
	cargo run --bin proto

image:
	cargo build --release
	docker build -t hub.lubui.com/hackbook-rust .
    docker push hub.lubui.com/hackbook-rust