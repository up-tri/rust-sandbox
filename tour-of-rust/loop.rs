fn main() {
  let mut x = 0;
  // while(true) 的な無限ループ
  loop {
    // `x++` とは書けない
    x += 1;
    if x == 10 { break; }
  }
  println!("{}", x);

  // 条件付き
  x = 0;
  while x != 10 {
    x += 1;
  }
  println!("{}", x);

  // forループ
  for i in 0..5 {
    // 0 <= i < 5 の範囲のイテレータ
    println!("{}", i);
  }

  for i in 0..=5 {
    // 0 <= i <= 5 の範囲
    println!("{}", i);
  }

  let mut j = 0;
  // breakに値を渡すことで、ループから値を返せる。
  // forやwhileでは使えない。
  let word = loop {
    j += 1;
    if j == 3 {
      break "4以上は認めない。";
    }
  // 式なので、ブロック末尾に `;` が必要！
  };

  println!("{}", word);
}