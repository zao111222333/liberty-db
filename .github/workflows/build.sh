set -euo pipefail
cargo fmt --all -- --check
cargo check --all-features
cargo build --verbose
cargo test --verbose --all-features
cargo test --verbose --all-features --package dev
cargo clippy --all-features
cargo tarpaulin --verbose --lib --examples --workspace --exclude dev liberty-macros --all-features --out xml html --output-dir target/codecov