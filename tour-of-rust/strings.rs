fn main() {
  let taro_name = String::from("Yamada Taro");
  println!("{} (length={})", taro_name, taro_name.len());

  let ja_taro_name = String::from("山田　太郎");
  // マルチバイトの文字数を数えるときは
  // println!("{} (length={})", ja_taro_name, ja_taro_name.len());
  // ではなく、こう↓
  println!("{} (length={})", ja_taro_name, ja_taro_name.len());
  println!("{} (chars.count={})", ja_taro_name, ja_taro_name.chars().count());
  let ja_taro_icon = String::from("😀");
  println!("{} (chars.count={})", ja_taro_icon, ja_taro_icon.chars().count());
  // サロゲートペアは2文字カウントされる。
  let ja_jiro_icon = String::from("🙆🏻");
  println!("{} (chars.count={})", ja_jiro_icon, ja_jiro_icon.chars().count());

  // precomposed character (日: 合成済み文字)の場合は
  // 冒頭に `use unicode_segmentation::UnicodeSegmentation;` 宣言が必要。
  // cf. https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust
}