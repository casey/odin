watch:
	cargo watch --clear --exec test

minimal-versions:
	cargo +nightly generate-lockfile -Z minimal-versions

install-default-config:
	cp odin.yaml ~/.config/odin.yaml

injstall:
	cargo install --force --path .

clippy:
	cargo clippy
