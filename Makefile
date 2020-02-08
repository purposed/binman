TARGET := binman

.PHONY: build_debug
build_debug:
	@echo "+$@"
	cargo build

.PHONY: build_release
build_release:
	@echo "+$@"
	cargo build --release

.PHONY: install
install: build_release
	@echo "+$@"
	cp ./target/release/$(TARGET) ~/bin
