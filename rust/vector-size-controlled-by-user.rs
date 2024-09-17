fn main() {
  let mut x = 10; // User controlled variable
  let mut data : Vec<u8> = Vec::with_capacity(x);
  let mut data2 : Vec<u8> = Vec::with_capacity(10);
  data.push(128);
  data2.push(128);

  println!("Data len {}", data.len());
}