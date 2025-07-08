#!/bin/bash

echo "🚀 Terminator-Dancer MVP Demo"
echo "============================="
echo ""
echo "Building the project..."
cargo build --release

echo ""
echo "Running the demo..."
cargo run --example demo

echo ""
echo "Running unit tests..."
cargo test

echo ""
echo "✅ Demo completed!"
echo ""
echo "Next steps:"
echo "1. Review the code structure in src/"
echo "2. Check out the demo implementation in examples/demo.rs"
echo "3. Contribute to the project on GitHub"
echo "4. Extend the runtime with additional Solana features"
