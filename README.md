# `asm-delay`

> no_std implementation of embedded-hal's DelayMs & DelayUs for cortex-m.

[![Build Status](https://travis-ci.org/copterust/asm-delay.svg?branch=master)](https://travis-ci.org/copterust/asm-delay)


## Basic usage

Include [library](https://crates.io/crates/asm-delay) as a dependency in your Cargo.toml
[![crates.io](http://meritbadge.herokuapp.com/asm-delay?style=flat-square)](https://crates.io/crates/asm-delay):

```
[dependencies.asm_delay]
version = "<version>"
```

```rust
use embedded_hal::prelude::*;
use asm_delay::AsmDelay;
use asm_delay::bitrate::*;

let d = AsmDelay::new(64.mhz());
d.delay_ms(5);
```

## Documentation

API Docs available on [docs.rs](https://docs.rs/asm-delay).

## License

Licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
