fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() <=  1 {
    println!("recho");
  } else {
    println!("{}",args[1]);
   }
}
