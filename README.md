# sub-array

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

Modifying through a mutable sub array:

```rust
use sub_array::SubArray;

let mut arr = ["baz".to_string(), "qux".to_string(), "foo".to_string()];

// Get mutable sub-array starting at offset 2 (last element)
let sub: &mut [String; 1] = arr.sub_array_mut(2);
sub[0].push_str("bar");

// The original array has been modified
assert_eq!(
    arr,
    ["baz".to_string(), "qux".to_string(), "foobar".to_string()]
);
```

<!-- cargo-sync-readme end -->

# License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or https://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

