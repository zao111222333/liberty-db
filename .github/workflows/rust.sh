set -euo pipefail
cargo fmt --all -- --check
cargo check --all-features
cargo build --verbose
cargo test --verbose --all-features
cargo clippy --all-features