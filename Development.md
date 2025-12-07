# Requirements

- Rust
- cargo
- Espressif Xtensa Rust toolkit (install via `espup`)

I have compiled this repo with Rust 1.91.1 on Manjaro Linux.

Make sure to source the esp script:

```bash
source ~/export-esp.sh
```

# Build

```bash
cargo build
```

# Test

```bash
cargo test --all
```

# Flash

To flash and run the firmware on a ESP32-PhotoPainter, run this:

```bash
cargo espflash flash --monitor
```

# Maintenance

## Update dependencies

`cargo update` only updates dependencies inside `Cargo.lock`.
To update your dependencies in the `Cargo.toml`, use
[cargo-edit](https://archlinux.org/packages/extra/x86_64/cargo-edit/) and
[cargo-outdated](https://archlinux.org/packages/extra/x86_64/cargo-outdated/).

List outdated dependencies:

```bash
cargo outdated
```

To update/upgrade dependencies, use this:

```bash
cargo upgrade --incompatible allow
cargo update
```

## Format code, fix warnings

```bash
cargo fmt
cargo check
cargo fix
cargo clippy --all-targets --all-features -- --deny warnings
```
