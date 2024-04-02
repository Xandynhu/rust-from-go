.PHONY: build
build:
	@cargo build --release
	@cp target/release/librust_from_go.a lib/
	go build main.go
	@mkdir -p bin && mv main bin/