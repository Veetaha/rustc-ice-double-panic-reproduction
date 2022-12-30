Reproduction of the ICE in rustc that produces a double panic and abort consequently.

[Issue reported in rust-lang/rust repository](https://github.com/rust-lang/rust/issues/106298).

Use the following command to reproduce:

```
cargo check
   Compiling proc-macro2 v1.0.49
   Compiling quote v1.0.23
   Compiling unicode-ident v1.0.6
   Compiling syn v1.0.107
   Compiling ident_case v1.0.1
   Compiling fnv v1.0.7
   Compiling strsim v0.10.0
   Compiling darling_core v0.13.4
   Compiling darling_macro v0.13.4
   Compiling darling v0.13.4
   Compiling from_variants_impl v1.0.0
    Checking from_variants v1.0.0
    Checking rustc-ice-double-panic-reproduction v0.1.0 (/home/veetaha/dev/rustc-ice-double-panic-reproduction)
thread panicked while panicking. aborting.
error: could not compile `rustc-ice-double-panic-reproduction`

Caused by:
  process didn't exit successfully: `rustc --crate-name rustc_ice_double_panic_reproduction --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=59bd2ae5acf02373 -C extra-filename=-59bd2ae5acf02373 --out-dir /home/veetaha/dev/rustc-ice-double-panic-reproduction/target/debug/deps -C incremental=/home/veetaha/dev/rustc-ice-double-panic-reproduction/target/debug/incremental -L dependency=/home/veetaha/dev/rustc-ice-double-panic-reproduction/target/debug/deps --extern from_variants=/home/veetaha/dev/rustc-ice-double-panic-reproduction/target/debug/deps/libfrom_variants-efa1e139161f23f5.rmeta` (signal: 6, SIGABRT: process abort signal)
```
