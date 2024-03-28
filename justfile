clean:
    cargo clean

format:
    cargo fmt

lint:
    cargo check
    cargo clippy

doc:
    cargo doc --document-private-items

bench:
    cargo bench

update:
    nix flake update
    cargo update

build *args: lint format
    cargo build {{args}}

test *args: lint format
    cargo nextest run --lib
