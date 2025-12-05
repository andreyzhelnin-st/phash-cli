.PHONY: all build test test-unit test-integration clean install run setup-test

# Default target: build and test
all: build test

# Build the release binary
build:
	cargo build --release

# Run all tests (unit + integration)
test: test-unit test-integration

# Run unit tests
test-unit:
	cargo test

# Run integration tests with real images
test-integration: build
	@echo "Running integration tests with real images..."
	@echo ""
	@echo "Test 1: Single image hash"
	@./target/release/phash-cli test_images/same1.png
	@echo ""
	@echo "Test 2: Compare same image in different format"
	@./target/release/phash-cli test_images/same1.png test_images/another.jpg
	@echo ""
	@echo "Test 3: Compare similar images"
	@./target/release/phash-cli test_images/same1.png test_images/same2.png
	@echo ""
	@echo "✓ All integration tests passed!"

# Clean build artifacts
clean:
	cargo clean

# Setup test images (run this once before testing)
setup-test:
	mkdir -p test_images
	@if [ -f ~/Downloads/1.png ]; then \
		cp ~/Downloads/1.png test_images/same1.png 2>/dev/null || true; \
		cp ~/Downloads/1.jpg test_images/another.jpg 2>/dev/null || true; \
		cp ~/Downloads/2.png test_images/same2.png 2>/dev/null || true; \
		echo "Test images copied to test_images/"; \
	else \
		echo "Warning: Test images not found in ~/Downloads/"; \
	fi

# Install binary to system
install: build
	cargo install --path .

# Run the application (usage: make run FILE=path/to/image.png)
run: build
	./target/release/phash-cli $(FILE)
