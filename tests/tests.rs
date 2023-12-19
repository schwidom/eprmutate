#[cfg(test)]
mod tests {

 use eprmutate::fix_length;
 use eprmutate::permutation;
 use eprmutate::permutation_number;
 use eprmutate::permutation_number_unsafe;
 use eprmutate::PermutationRemainder;

 #[test]
 fn test001() {
  assert_eq!(permutation(0).v.as_slice(), [0]);
  assert_eq!(permutation(1).v.as_slice(), [1, 0]);
  assert_eq!(permutation(2).v.as_slice(), [0, 2, 1]);
  assert_eq!(permutation(3).v.as_slice(), [1, 2, 0]);
  assert_eq!(permutation(4).v.as_slice(), [2, 0, 1]);
  assert_eq!(permutation(5).v.as_slice(), [2, 1, 0]);
  assert_eq!(permutation(6).v.as_slice(), [0, 1, 3, 2]);
 }

 #[test]
 fn test002() {
  assert_eq!(permutation(10).v.as_slice(), [2, 0, 3, 1]);
  assert_eq!(permutation(100).v.as_slice(), [4, 2, 0, 1, 3]);
  assert_eq!(permutation(1000).v.as_slice(), [2, 3, 0, 5, 4, 6, 1]);
  assert_eq!(permutation(10000).v.as_slice(), [6, 5, 2, 3, 0, 4, 7, 1]);
  assert_eq!(permutation(100000).v.as_slice(), [5, 6, 2, 3, 7, 0, 8, 4, 1]);
  assert_eq!(permutation(1000000).v.as_slice(), [5, 7, 8, 2, 3, 0, 6, 9, 4, 1]);
  assert_eq!(permutation(10000000).v.as_slice(), [5, 2, 9, 3, 8, 0, 4, 1, 10, 6, 7]);
  assert_eq!(permutation(10000001).v.as_slice(), [5, 2, 9, 3, 8, 1, 4, 0, 10, 6, 7]);
  assert_eq!(permutation(100000000).v.as_slice(), [5, 2, 3, 8, 9, 10, 0, 4, 6, 11, 7, 1]);
  assert_eq!(permutation(100000001).v.as_slice(), [5, 2, 3, 8, 9, 10, 1, 4, 6, 11, 7, 0]);
  assert_eq!(permutation(100000002).v.as_slice(), [5, 3, 0, 8, 9, 10, 1, 4, 6, 11, 7, 2]);
  assert_eq!(
   permutation(u128::MAX).v.as_slice(),
   [
    18, 13, 11, 8, 27, 16, 20, 22, 12, 24, 9, 1, 30, 3, 15, 23, 25, 2, 28, 19, 14, 29, 5, 10, 17,
    31, 32, 26, 33, 0, 4, 6, 21, 34, 7
   ]
  );
  assert_eq!(
   permutation(u128::MAX - 1).v.as_slice(),
   [
    18, 13, 11, 8, 27, 16, 20, 22, 12, 24, 9, 0, 30, 3, 15, 23, 25, 2, 28, 19, 14, 29, 5, 10, 17,
    31, 32, 26, 33, 1, 4, 6, 21, 34, 7
   ]
  );
 }

 #[test]
 fn test003() {
  let mut v: Vec<u8> = vec![1, 0];
  assert!(fix_length(3, &mut v));
  assert_eq!([1, 0, 2], v.as_slice());
  assert!(fix_length(4, &mut v));
  assert_eq!([1, 0, 2, 3], v.as_slice());
  assert!(fix_length(4, &mut v));
  assert_eq!([1, 0, 2, 3], v.as_slice());
  assert!(!fix_length(3, &mut v));
 }

 fn remainder_incrementation_unsafe(n: u128) {
  let p1 = PermutationRemainder::new(n);
  let p2 = PermutationRemainder::new(n + 1);

  let p3 = p1.next_unsafe();

  println!("{}", n);
  println!("p1 {:?}", p1);
  assert_eq!(n, p1.permutation_number_unsafe());
  println!("p2 {:?}", p2);
  println!("p3 {:?}", p3);
  assert_eq!(p2, p3);
 }

 fn remainder_decrementation_unsafe(n: u128) {
  println!("{}", n);
  let p1 = PermutationRemainder::new(n);
  let p2 = PermutationRemainder::new(n - 1);
  println!("p1 {:?}", p1);

  let p3 = p1.prev().unwrap();

  assert_eq!(n, p1.permutation_number_unsafe());
  println!("p2 {:?}", p2);
  println!("p3 {:?}", p3);
  assert_eq!(p2, p3);
 }

 #[test]
 fn test004() {
  for i in 0..1000 {
   remainder_incrementation_unsafe(i)
  }

  remainder_incrementation_unsafe(u128::MAX - 1);
 }

 #[test]
 fn test004_2() {
  for i in 1..1000 {
   remainder_decrementation_unsafe(i)
  }

  remainder_decrementation_unsafe(u128::MAX - 1);
  remainder_decrementation_unsafe(u128::MAX);

  assert_eq!(None, PermutationRemainder::new(0).prev());
 }

 #[test]
 fn test005() {
  let p1 = PermutationRemainder::new(u128::MAX);
  let p2 = p1.next_unsafe(); // the permutation number would be u128::MAX + 1
  assert_eq!(p1.permutation_number(), Some(u128::MAX));
  assert_eq!(p2.permutation_number(), None);
  assert_eq!(p1.next(), None);
  let p3 = p2.prev().unwrap();

  assert_eq!(p1, p3);
 }

 #[test]
 fn test006() {
  for u in 0..10000 {
   println!("{}", u);
   let pr1 = PermutationRemainder::new(u);
   let p = pr1.permutation();
   println!("{:?}", p);
   let pr2 = PermutationRemainder::from_permutation_unsafe(&p);
   let n = pr2.permutation_number().unwrap();
   println!("{:?}", pr1);
   println!("{:?}", pr2);
   assert_eq!(u, n);
  }

  let p = permutation(u128::MAX);
  assert_eq!(permutation_number_unsafe(&p), u128::MAX);
 }

 #[test]
 fn test007() {
  for u in 0..10000 {
   println!("{}", u);
   let pr1 = PermutationRemainder::new(u);
   let p = pr1.permutation();
   println!("{:?}", p);
   let pr2 = PermutationRemainder::from_permutation(&p).unwrap();
   let n = pr2.permutation_number().unwrap();
   println!("{:?}", pr1);
   println!("{:?}", pr2);
   assert_eq!(u, n);
  }

  let p = permutation(u128::MAX);
  assert_eq!(permutation_number(&p), Some(u128::MAX));
 }
}
