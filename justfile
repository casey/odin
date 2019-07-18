watch:
	cargo watch --clear --exec test

minimal-versions:
	cargo +nightly generate-lockfile -Z minimal-versions
