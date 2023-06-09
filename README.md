# Intervasp Messaging Standard 101 Rust library

[![Crates.io](https://img.shields.io/crates/v/ivms101.svg)](https://crates.io/crates/ivms101)
[![Documentation](https://docs.rs/ivms101/badge.svg)](https://docs.rs/ivms101/)

`ivms101` is a Rust library for working with data payloads defined in the [Intervasp Messaging Standard 101](https://intervasp.org/).

## Example

```rust
use ivms101::messages::Validatable;

fn main() {
    let person =
        ivms101::messages::NaturalPerson::new("John", "Doe", Some("id-273934"), None).unwrap();
    assert!(person.validate().is_ok());
}
```

## Usage

Add `ivms101` to your `Cargo.toml`:

```sh
cargo add ivms101
```

## Authors

This crate is developed and maintained by [21 Analytics](https://21analytics.ch).

## License

This project is licensed under the GNU Affero General Public license.
