# libinjection-rs

[![crates.io](https://img.shields.io/crates/v/libinjection.svg)](https://crates.io/crates/libinjection) [![Documentation](https://img.shields.io/badge/Docs-libinjection-blue.svg)](https://docs.rs/libinjection) ![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg) [![Test](https://github.com/yaa110/libinjection-rs/actions/workflows/test.yml/badge.svg)](https://github.com/yaa110/libinjection-rs/actions/workflows/test.yml)

Rust bindings for [libinjection](https://github.com/libinjection/libinjection).

## How to use

- Add `libinjection` to `dependencies` of `Cargo.toml`:

```toml
libinjection = "*"
```

- Import crate:

```rust
extern crate libinjection;

use libinjection::{sqli, xss};
```

## Examples

- SQLi Detection:

```rust
let (is_sqli, fingerprint) = sqli("' OR '1'='1' --").unwrap();
assert!(is_sqli);
assert_eq!("s&sos", fingerprint);
```

**Fingerprints:** Please refer to [fingerprints.txt](https://github.com/libinjection/libinjection/blob/master/src/fingerprints.txt).

- XSS Detection:

```rust
let is_xss = xss("<script type='text/javascript'>alert('xss');</script>").unwrap();
assert!(is_xss);
```
