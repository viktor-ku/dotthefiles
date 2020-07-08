all: build

clean:
	cargo clean

clean-all: clean
	rm -rf osxcross

setup-osx:
	rustup target add x86_64-apple-darwin
	./scripts/osxcross-setup.sh

build-linux:
	cargo build --release

build-osx: setup-osx
	PATH=osxcross/target/bin:$$PATH cargo build --target x86_64-apple-darwin --release

build: build-linux build-osx

install: install-linux

install-linux:
	sudo cp -f target/release/envkit /usr/local/bin/envkit

install-osx:
	sudo cp -f target/x86_64-apple-darwin/release/envkit /usr/local/bin/envkit

uninstall:
	sudo rm -f /usr/local/bin/envkit
