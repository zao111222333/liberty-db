cp docs/header.html liberty-db/header.html
cargo publish --allow-dirty -p liberty-db
rm liberty-db/header.html