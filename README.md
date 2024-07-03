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

```rust
// bool
let seed = 10;

let is_even = ternary!(seed % 2 == 0, true, false);
assert!(is_even);
```

```rust
// &str
let seed = 10;

let result = ternary!(seed > 5, "greater than 5", "less than or equal to 5");
assert_eq!("greater than 5", result);
```

#### 2.1.2.`ternary_eq`

```rust
let eq = ternary_eq!(2, 2, 1, -1);
let ne = ternary_eq!(1, 2, 1, -1);

assert_eq!(1, eq);
assert_eq!(-1, ne);
```

```rust
// date-time
let now = SystemTime::now();
let now_add = now.add(Duration::from_millis(1_000));
let now_sub = now.sub(Duration::from_millis(1_000));

let eq = ternary_eq!(now, now, 1, -1);
let ne = ternary_eq!(now, now.add(Duration::from_millis(1_000)), now_add, now_sub);

assert_eq!(1, eq);
assert_eq!(now_sub, ne);
```

#### 2.1.3.`ternary_ne`

```rust
let ne = ternary_ne!(3, 2, 1, -1);
let eq = ternary_ne!(2, 2, 1, -1);

assert_eq!(1, ne);
assert_eq!(-1, eq);
```

```rust
// date-time
let now = SystemTime::now();
let now_add = now.add(Duration::from_millis(1_000));
let now_sub = now.sub(Duration::from_millis(1_000));

let ne = ternary_ne!(now, now.add(Duration::from_millis(1_000)), 1, -1);
let eq = ternary_ne!(now, now, now_add,now_sub);

assert_eq!(1, ne);
assert_eq!(now_sub, eq);
```



## 2.2.`Repeat`

- `@since 0.2.0`

### 2.2.1.`repeat!`

```rust
let repeat_1 = repeat!("A", 5);
assert_eq!(vec!["A", "A", "A", "A", "A"], repeat_1);

let repeat_2 = repeat!(101, 5);
assert_eq!(vec![101, 101, 101, 101, 101], repeat_2);
```



### 2.2.2.`repeat_str!`

```rust
// Item: Alphabet
// Default separator -> `""`
let repeat_str_1 = repeat_str!("A", 5);
assert_eq!("AAAAA", repeat_str_1);

// Item: Alphabet
// Custem separator -> `,`
let repeat_str_2 = repeat_str!("A", 5, ",");
assert_eq!("A,A,A,A,A", repeat_str_2);

// Item: Number 
let repeat_str_3 = repeat_str!(101, 5, ",");
assert_eq!("101,101,101,101,101", repeat_str_3);
```

