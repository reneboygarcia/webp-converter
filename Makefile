.PHONY: help build release install test format lint benchmark sca sbom pre-merge clean

help:
	@echo "webp-converter Development Makefile (Rust)"
	@echo ""
	@echo "  build      cargo build (debug)"
	@echo "  release    cargo build --release"
	@echo "  install    cargo install --path ."
	@echo "  test       cargo test"
	@echo "  format     cargo fmt"
	@echo "  lint       cargo clippy --all-targets -- -D warnings"
	@echo "  benchmark  build release binary + run tests/benchmark.py"
	@echo "  sca        trivy vulnerability scan (CRITICAL,HIGH)"
	@echo "  sbom       generate CycloneDX SBOM to sbom.cyclonedx.json"
	@echo "  pre-merge  test + format check + lint + sca"
	@echo "  clean      remove build artifacts"

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy --all-targets -- -D warnings

benchmark: release
	@command -v python3 >/dev/null 2>&1 || (echo "python3 required for benchmark" && exit 1)
	python3 tests/benchmark.py

sca:
	@echo "Running Trivy vulnerability scan..."
	@command -v trivy >/dev/null 2>&1 || (echo "Install trivy: brew install trivy" && exit 1)
	trivy fs --scanners vuln --severity CRITICAL,HIGH --exit-code 1 .

sbom:
	@command -v trivy >/dev/null 2>&1 || (echo "Install trivy: brew install trivy" && exit 1)
	trivy fs -f cyclonedx -o sbom.cyclonedx.json .
	@echo "SBOM written to sbom.cyclonedx.json"

pre-merge: test
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	$(if $(SKIP_SCA),,$(MAKE) sca)
	@echo "All pre-merge checks passed!"

clean:
	cargo clean
	rm -f sbom.cyclonedx.json
