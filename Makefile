CARGO_MANIFESTS=./Cargo.toml $(wildcard ./lang/**/Cargo.toml)
TEST_PROV=cargo test
TEST_PROV_ARGS=--manifest-path

# nextest doesn't test all of the crates natively
test:
	echo Testing all crates
	$(foreach file, $(CARGO_MANIFESTS), $(TEST_PROV) $(TEST_PROV_ARGS) $(file);)
