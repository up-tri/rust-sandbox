fn main() {
  let x = 123;

  match x {
    0 => {
      println!("ゼロです。");
    }
    // 1or2 的な
    1 | 2 => {
      println!("1か2です。");
    }
    // イテレータで範囲を指定できる
    3..=9 => {
      println!("3~9ですね。")
    }
    // ヒットした値を変数(matched)に再代入できる
    // エディタが再代入先(matched)を理解できていないので、利用時は要注意、かな
    matched @ 10..=1000 => {
      println!("matched: {}", matched);
    }
    _ => {
      println!("1000より大きいよ！");
    }
  }
}