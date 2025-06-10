# Demo Rust scrypt

Demonstration of Rust scrypt password hashing function.

* [Rust programming language](https://www.rust-lang.org/)

* [scrypt crate](https://crates.io/crates/scrypt)

## Usage

The demo password is "toomanysecrets".

Run:

```sh
cargo run
```

Output:

```stdout
Create: $scrypt$ln=17,r=8,p=1$3Wfw13ohcPYvPKv+Py9lDQ$QTviw+3HEv1L2SqCI8ifmzxcyc3c0RpNtIQ+eUaS08Q
Verify: true
```

## PHC String Format

<https://github.com/simonepri/phc-format>

The PHC String Format is an attempt to specify a common hash string format that's a restricted & well defined subset of the Modular Crypt Format.

Example:

```txt
$scrypt$ln=17,r=8,p=1$3Wfw13ohcPYvPKv+Py9lDQ$QTviw+3HEv1L2SqCI8ifmzxcyc3c0RpNtIQ+eUaS08Q
```
