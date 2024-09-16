fn main() {
  let my_none_val : Option<i32> = None;
  let my_some_val = Some(1);

  my_none_val.and({println!("eager none"); Some(2)});
  my_some_val.and({println!("eager some"); Some(2)});
  
  println!("{:?}:{:?}", my_none_val, my_some_val);
}