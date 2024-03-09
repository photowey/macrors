# `macrors`

A simple custom `macro` library in `Rust`.

[APIs Documents](https://docs.rs/macrors)

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
macrors = "0.1"
```

## 2.`APIs`

### 2.1.`Operator`

#### 2.1.1.`ternary`

```rust
let seed = 10;

let pos = ternary!(seed % 2 == 0, 1, -1);
let neg = ternary!((seed - 1) % 2 == 0, 1, -1);
assert_eq!(1, pos);
assert_eq!(-1, neg);
```

#### 2.1.2.`ternary_eq`

```rust
let eq = ternary_eq!(2, 2, 1, -1);
let ne = ternary_eq!(1, 2, 1, -1);

assert_eq!(1, eq);
assert_eq!(-1, ne);
```

#### 2.1.3.`ternary_ne`

```rust
let ne = ternary_ne!(3, 2, 1, -1);
let eq = ternary_ne!(2, 2, 1, -1);

assert_eq!(1, ne);
assert_eq!(-1, eq);
```

