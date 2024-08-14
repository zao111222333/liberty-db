# liberty-db

[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/rust.yml)
[![pipeline](https://github.com/zao111222333/liberty-db/actions/workflows/static.yml/badge.svg?branch=master)](https://github.com/zao111222333/liberty-db/actions/workflows/static.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![liberty-db](https://shields.io/crates/v/liberty-db.svg?style=flat-square&label=liberty-db)](https://crates.io/crates/liberty-db)
[![liberty-macros](https://shields.io/crates/v/liberty-macros.svg?style=flat-square&label=liberty-macros)](https://crates.io/crates/liberty-macros)
[![Docs](https://docs.rs/liberty-db/badge.svg)](https://docs.rs/liberty-db)

**Work in progress, unstable**

## Usage

```toml
[dependencies]
liberty_db = "0.5"
```

See more in [doc](https://docs.rs/liberty-db)


## Test

```shell
cargo test --package liberty-tests --test tests
```

## Testbench

```shell
cargo test --package liberty-tests --lib -- parser_bench::test_all_lib_files --exact --nocapture 
```

| [liberty-db](https://crates.io/crates/liberty-db) | [liberty-io](https://crates.io/crates/liberty-io) | [libertyparse](https://crates.io/crates/libertyparse) | Test Liberty File |
| ---------- | ---------- | ------------ | ----------------- |
| <span style="color:#4CBB17">**402.79ms**</span>   | <span style="color:#4CBB17">**115.81ms**</span>   | <span style="color:#4CBB17">**342.70ms**</span>     | [`tests/tech/SAED32_EDK/ccs/saed32hvt_pg_ff0p95v125c.lib`](tests/tech/SAED32_EDK/ccs/saed32hvt_pg_ff0p95v125c.lib)                      |
| <span style="color:#4CBB17">**599.51ms**</span>   | <span style="color:#4CBB17">**284.08ms**</span>   | <span style="color:#4CBB17">**839.44ms**</span>     | [`tests/tech/SAED32_EDK/ccs/saed32hvt_dlvl_ff0p85v25c_i0p85v.lib`](tests/tech/SAED32_EDK/ccs/saed32hvt_dlvl_ff0p85v25c_i0p85v.lib)              |
| <span style="color:#4CBB17">**83.12ms**</span>    | <span style="color:#4CBB17">**40.12ms**</span>    | <span style="color:#4CBB17">**122.26ms**</span>     | [`tests/tech/SAED32_EDK/nldm/saed32hvt_dlvl_ff0p85v25c_i0p85v.lib`](tests/tech/SAED32_EDK/nldm/saed32hvt_dlvl_ff0p85v25c_i0p85v.lib)             |
| <span style="color:#4CBB17">**4.50s**</span>      | <span style="color:#4CBB17">**1.48s**</span>      | <span style="color:red">**PANIC**</span>        | [`tests/tech/SAED32_EDK/nldm/saed32hvt_ff0p85v25c.lib`](tests/tech/SAED32_EDK/nldm/saed32hvt_ff0p85v25c.lib)                         |
| <span style="color:#4CBB17">**3.00ms**</span>     | <span style="color:#4CBB17">**20.58ms**</span>    | <span style="color:#4CBB17">**5.08ms**</span>       | [`tests/tech/freepdk45/gscl45nm.lib`](tests/tech/freepdk45/gscl45nm.lib)                                           |
| <span style="color:#4CBB17">**125.41µs**</span>   | <span style="color:#FF3131">**FAIL**</span>       | <span style="color:#FF3131">**FAIL**</span>         | [`tests/tech/cases/no_semicolon.lib`](tests/tech/cases/no_semicolon.lib)                                           |
| <span style="color:#4CBB17">**66.77µs**</span>    | <span style="color:#FF3131">**FAIL**</span>       | <span style="color:#4CBB17">**131.15µs**</span>     | [`tests/tech/cases/formula.lib`](tests/tech/cases/formula.lib)                                                |
| <span style="color:#4CBB17">**16.04ms**</span>    | <span style="color:#4CBB17">**8.67ms**</span>     | <span style="color:#4CBB17">**25.57ms**</span>      | [`tests/tech/cases/ocv.lib`](tests/tech/cases/ocv.lib)                                                    |
| <span style="color:#4CBB17">**13.69ms**</span>    | <span style="color:#4CBB17">**6.76ms**</span>     | <span style="color:#FF3131">**FAIL**</span>         | [`tests/tech/sky130/sky130_fd_sc_hs__bufinv_8__tt_1p80V_25C_ccsnoise.cell.lib`](tests/tech/sky130/sky130_fd_sc_hs__bufinv_8__tt_1p80V_25C_ccsnoise.cell.lib) |
| <span style="color:#4CBB17">**2.13s**</span>      | <span style="color:#4CBB17">**870.17ms**</span>   | <span style="color:#4CBB17">**2.85s**</span>        | [`tests/tech/nangate/NangateOpenCellLibrary_typical.lib`](tests/tech/nangate/NangateOpenCellLibrary_typical.lib)                       |

## TODO List

+ remove `GroupWapper`, `ComplexWapper`, `SimpleWapper`. At leaset remove it in parser and formatter
+ merge fields of `timing` at parse phase
```
impl __timing::ImmutIdTiming { xxx }
```
+ use PDFJS
+ stream input
+ Linked Group
+ Timing group
+ ~~fix missing newline at endding~~
+ ~~recursive parse boolean expression~~
+ ~~macros~~
+ ~~format to `liberty`~~
+ ~~support multi-line `\`~~
+ ~~support comment~~
+ ~~Use `MutSet` to store GroupMap~~
