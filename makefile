
run-default:
	cp Cargo-default.toml Cargo.toml
	cargo run

run-patch:
	cp Cargo-patch.toml Cargo.toml
	cargo run

verify:
	curl -s -i --head http://localhost:3000 | grep content-length
