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
liberty_db = "0.2"
```

See more in [doc](https://docs.rs/liberty-db)

## ToDo List
+ ![](https://progress-bar.dev/100?title=done) support comment
+ ![](https://progress-bar.dev/100?title=done) support multi-line `\`
+ ![](https://progress-bar.dev/90?title=doing) format to `liberty`
+ ![](https://progress-bar.dev/60?title=doing) macros
+ ![](https://progress-bar.dev/0?title=todo) stream input
+ `#[liberty(id)]` for self impl `HashedGroup`
+ fix missing newline at endding
+ recursive parse boolean expression
+ merge `readonly` into this macros
+ HashedGroup Bulder;
+ replace `#[liberty(group(type=Option))]`,`#[liberty(group(type=Vec))]` to `#[liberty(group)]`, since `Option<>` and `Vec<>` should have `Default::default(self)` function

``` rust
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[readonly::make]
pub struct TableLookUpMultiSegment {
  #[readonly]
  #[liberty(id)]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  #[readonly]
  #[liberty(id)]
  #[liberty(simple)]
  segment: usize,
  #[liberty(complex(type=Default))]
  pub index_1: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_2: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_3: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_4: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub values: Vec<f64>,
}
```
