# phash-cli

A command-line tool for generating perceptual hashes (pHash) of images and comparing their similarity.

## Features

- Generate perceptual hashes for images using DCT-based algorithm
- Compare two images and calculate their hash distance
- Support for multiple image formats: PNG, JPEG, GIF, WebP, BMP
- Output hashes in hexadecimal format

## Building

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))

### Build from source

```bash
cargo build --release
```

The binary will be available at `./target/release/phash-cli`

### Using Makefile

The project includes a Makefile for convenient building and testing:

```bash
# Build and run all tests (default)
make

# Just build the binary
make build

# Setup test images (needed for integration tests)
make setup-test

# Run all tests (unit + integration)
make test

# Run only unit tests
make test-unit

# Run only integration tests
make test-integration

# Clean build artifacts
make clean

# Install globally
make install
```

**Note:** Run `make setup-test` once to copy test images before running integration tests.

## Testing

The project includes comprehensive testing:

### Unit Tests

Run the unit test suite:
```bash
cargo test
# or
make test-unit
```

### Integration Tests

Integration tests use real images from the `test_images/` directory:
```bash
make test-integration
```

These tests verify:
- Single image hash generation
- Comparing images in different formats
- Distance calculation between different images

## Usage

### Generate hash for a single image

```bash
./target/release/phash-cli <image_path>
```

**Example:**
```bash
./target/release/phash-cli ~/Downloads/image.png
```

**Output:**
```
~/Downloads/image.png: 010072f67e180000
```

### Compare two images

```bash
./target/release/phash-cli <image1_path> <image2_path>
```

**Example:**
```bash
./target/release/phash-cli ~/Downloads/1.png ~/Downloads/1.jpg
```

**Output:**
```
/Users/andreyzhelnin/Downloads/1.png: 010072f67e180000
/Users/andreyzhelnin/Downloads/1.jpg: 7cdade3278e0e028
distance: 30
```

### Understanding Distance

The distance represents the Hamming distance between two perceptual hashes:
- **0**: Images are identical or perceptually the same
- **1-10**: Very similar images (minor edits, compression)
- **11-20**: Similar images (cropping, color adjustments)
- **21-30**: Moderately similar images
- **30+**: Different images

## Installation

To install globally:

```bash
cargo install --path .
```

Then you can use it from anywhere:

```bash
phash-cli <image_path>
```

## How it works

This tool uses the Mean hash algorithm with an 8x8 DCT (Discrete Cosine Transform) hash size, which provides a good balance between accuracy and performance for perceptual image comparison.

## License

See LICENSE file for details.
