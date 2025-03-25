.PHONY: - help
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo
more:## 	more help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/	/'
	#$(MAKE) -f Makefile help

all: bin
bin:
	cargo b --manifest-path Cargo.toml
##
##===============================================================================
##make cargo-*
cargo-help:### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
cargo-release-all:### 	cargo-release-all
## 	cargo-release-all 	recursively cargo build --release
	for t in */Cargo.toml;  do echo $$t; cargo b -r -vv --manifest-path $$t; done
	for t in ffi/*/Cargo.toml;  do echo $$t; cargo b -r -vv --manifest-path $$t; done
cargo-clean-release:### 	cargo-clean-all - clean release artifacts
## 	cargo-clean-all 	recursively cargo clean --release
	for t in *Cargo.toml;  do echo $$t && cargo clean --release -vv --manifest-path $$t 2>/dev/null; done
cargo-publish-all:### 	cargo-publish-all
## 	cargo-clean-all 	recursively publish rust projects
	for t in *Cargo.toml;  do echo $$t; cargo publish -vv --manifest-path $$t; done

cargo-install-bins:### 	cargo-install-bins
## 	cargo-install-all 	recursively cargo install -vv $(SUBMODULES)
## 	*** cargo install -vv --force is NOT used.
## 	*** cargo install -vv --force --path <path>
## 	*** to overwrite deploy cargo.io crates.
	export RUSTFLAGS=-Awarning;  for t in $(SUBMODULES); do echo $$t; cargo install --bins --path  $$t -vv 2>/dev/null || echo ""; done
	#for t in $(SUBMODULES); do echo $$t; cargo install -vv gnostr-$$t --force || echo ""; done

cargo-b:cargo-build### 	cargo b
cargo-build:### 	cargo build
## 	cargo-build q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET)
cargo-i:cargo-install
cargo-install:### 	cargo install --path jj
	@. $(HOME)/.cargo/env
	@cargo install --path . $(FORCE)
cargo-br:cargo-build-release### 	cargo-br
## 	cargo-br q=true
cargo-build-release:### 	cargo-build-release
## 	cargo-build-release q=true
	@. $(HOME)/.cargo/env
	@cargo b --release $(QUIET)
cargo-c:cargo-check
cargo-check:### 	cargo-check
	@. $(HOME)/.cargo/env
	@cargo c
cargo-bench:### 	cargo-bench
	@. $(HOME)/.cargo/env
	@cargo bench
cargo-t:cargo-test
cargo-test:### 	cargo-test
	@. $(HOME)/.cargo/env
	#@cargo test
	cargo test -p rust-project-template utils::pathing::tests::test_unix_config_path
cargo-report:### 	cargo-report
	@. $(HOME)/.cargo/env
	cargo report future-incompatibilities --id 1

##===============================================================================
cargo-dist:### 	cargo-dist -h
	cargo dist -h
cargo-dist-build:### 	cargo-dist-build
	RUSTFLAGS="--cfg tokio_unstable" cargo dist build
cargo-dist-manifest-global:### 	cargo dist manifest --artifacts=all
	cargo dist manifest --artifacts=all

# vim: set noexpandtab:
# vim: set setfiletype make
