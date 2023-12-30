PROJ = mdsplode
BIN_DIR = ./bin
BIN = target/release/$(PROJ)

default: all

all: clean deps build lint test

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

build: $(BIN_DIR)
	@cargo build --release
	@rm -f $(BIN_DIR)/*
	@cargo install --path ./ --root ./

dev-build: $(BIN_DIR)
	@cargo build --release --features logging
	@rm -f $(BIN_DIR)/*
	@cargo install --path ./ --root ./

lint:
	@cargo +nightly clippy --version
	@cargo +nightly clippy --all-targets --all-features -- --no-deps -D clippy::all

cicd-lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

check:
	@cargo deny check
	@cargo +nightly udeps

cicd-check:
	@cargo udeps

test:
	@RUST_BACKTRACE=1 cargo test

test-all: list check test

deps:
	@cargo update

publish:
	@cargo publish

tag:
	@git tag $$($(BIN_DIR)/$(PROJ) --version | cut -d " " -f 2)
	@git push --tags

release: build lint test tag publish

clean:
	@cargo clean
	@rm -f $(BIN_DIR)/$(PROJ)

clean-all: clean
	@rm .crates.toml .crates2.json Cargo.lock

fresh-all: clean-all all

fresh: clean all

nightly:
	@rustup toolchain install nightly

docs: DOCS_PATH = target/doc/$(PROJ)
docs:
	@cargo doc --all-features --no-deps --workspace
	@echo
	@echo "Docs are available here:"
	@echo " * $(DOCS_PATH)"
	@echo " * file://$(shell pwd)/$(DOCS_PATH)/index.html"
	@echo

open-docs:
	@cargo doc --all-features --no-deps --workspace --open

install-cargo-deny:
	@echo ">> Installing cargo deny ..."
	@cargo install --locked cargo-deny

setup-cargo-deny: install-cargo-deny
	@echo ">> Setting up cargo deny ..."
	@cargo deny init

install-udeps:
	@echo ">> Setting up cargo udeps ..."
	@cargo install cargo-udeps --locked

cicd-install-udeps:
	@echo ">> Setting up cargo udeps ..."
	@cargo install cargo-udeps
