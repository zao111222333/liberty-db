# liberty-db

[![DeepWiki](https://img.shields.io/badge/DeepWiki-zao111222333%2Fliberty--db-blue.svg?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACwAAAAyCAYAAAAnWDnqAAAAAXNSR0IArs4c6QAAA05JREFUaEPtmUtyEzEQhtWTQyQLHNak2AB7ZnyXZMEjXMGeK/AIi+QuHrMnbChYY7MIh8g01fJoopFb0uhhEqqcbWTp06/uv1saEDv4O3n3dV60RfP947Mm9/SQc0ICFQgzfc4CYZoTPAswgSJCCUJUnAAoRHOAUOcATwbmVLWdGoH//PB8mnKqScAhsD0kYP3j/Yt5LPQe2KvcXmGvRHcDnpxfL2zOYJ1mFwrryWTz0advv1Ut4CJgf5uhDuDj5eUcAUoahrdY/56ebRWeraTjMt/00Sh3UDtjgHtQNHwcRGOC98BJEAEymycmYcWwOprTgcB6VZ5JK5TAJ+fXGLBm3FDAmn6oPPjR4rKCAoJCal2eAiQp2x0vxTPB3ALO2CRkwmDy5WohzBDwSEFKRwPbknEggCPB/imwrycgxX2NzoMCHhPkDwqYMr9tRcP5qNrMZHkVnOjRMWwLCcr8ohBVb1OMjxLwGCvjTikrsBOiA6fNyCrm8V1rP93iVPpwaE+gO0SsWmPiXB+jikdf6SizrT5qKasx5j8ABbHpFTx+vFXp9EnYQmLx02h1QTTrl6eDqxLnGjporxl3NL3agEvXdT0WmEost648sQOYAeJS9Q7bfUVoMGnjo4AZdUMQku50McDcMWcBPvr0SzbTAFDfvJqwLzgxwATnCgnp4wDl6Aa+Ax283gghmj+vj7feE2KBBRMW3FzOpLOADl0Isb5587h/U4gGvkt5v60Z1VLG8BhYjbzRwyQZemwAd6cCR5/XFWLYZRIMpX39AR0tjaGGiGzLVyhse5C9RKC6ai42ppWPKiBagOvaYk8lO7DajerabOZP46Lby5wKjw1HCRx7p9sVMOWGzb/vA1hwiWc6jm3MvQDTogQkiqIhJV0nBQBTU+3okKCFDy9WwferkHjtxib7t3xIUQtHxnIwtx4mpg26/HfwVNVDb4oI9RHmx5WGelRVlrtiw43zboCLaxv46AZeB3IlTkwouebTr1y2NjSpHz68WNFjHvupy3q8TFn3Hos2IAk4Ju5dCo8B3wP7VPr/FGaKiG+T+v+TQqIrOqMTL1VdWV1DdmcbO8KXBz6esmYWYKPwDL5b5FA1a0hwapHiom0r/cKaoqr+27/XcrS5UwSMbQAAAABJRU5ErkJggg==)](https://deepwiki.com/zao111222333/liberty-db)

Fully defined liberty (std. cells in VLSI) data structure, efficient parser & formatter.

[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/build.yml)
[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/bench_deploy.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/bench_deploy.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![liberty-db](https://shields.io/crates/v/liberty-db.svg?style=flat-square&label=crates.io)](https://crates.io/crates/liberty-db)
[![Docs](https://docs.rs/liberty-db/badge.svg)](https://docs.rs/liberty-db)
[![Benchmark](https://img.shields.io/badge/Benchmark-8A2BE2)](https://zao111222333.github.io/liberty-db/bench)
[![codecov](https://codecov.io/github/zao111222333/liberty-db/graph/badge.svg?token=AI2BVDIFOI)](https://codecov.io/github/zao111222333/liberty-db)

## Highlight Features

+ Support all `liberty` attributes with typed datastructure, rather than syntax tree (AST) only, in the definition of [Liberty Version R-2020.09, September 2020](https://zao111222333.github.io/liberty-db/2020.09/reference_manual.pdf)
+ Support [`liberty` boolean expression syntax](https://docs.rs/liberty-db/latest/liberty_db/expression/struct.BddBooleanExpression.html), use [binary decesion diagram (BDD)](https://github.com/sybila/biodivine-lib-bdd) to identify (hashing)
+ Parser & formatter are implemented by static macros, providing a good preformance and tiny ~3MB `parser & formatter` binary. See [benchmark summary](https://zao111222333.github.io/liberty-db/bench)
+ Support generics [user-defined context](https://docs.rs/liberty-db/latest/liberty_db/trait.Ctx.html) for library, cell, and more.
+ Support `define define_group` user-defined attributes
+ Verified with library-complier, well defined [document](https://docs.rs/liberty-db/latest/liberty_db/library/struct.Library.html), and [examples](examples) provided

## Usage

This library is implemented in [Rust](https://doc.rust-lang.org/book/ch01-00-getting-started.html), with [document](https://docs.rs/liberty-db).

```toml
[dependencies]
liberty_db = "0.11"
```

One basic demo here:

```rust
use liberty_db::{DefaultCtx, Library, MutSetExt, PinId};
use std::{
  fs::File,
  io::{BufWriter, Write},
};
static TEMPLATE: &str = r#"
library(demo) {
  time_unit : "1ps";
  voltage_unit : "10mV";
  current_unit : "1uA";
  operating_conditions ( typical ) {
      process : 1;
      voltage : 1.1;
  }
  lu_table_template(delay_template_4x5) {
    variable_1 : total_output_net_capacitance;
    variable_2 : input_net_transition;
    index_1 ("1000.0, 1001.0, 1002.0, 1003.0");
    index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0");
  }
  cell (DFF) {
    pin (D) {}
    pin (CK) {}
    pin (Q) {}
  }
}"#;
fn main(){
  let mut library = Library::<DefaultCtx>::parse_lib(TEMPLATE, None).unwrap();
  // modify library
  library.cell.get_mut("DFF").map(|cell_dff| {
    cell_dff
      .pin
      .get_mut(&PinId::from("CK"))
      .map(|pin_ck| pin_ck.clock = Some(true))
  });
  // print library
  println!("{library}");
  // write library
  let out_file = File::create("demo.lib").unwrap();
  let mut writer = BufWriter::new(out_file);
  write!(&mut writer, "{}", library).unwrap();
}
```

See more [examples](examples), and run them if you clone this repo:

``` shell
# example0
cargo run --example 0_parse_fmt
# example1
cargo run --example 1_parse_fmt_file -- dev/tech/cases/ocv.lib
# example2
cargo run --example 2_prune_lib -- dev/tech/cases/ocv.lib
# example3
cargo run --example 3_lookup_timing
```

Or you can download the [releases/latest/examples.zip](https://github.com/zao111222333/liberty-db/releases/latest/download/examples_x86_64-unknown-linux-musl.zip), then

``` shell
cd ./examples_x86_64-unknown-linux-musl
# example0
./0_parse_fmt
# example1
./1_parse_fmt_file dev/tech/cases/ocv.lib
# example2
./2_prune_lib dev/tech/cases/ocv.lib
# example3
./3_lookup_timing
```

## Benchmark
Basic information as follow, see latest [benchmark summary](https://zao111222333.github.io/liberty-db/bench).

### Project Comparison
<div class="info-table"><table><thead><tr><th rowspan="2" style="font-weight:bold;">Project</th><th rowspan="2" style="font-weight:bold;">Lang</th><th rowspan="2" style="font-weight:bold;">Version</th><th colspan="3" style="text-align:center;font-weight:bold;">Type Support</th><th rowspan="2" style="font-weight:bold;">Boolean<br>Expression</th><th rowspan="2" style="font-weight:bold;">Comment</th></tr><tr><th>All</th><th>Partly</th><th>AST only</th></tr></thead><tbody><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-db">liberty-db</a></th><th>rust</th><th>latest</th><th>✓</th><th></th><th></th><th>✓</th><th>current version</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/csguth/LibertyParser">si2dr_liberty</a></th><th>C</th><th>1.0</th><th>✓</th><th></th><th></th><th>✓</th><th>Synopsys's version at 2005, many attributes are not supported</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/OpenTimer/OpenTimer/tree/a57d03b39886c1e2f113c1a893f5b3fad9199a52">OpenTimer</a></th><th>C++17</th><th>2</th><th></th><th>✓</th><th></th><th>✓</th><th>STA tool's liberty component</th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/liberty-io">liberty-io</a></th><th>rust</th><th>0.0.4</th><th></th><th></th><th>✓</th><th></th><th></th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://crates.io/crates/libertyparse">libertyparse</a></th><th>rust</th><th>0.3.0</th><th></th><th>✓</th><th></th><th>✓</th><th></th></tr><tr><th style="text-align:left;padding-left:5px"><a href="https://github.com/erihsu/liberty2json/tree/7d0a4f233f143fce9c2844208f4d48033622d93f">liberty2json</a></th><th>rust</th><th>0.1.0</th><th></th><th></th><th>✓</th><th></th><th></th></tr></tbody></table></div>

## Dev

Run unit-test and regression.

```shell
cargo test --release
```

Run benchmark, it will takes 40mins.

```shell
cargo bench --package dev --bench benchmark --features bench
```

## TODO

+ Parse: use `nom_locate`
+ Parse: check all rules
+ Parse: Linked Group
+ Parse: async
+ MISC: bitcode support