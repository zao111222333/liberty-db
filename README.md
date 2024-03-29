# liberty-db

[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/rust.yml)
[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/static.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/static.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![liberty-db](https://shields.io/crates/v/liberty-db.svg?style=flat-square&label=liberty-db)](https://crates.io/crates/liberty-db)
[![liberty-macros](https://shields.io/crates/v/liberty-macros.svg?style=flat-square&label=liberty-macros)](https://crates.io/crates/liberty-macros)
[![Docs](https://docs.rs/liberty-db/badge.svg)](https://docs.rs/liberty-db)


# liberty-db
**Work in progress, unstable**


## Test
```shell
cargo test --package liberty-tests --test tests
```
## Usage
```toml
[dependencies]
liberty_db = "0.4"
```

See more in [doc](https://docs.rs/liberty-db)

## ToDo List

+ stream input
+ fix missing newline at endding
+ recursive parse boolean expression
+ ~~macros~~
+ ~~format to `liberty`~~
+ ~~support multi-line `\`~~
+ ~~support comment~~
+ ~~Use `MutSet` to store GroupMap~~
