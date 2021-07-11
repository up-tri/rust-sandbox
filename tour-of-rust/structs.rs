// コンパイラのwarningメッセージを無効化する
#![allow(dead_code)]

struct Human {
  // Stringも構造体らしい(javaのStringに思想は似ているかも？)
  name: String,
  height: i32,
  weight: f64,
}

// ジェネリック型
// 身長の型をジェネリック型にしてみた。
struct DynamicHuman<T> {
  name: String,
  height: T,
}

struct Geo(f64, f64);

// 要素なしの構造体も宣言できる（何に使うんだ）
struct Hoge;

fn main() {
  let taro = Human {
    name: String::from("taro"),
    height: 170,
    weight: 65.4,
  };
  println!("{} is {}cm and {}kg.", taro.name, taro.height, taro.weight);
  
  let jiro = DynamicHuman::<f64> {
    name: String::from("jiro"),
    height: 167.89,
  };
  println!("{} is {}cm.", jiro.name, jiro.height);

  // タプルでも宣言できるが、補完がいまいち。
  // それぞれのメンバが何を表すのかも伝わらないので要注意、かな
  let tokyo_station = Geo(35.6809591, 139.7673068);
  println!("東京駅: ({}, {})", tokyo_station.0, tokyo_station.1);
}