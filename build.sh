#!/bin/bash
set -e

echo "Building voxweave project..."
cargo build --release

echo "Build complete! Testing..."
cargo test

echo "All tests passed!"
#!/bin/bash
set -e

echo "Building voxweave project..."
cargo build --release

echo "Build complete! Testing..."
cargo test

echo "All tests passed!"
