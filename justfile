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

download-image:
    python ./scripts/download_images -c 10
    # python ./scripts/download_course_images.py

fetch:
    python ./scripts/crawler/fetch_geektime.py

make-epub course_id:
    ./scripts/convert_to_epub -c {{ course_id }}

list-course:
    just sql 'select id,title from course;' | fzf

sql sql:
    sqlite3 'file:./storage/hackbook.db?immutable=1' '{{ sql }}'
