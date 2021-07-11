fn add(x: i32, y: i32) -> i32 {
  return x + y;
}

fn swap(x: i32, y: i32) -> (i32, i32) {
  return (y, x);
}

fn greet() -> () {
  println!("hello!");
}

fn main() {
  println!("{}", add(12, 34));
  // タプルを用いて複数の値を返却
  let result = swap(123, 456);
  // タプルの要素を取得するには、`<変数名>.<index>`
  println!("{}, {}", result.0, result.1);

  let result2 = greet();
  println!("{:?}", result2);
}