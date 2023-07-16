all: clean build

clean:
	cargo clean
	rm -f -r target

build:
	cargo build

test-build:
	cargo test --no-run

clippy:
	cargo clippy --all-features