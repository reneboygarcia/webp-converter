# Makefile for webp-converter
# Usage: make <target>

.PHONY: help install test sca sbom pre-merge clean

# Help target to display available commands
help:
	@echo "webp-converter Development Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  install    - Create virtualenv and install dependencies"
	@echo "  test       - Run all unit tests"
	@echo "  sca        - Run Trivy vulnerability scan (CRITICAL,HIGH; exit 1 if found)"
	@echo "  sbom       - Generate CycloneDX SBOM (Trivy) to sbom.cyclonedx.json"
	@echo "  pre-merge  - Run quality gates: test and sca"
	@echo "  clean      - Clean build artifacts and temporary files"
	@echo ""

install:
	python3 -m venv venv
	./venv/bin/pip install -r requirements.txt
	./venv/bin/pip install -e .

test:
	./venv/bin/python -m unittest discover -s tests

sca:
	@echo "🔍 Running Trivy vulnerability scan (CRITICAL,HIGH)..."
	@if ! command -v trivy >/dev/null 2>&1; then \
		echo "⚠️  Trivy is not installed. Install it with: brew install trivy"; exit 1; \
	fi
	trivy fs --scanners vuln --severity CRITICAL,HIGH --exit-code 1 .

sbom:
	@echo "📋 Generating CycloneDX SBOM..."
	@if ! command -v trivy >/dev/null 2>&1; then \
		echo "⚠️  Trivy is not installed. Install it with: brew install trivy"; exit 1; \
	fi
	trivy fs -f cyclonedx -o sbom.cyclonedx.json .
	@echo "✅ SBOM written to sbom.cyclonedx.json"

# When SKIP_SCA=1, skip Trivy (e.g. local Docker/Trivy DB issues).
pre-merge: test $(if $(SKIP_SCA),,sca)
	@echo "✅ All pre-merge checks passed!"

clean:
	rm -rf build/ dist/ *.egg-info webp_converter.egg-info
	rm -f sbom.cyclonedx.json
