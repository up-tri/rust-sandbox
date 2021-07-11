struct Foo {
  x: i32,
}

fn do_something(f: Foo) {
  println!("{}", f.x);
}

fn do_something2(f: &Foo) {
  println!("{}", &f.x);
  // 宣言時に mut 修飾しても、借用変数は書き換えられない。
  // f.x = 32;
}

fn do_something3(f: &mut Foo) {
  println!("{}", &f.x);
  f.x = 32;
}

fn main() {
  let foo = Foo { x: 10 };

  // ここで foo の所有権が do_something へ移る。
  do_something(foo);

  // main に foo の所有権がないので下記はエラー
  // println!("{}", foo.x);

  let foo2 = Foo { x: 20 };
  // 関数へref(日: 参照)(ポインタ的なもの)を渡した場合
  // 所有権は移らない
  // "借用"と呼ぶ
  do_something2(&foo2);
  println!("{}", foo2.x);

  // foo3を借用する側で変更するには
  // mut 修飾を施し、変数の参照に &mut 修飾を施す
  let mut foo3 = Foo { x: 30 };
  do_something3(&mut foo3);
  println!("{}", foo3.x);
}