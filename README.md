# eprmutate
Permutation library.

Calculation of a permutation from a number and back:
```rust
use eprmutate::permutation;
use eprmutate::permutation_number;

let p = permutation(u128::MAX);

assert_eq!(permutation_number(&p), Some(u128::MAX)); // p doesn't know the number

assert_eq!( p.v, [
    18, 13, 11, 8, 27, 16, 20, 22, 12, 24, 9, 1, 30, 3, 15, 23, 25, 2, 28, 19, 14, 29, 5, 10, 17,
    31, 32, 26, 33, 0, 4, 6, 21, 34, 7
]);
```

Permutating by number:
```rust
use eprmutate::permutation;
assert_eq!(permutation(0).v.as_slice(), [0]);
assert_eq!(permutation(1).v.as_slice(), [1, 0]);
assert_eq!(permutation(2).v.as_slice(), [0, 2, 1]);
assert_eq!(permutation(3).v.as_slice(), [1, 2, 0]);
assert_eq!(permutation(4).v.as_slice(), [2, 0, 1]);
assert_eq!(permutation(5).v.as_slice(), [2, 1, 0]);
assert_eq!(permutation(6).v.as_slice(), [0, 1, 3, 2]);
```

Initializing by a start number and continuing:

```rust
use eprmutate::PermutationRemainder;
let pr = PermutationRemainder::new( 7);
assert_eq!( pr.permutation().v.as_slice(), [1, 0, 3, 2]);
let pr = pr.next().unwrap();
assert_eq!( pr.permutation().v.as_slice(), [0, 2, 3, 1]);
let pr = pr.next().unwrap();
assert_eq!( pr.permutation().v.as_slice(), [1, 2, 3, 0]);
let pr = pr.next().unwrap();
assert_eq!( pr.permutation().v.as_slice(), [2, 0, 3, 1]);
assert_eq!( 10, pr.permutation_number().unwrap())
```

Managing a fixed length permutation
```rust
use eprmutate::permutation;
assert_eq!(permutation(0).fix_length(3).unwrap().as_slice(), [0, 1, 2]);
assert_eq!(permutation(1).fix_length(3).unwrap().as_slice(), [1, 0, 2]);
assert_eq!(permutation(2).fix_length(3).unwrap().as_slice(), [0, 2, 1]);
assert_eq!(permutation(3).fix_length(3).unwrap().as_slice(), [1, 2, 0]);
assert_eq!(permutation(4).fix_length(3).unwrap().as_slice(), [2, 0, 1]);
assert_eq!(permutation(5).fix_length(3).unwrap().as_slice(), [2, 1, 0]);
assert_eq!(permutation(6).fix_length(3), None);
```
