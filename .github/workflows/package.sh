#! /bin/bash
# .github/workflows/package.sh 20241110
DATE=$1
VERSION="0.0.0-$DATE"
rm -rf target/package
sed -i "/\[workspace.package\]/,/version/ s/version = \"[^\"]*\"/version = \"$VERSION\"/" Cargo.toml
cd macros
cargo package --allow-dirty --no-verify
cd ..
sed -i 's|liberty-macros = { path = "macros" }|liberty-macros = { path = "macros", version = "'"$VERSION"'" }|' Cargo.toml
cargo package --allow-dirty --no-verify
sed -i 's|liberty-macros = { path = "macros", version = "'"$VERSION"'" }|liberty-macros = { path = "macros" }|' Cargo.toml