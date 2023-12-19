/*!
Permutation Library. 

Calculation of a permutation from a number and back:
```rust
use eprmutate::permutation;
use eprmutate::permutation_number;

let p = permutation(u128::MAX);
assert_eq!(permutation_number(&p), Some(u128::MAX));

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

*/

#[derive(Debug, PartialEq)]
pub struct Permutation {
 pub v : Vec<u8>,
}

impl Permutation {
 pub fn fix_length( &self, len: u8) -> Option<Vec<u8>> {
  let mut v = self.v.clone();
  if ! fix_length(len, &mut v) { return None;}
  Some( v)
 }
}

#[derive(Debug, PartialEq)]
pub struct DivRem {
 div: Div,
 rem: Rem,
}

#[derive(Debug, PartialEq)]
struct Div {
 div: u128,
}

#[derive(Debug, PartialEq)]
struct Rem {
 rem: u8,
}

impl DivRem {
 fn divrem(nominator: u128, denominator: u8) -> Self {
  let denominator = denominator as u128;
  let div = Div {
   div: nominator / denominator,
  };
  let rem = nominator % denominator;
  let rem = Rem { rem: rem as u8 };
  Self { div, rem }
 }
}

#[derive(Debug, PartialEq)]
pub struct PermutationRemainder {
 vr: Vec<Rem>,
}

/// holds the permutation reminders
impl PermutationRemainder {
 pub fn new(mut u: u128) -> PermutationRemainder {
  let mut vr = Vec::<Rem>::new();

  for i in 2.. {
   if u == 0 {
    break;
   }
   let dr = DivRem::divrem(u, i);
   u = dr.div.div;
   vr.push(dr.rem);
  }

  PermutationRemainder { vr }
 }

 pub fn next_unsafe(&self) -> PermutationRemainder {
  let mut vr = Vec::<Rem>::new();

  let mut overflow = 1;

  for idx in 0u8..self.vr.len() as u8 {
   let rem = self.vr[idx as usize].rem;
   let denominator = 2 + idx;
   let sum = rem + overflow;
   overflow = if sum >= denominator { 1 } else { 0 };
   let newrem = sum % denominator;
   vr.push(Rem { rem: newrem })
  }

  if 1 == overflow {
   vr.push(Rem { rem: 1 })
  }

  PermutationRemainder { vr }
 }

 /// as long as the permutation number is always between including 0 and including u128::MAX then the result will always be correct
 pub fn permutation_number_unsafe(&self) -> u128 {
  let mut ret: u128 = 0;
  let mut base: u128 = 1;
  let mut denominator: u128 = 1;
  for idx in 0u8..self.vr.len() as u8 {
   base *= denominator;
   let rem = self.vr[idx as usize].rem;
   denominator = 2 + idx as u128;
   ret += rem as u128 * base;
  }

  ret
 }

 /// if the permutation number exceeds u128::MAX the return value is None
 pub fn permutation_number(&self) -> Option<u128> {
  let mut ret: u128 = 0;
  let mut base: u128 = 1;
  let mut denominator: u128 = 1;
  for idx in 0u8..self.vr.len() as u8 {
   base *= denominator;
   let rem = self.vr[idx as usize].rem;
   denominator = 2 + idx as u128;
   match base.checked_mul(rem as u128) {
    None => return None,
    Some(prod) => match ret.checked_add(prod) {
     None => return None,
     Some(res) => ret = res,
    },
   }
  }

  Some(ret)
 }

 pub fn next(&self) -> Option<PermutationRemainder> {
  let ret = self.next_unsafe();
  ret.permutation_number()?;
  Some(ret)
 }

 pub fn prev(&self) -> Option<PermutationRemainder> {
  let mut vr = Vec::<Rem>::new();

  let mut underflow = 1;

  for idx in 0u8..self.vr.len() as u8 {
   let rem = self.vr[idx as usize].rem;
   let denominator = 2 + idx;

   let sum = if rem == 0 && underflow == 1 {
    denominator - 1
   } else {
    let uf_tmp = underflow;
    underflow = 0;
    rem - uf_tmp
   };
   vr.push(Rem { rem: sum })
  }

  if vr.last()?.rem == 0 {
   vr.pop();
  }

  Some(PermutationRemainder { vr })
 }

 pub fn permutation(&self) -> Permutation {
  let remainders = &self.vr;

  let mut ret = vec![];

  for i in 0..=remainders.len() {
   ret.push(i as u8);
  }

  for remainder in remainders.iter().enumerate() {
   for i in 0..remainder.1.rem {
    let idx = 1 + remainder.0 - (i as usize);

    ret.swap(idx, idx - 1);
   }
  }

  Permutation{ v: ret}
 }

 pub fn from_permutation(p: &Permutation) -> Option<Self> {

  if 0 == p.v.len() {
   return None;
  }

  let mut v = p.v.clone();

  for idx in (0..v.len()).rev() {
   if idx as u8 == v[idx] {
    v.pop();
   } else {
    break;
   }
  }

  let mut ret = Vec::<Rem>::new();

  while !v.is_empty() {
   let len = v.len();
   let denominator = (len - 1) as u8;
   let mut found = false;
   for idx in (0..v.len()).rev() {
    if denominator == v[idx] {
     v.remove(idx);
     ret.push(Rem {
      rem: denominator - idx as u8,
     });
     found = true;
     break;
    }
   }
   if !found {
    return None;
   }
  }

  if 0 != ret.len() {
   ret.pop();
  }

  ret.reverse();

  Some(Self { vr: ret })
 }

 pub fn from_permutation_unsafe(p: &Permutation) -> Self {
  let mut v = p.v.clone();

  for idx in (0..v.len()).rev() {
   if idx as u8 == v[idx] {
    v.pop();
   } else {
    break;
   }
  }

  let mut ret = Vec::<Rem>::new();

  while !v.is_empty() {
   let len = v.len();
   let denominator = (len - 1) as u8;
   for idx in (0..v.len()).rev() {
    if denominator == v[idx] {
     v.remove(idx);
     ret.push(Rem {
      rem: denominator - idx as u8,
     });
     break;
    }
   }
  }

  if 0 != ret.len() {
   ret.pop();
  }

  ret.reverse();

  Self { vr: ret }
 }
}

/// convenience function, calls PermutationRem::new(u).permutation()
pub fn permutation(u: u128) -> Permutation {
 PermutationRemainder::new(u).permutation()
}

/// convenience function, calls PermutationRem::from_permutation_unsafe(v).permutation_number_unsafe()
pub fn permutation_number_unsafe(p: &Permutation) -> u128 {
 PermutationRemainder::from_permutation_unsafe(p).permutation_number_unsafe()
}

/// convenience function, calls PermutationRem::from_permutation(v)?.permutation_number()
pub fn permutation_number(p: &Permutation) -> Option<u128> {
 PermutationRemainder::from_permutation(p)?.permutation_number()
}

/// fixes the length of a permutation vector, returns false
pub fn fix_length(len: u8, v: &mut Vec<u8>) -> bool {
 if (len as usize) < v.len() {
  return false;
 }
 for i in v.len()..(len as usize) {
  v.push(i as u8);
 }
 true
}
