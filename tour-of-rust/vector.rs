fn main() {
  // ベクタは要素可変のイテレータ
  // 要素を変更する場合、mutである必要がある
  let mut vec1 = Vec::<i32>::new();
  vec1.push(1);
  vec1.push(2);
  vec1.push(3);

  for num in vec1.iter() {
    println!("{}", num);
  }
}