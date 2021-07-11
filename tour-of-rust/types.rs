fn main() {
  let a = 12u8; // u8型
  let b = 34u32; // u32型

  // 型のキャスト
  let c = (a as u32) + b;
  println!("{}", c);

}