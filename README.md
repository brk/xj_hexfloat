# xj_hexfloat

A thin wrapper around [`hexfloat2`](https://crates.io/crates/hexfloat2) whose `Display` always emits an explicit `+` sign in the binary exponent.

```
hexfloat2:   0x1.0p0   0x1.8p6   0x1.0p-1
xj_hexfloat: 0x1.0p+0  0x1.8p+6  0x1.0p-1
```

Negative exponents and special values (`inf`, `NaN`) are passed through unchanged.

## Usage

```rust
use xj_hexfloat::HexFloat;

let s = format!("{}", HexFloat::from(1.5f32));
assert_eq!(s, "0x1.8p+0");
```
