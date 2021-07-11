// エラーである可能性を持つ構造体
fn do_something(i: i32) -> Result<i32, String> {
  if i == 10 {
    Ok(1)
  } else {
    Err(String::from("エラー"))
  }
}

fn main() -> Result<(), String> {
  let hoge: Option<i32> = None;
  let fuga: Option<i32> = Some(123);

  if fuga.is_some() {
    println!("fuga は非Noneです。")
  }
  
  if hoge.is_none() {
    println!("hoge はNoneです。")
  }

  let result = do_something(1);
  match result {
    Ok(v) => println!("OK: {}", v),
    Err(v) => println!("Err: {}", v),
  }

  let result2 = do_something(1)?;
  Ok(())
}