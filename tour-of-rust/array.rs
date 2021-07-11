fn main() {
  // 配列の宣言は `[<要素の型>; <要素数>]`
  // サイズは不変
  let nums: [i32; 3] = [1, 2, 3];
  println!("{:?}", nums);
  println!("{}", nums[1]);
}