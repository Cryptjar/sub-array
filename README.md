# sub-array

[![Crates.io](https://img.shields.io/crates/v/sub-array.svg)](https://crates.io/crates/sub-array)
[![API](https://docs.rs/sub-array/badge.svg)](https://docs.rs/sub-array)

<!-- cargo-sync-readme start -->

Allows to extract a sub-array out of an array

# Example

Getting a sub array:

```rust
use sub_array::SubArray;

let arr: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];

// Get a sub-array starting at offset 1
let sub: &[u8; 3] = arr.sub_array_ref(1);
assert_eq!(sub, &[2, 3, 4]);
```

Initializing an `[u8;10]` array with `(u16, u32, u32)`:

```rust
use sub_array::SubArray;

let foo: u16 = 42;
let bar: u32 = 0x1234;
let baz: u32 = 0x5678;

let mut arr = [0_u8; 10];
*arr.sub_array_mut::<2>(0) = foo.to_be_bytes();
*arr.sub_array_mut::<4>(2) = bar.to_be_bytes();
*arr.sub_array_mut::<4>(6) = baz.to_be_bytes();

assert_eq!(
    arr,
    [
        0, 42, // foo
        0x0, 0x0, 0x12, 0x34, // bar
        0x0, 0x0, 0x56, 0x78, // baz
    ]
);
```

<!-- cargo-sync-readme end -->

# License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or https://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
