# libsugar
![Rust](https://github.com/libsugar/sugar.rs/workflows/Rust/badge.svg)
[![version](https://img.shields.io/crates/v/libsugar)](https://crates.io/crates/libsugar)
[![documentation](https://docs.rs/libsugar/badge.svg)](https://docs.rs/libsugar)
![LICENSE](https://img.shields.io/crates/l/libsugar)

Like syntactic sugar, but is library  

## Features

default = `["std", "combin", "named-into", "macro-lit", "side-effect", "re-exports", "chain_panic", "chain_todo", "tuples", "once_get", "chain_drop"]`  

- `"std"` Enable std  
- `"side-effect"` Enable mod [side_effect](https://docs.rs/libsugar/2.4.0/libsugar/side_effect/index.html)  
- `"named-into"` Enable mod [named_into](https://docs.rs/libsugar/2.4.0/libsugar/named_into/index.html)  
- `"combin"` Enable mod [combin](https://docs.rs/libsugar/2.4.0/libsugar/combin/index.html)  
- `"macro-lit"` Enable macro like [new](https://docs.rs/libsugar/2.4.0/libsugar/macro.new.html), [list](https://docs.rs/libsugar/2.4.0/libsugar/macro.list.html)  
- `"chain_panic"` Enable mod [chain_panic](https://docs.rs/libsugar/2.4.0/libsugar/chain_panic/index.html)
- `"chain_todo"` Enable mod [chain_todo](https://docs.rs/libsugar/2.4.0/libsugar/chain_todo/index.html)
- `"chain_drop"` Enable mod [chain_drop](https://docs.rs/libsugar/2.4.0/libsugar/chain_drop/index.html)
- `"tuples"` Enable mod [tuples](https://docs.rs/libsugar/2.4.0/libsugar/tuples/index.html)  
- `"once_get"` Enable mod [once_get](https://docs.rs/libsugar/2.4.0/libsugar/once_get/index.html)  
- `"re-exports"` Enable re-export of all mods
