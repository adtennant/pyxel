# Pyxel

[![Build Status](https://api.travis-ci.org/adtennant/pyxel.svg?branch=master)](https://travis-ci.org/adtennant/pyxel)
[![Crates.io](https://img.shields.io/crates/v/pyxel.svg)](https://crates.io/crates/pyxel)
[![Docs](https://docs.rs/pyxel/badge.svg)](https://docs.rs/pyxel)
[![License](https://img.shields.io/crates/l/pyxel.svg)](https://github.com/adtennant/pyxel/blob/master/LICENSE)

Pyxel is a library for loading [PyxelEdit](https://pyxeledit.com) documents in Rust. Current only the latest (v0.4.8) version of PyxelEdit is officially supported.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pyxel = "0.2.0"
```

Then open a `.pyxel` file like this:

```rust
let doc = pyxel::open("resources/doc.pyxel")?;
```

See the [docs](https://docs.rs/pyxel) for more information.

## Optional Features

The following features are available:

- **`images`** — Can be used to automatically load layer and tileset images within the Pyxel document using [`image`](https://crates.io/crates/image).

## License

[MIT](https://github.com/adtennant/morgan-console/blob/master/LICENSE)
