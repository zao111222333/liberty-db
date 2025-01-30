#! /bin/bash
set -euo pipefail

BASE_URL=$1
INCOMING_URL=$2
sed -i 's|^liberty-db-incoming|## liberty-db-incoming|' dev/Cargo.toml
sed -i 's|^# liberty-db-incoming|liberty-db-incoming|' dev/Cargo.toml
cat dev/Cargo.toml # debug
sed -i "s|BASE_URL|${BASE_URL}|" dev/src/projs.rs
sed -i "s|INCOMING_URL|${INCOMING_URL}|" dev/src/projs.rs
cargo bench --package dev --bench compare --features compare
sed -i 's|^liberty-db-incoming|# liberty-db-incoming|' dev/Cargo.toml
sed -i 's|^## liberty-db-incoming|liberty-db-incoming|' dev/Cargo.toml