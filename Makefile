.PHONY: test
test:
	cargo test --manifest-path=ch01/hello_cargo/Cargo.toml
	cargo test --manifest-path=ch02/guessing_game/Cargo.toml
