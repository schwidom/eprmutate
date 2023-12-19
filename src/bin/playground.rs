use eprmutate::*;

fn main() {
 for i in 0..=25 {
  println!("{}", i);
  println!("{:?}", PermutationRemainder::new(i));
 }
 for i in 0..=25 {
  println!("{}", i);
  println!("{:?}", permutation(i));
 }
 for i in 0..=1000 {
  // println!("{}", i);
  println!("{:?}", permutation(i));
 }
}
