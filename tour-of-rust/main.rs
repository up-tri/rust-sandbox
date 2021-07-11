// 定数
// 命名は大文字スネークケース
const HOGEHOGE_CONSTANT: f32 = 3.14159;

// Cよろしく、エントリーポイントは main 関数
// コメントは `//`、 もしくは `/** ... */` のよう。
fn main() {
  println!("Hello Rust world!!");
  println!("{}", HOGEHOGE_CONSTANT);

  // 型推論
  let guessing = 123;
  // 型を明示してみる
  let detecting: f64 = 456.789;
  // ちなみに、f64に整数を入れると怒られた。
  // let detecting: f64 = 456;
  
  println!("guessing: {}", guessing);
  println!("detecting: {}", detecting);

  // 再代入が許される"可変属性"を持つ変数
  // ちなみに、変数命名はスネークケース。
  let mut mutable_var = 123;
  println!("mutable_var: {}", mutable_var);
  mutable_var = 456;
  println!("mutable_var[updated]: {}", mutable_var);

  // 冒頭の `guessing` へ再代入した結果はこちら。
  // 
  // guessing = 456;
  // 
  //      |
  //   7  |   let guessing = 123;
  //      |       --------
  //      |       |
  //      |       first assignment to `guessing`
  //      |       help: consider making this binding mutable: `mut guessing`
  //   ...
  //   13 |   guessing = 456;
  //      |   ^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  // 
  // ＊変数名を再利用する場合は、`let guessing = 456` とすると再利用できる（使わない方がよさそう）
}