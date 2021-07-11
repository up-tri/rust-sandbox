// if, match, fn などのブロック式は、値を返せる。
// もちろん単体のブロック式も可。

fn half(x: i32) -> i32 {
  // 行末に ; を書かなければその値が返される
  x / 2
}

fn main() {
  let target = 123;

  // 三項演算子的な使い方
  let result = if target % 2 == 0 { "even" } else { "odd" };
  println!("{}", result);

  // 即時関数的な使い方
  let result2 = {
    // ブロックスコープ内は分離されているので
    // ブロック外の `target` とは別物。
    let target = 123456;
    println!("in block [result]: {}", target);
    half(target)
  };
  println!("outof block [result]: {}", target);

  println!("{}", result2);
}