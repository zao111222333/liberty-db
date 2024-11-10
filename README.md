# liberty-db

[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/build.yml)
[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/bench_deploy.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/bench_deploy.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![liberty-db](https://shields.io/crates/v/liberty-db.svg?style=flat-square&label=liberty-db)](https://crates.io/crates/liberty-db)
[![Docs](https://docs.rs/liberty-db/badge.svg)](https://docs.rs/liberty-db)
[![Benchmark](https://img.shields.io/badge/Benchmark-8A2BE2)](https://zao111222333.github.io/liberty-db/bench)
[![codecov](https://codecov.io/github/zao111222333/liberty-db/graph/badge.svg?token=AI2BVDIFOI)](https://codecov.io/github/zao111222333/liberty-db)

## Highlight Features

+ Support all `liberty` attributes with typed datastructure, rather than syntax tree (AST) only, in the definition of [Liberty Version R-2020.09, September 2020](https://zao111222333.github.io/liberty-db/2020.09/reference_manual.pdf)
+ Support [`liberty` boolean expression syntax](https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=132.36+132.41&end=132.38+133.13), employ [binary decesion diagram (BDD)](https://github.com/sybila/biodivine-lib-bdd) to [identify (hashing)](https://docs.rs/liberty-db/latest/liberty_db/expression/struct.IdBooleanExpression.html)
+ Parser & formatter are implemented by static macros, providing a decent preformance and tiny ~2MB `parser & formatter` binary. See [benchmark summary](https://zao111222333.github.io/liberty-db/bench)
+ Support `define define_group` user-defined attributes
+ Verified with library-complier, well defined [document](https://docs.rs/liberty-db/latest/liberty_db/library/struct.Library.html), and [examples](examples) provided

## Usage

This library is implemented in [Rust](https://doc.rust-lang.org/book/ch01-00-getting-started.html), with [document](https://docs.rs/liberty-db).

```toml
[dependencies]
liberty_db = "0.6"
```

You can run [examples](examples) with commands:

``` shell
# example0
cargo run --package example0_parse_fmt
# example1
cargo run --package example1_parse_fmt_file -- dev/tech/cases/ocv.lib
# example2
cargo run --package example2_prune_lib -- dev/tech/cases/ocv.lib
```

## Benchmark
Basic information as follow, see latest [benchmark summary](https://zao111222333.github.io/liberty-db/bench).

### Project Comparison
<div class="info-table"><table><thead><tr><th rowspan="2" style="font-weight:bold;">Project</th><th rowspan="2" style="font-weight:bold;">Lang</th><th rowspan="2" style="font-weight:bold;">Version</th><th colspan="3" style="text-align:center;font-weight:bold;">Type Support</th><th rowspan="2" style="font-weight:bold;">Boolean<br>Expression</th><th rowspan="2" style="font-weight:bold;">Comment</th></tr><tr><th>All</th><th>Partly</th><th>AST only</th></tr></thead><tbody><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db">liberty-db</a></th><th>rust</th><th>latest</th><th>✓</th><th></th><th></th><th>✓</th><th>current version</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/csguth/LibertyParser">si2dr_liberty</a></th><th>C</th><th>1.0</th><th>✓</th><th></th><th></th><th>✓</th><th>Synopsys's version at 2005, many attributes are not supported</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/OpenTimer/OpenTimer/tree/a57d03b39886c1e2f113c1a893f5b3fad9199a52">OpenTimer</a></th><th>C++17</th><th>2</th><th></th><th>✓</th><th></th><th>✓</th><th>STA tool's liberty component</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-io">liberty-io</a></th><th>rust</th><th>0.0.4</th><th></th><th></th><th>✓</th><th></th><th></th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/libertyparse">libertyparse</a></th><th>rust</th><th>0.3.0</th><th></th><th>✓</th><th></th><th>✓</th><th></th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/erihsu/liberty2json/tree/7d0a4f233f143fce9c2844208f4d48033622d93f">liberty2json</a></th><th>rust</th><th>0.1.0</th><th></th><th></th><th>✓</th><th></th><th></th></tr></tbody></table></div>

### Self Regression
<div class="info-table"><table><thead><tr><th rowspan="2" style="font-weight:bold;">Project</th><th rowspan="2" style="font-weight:bold;">Lang</th><th rowspan="2" style="font-weight:bold;">Version</th><th colspan="3" style="text-align:center;font-weight:bold;">Type Support</th><th rowspan="2" style="font-weight:bold;">Boolean<br>Expression</th><th rowspan="2" style="font-weight:bold;">Comment</th></tr><tr><th>All</th><th>Partly</th><th>AST only</th></tr></thead><tbody><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db">liberty-db</a></th><th>rust</th><th>latest</th><th>✓</th><th></th><th></th><th>✓</th><th>current version</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db/0.6.3">liberty-db</a></th><th>rust</th><th>0.6.3</th><th>✓</th><th></th><th></th><th>✓</th><th>published at 2024-09-07</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db/0.5.9">liberty-db</a></th><th>rust</th><th>0.5.9</th><th>✓</th><th></th><th></th><th>✓</th><th>published at 2024-08-27</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db/0.4.13">liberty-db</a></th><th>rust</th><th>0.4.13</th><th>✓</th><th></th><th></th><th>✓</th><th>published at 2024-08-13</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db/0.3.1">liberty-db</a></th><th>rust</th><th>0.3.1</th><th></th><th>✓</th><th></th><th></th><th>published at 2023-08-03</th></tr></tbody></table></div>

## Dev

Run unit-test and regression.

```shell
cargo test
cargo test --package dev
```

Run benchmark, it will takes 5h.

```shell
cargo bench --bench dev
```

## TODO

+ `intrinsic_parasitic` group
+ `leakage_current` group
+ `dynamic_current` group
+ stream input
+ Linked Group
+ ~~user `define`~~
+ ~~remove `GroupWapper`, `ComplexWapper`, `SimpleWapper`. At leaset remove it in parser and formatter~~
+ ~~fix missing newline at endding~~
+ ~~recursive parse boolean expression~~
+ ~~macros~~
+ ~~format to `liberty`~~
+ ~~support multi-line `\`~~
+ ~~support comment~~
+ ~~Use `MutSet` to store GroupMap~~
