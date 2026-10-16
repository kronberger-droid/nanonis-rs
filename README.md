# nanonis-rs

[![Crates.io](https://img.shields.io/crates/v/nanonis-rs.svg)](https://crates.io/crates/nanonis-rs)
[![Documentation](https://docs.rs/nanonis-rs/badge.svg)](https://docs.rs/nanonis-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A type-safe Rust client for the [Nanonis](https://www.specs-group.com/nanonis/)
TCP protocol, for scripting STM and AFM experiments on a Nanonis SPM
controller.

```rust
use nanonis_rs::NanonisClient;

fn main() -> Result<(), nanonis_rs::NanonisError> {
    let mut client = NanonisClient::new("127.0.0.1", 6501)?;

    println!("bias: {} V", client.bias_get()?);
    for (i, name) in client.signal_names_get()?.iter().enumerate() {
        println!("{i}: {name}");
    }
    Ok(())
}
```

- **Broad coverage.** Around 580 of the protocol's commands, from bias, Z
  controller and motors to scans, spectroscopy, oscilloscopes and the PLL,
  each as a method with typed arguments and results.
- **Streaming.** `TCPLoggerStream` reads the TCP Logger data port frame by
  frame, optionally on a background thread.
- **Typed errors.** `NanonisError` separates timeouts, I/O and server-side
  errors with their code and message.
- **Opt-in tip safety.** `NanonisClient::builder().safe_tip_on_drop(true)`
  withdraws the tip and moves the motors clear when the client drops.
- **No panics.** Panic paths are denied crate-wide, so a failed command
  returns an error instead of taking the caller down.

## Install

```sh
cargo add nanonis-rs
```

Needs a Nanonis controller with the TCP interface enabled, reachable on its
control ports (typically 6501 to 6504) and, for streaming, the TCP Logger port
(typically 6590).

## Docs

- [API reference](https://docs.rs/nanonis-rs) on docs.rs
- [`examples/`](examples) for runnable programs
- [`docs/tcp-protocol.md`](docs/tcp-protocol.md), the SPECS "Nanonis TCP
  Protocol" manual (R14718, April 2025) as searchable markdown
- [CHANGELOG](CHANGELOG.md)

[rusty-tip](https://github.com/kronberger-droid/rusty-tip) builds
controller-agnostic SPM automation on top of this crate.

## License

[MIT](LICENSE). Nanonis is a registered trademark of SPECS Zurich GmbH.
