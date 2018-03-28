use std::fs;
fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() <=  1 {
    panic!("rrmdir nombre-directorio");
  } else {  
     match fs::remove_dir(&args[1]) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }


  }
}
