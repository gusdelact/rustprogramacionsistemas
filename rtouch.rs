use std::path::Path;
use std::fs::File;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() <=  1 {
    panic!("rtouch nombre-archivo");
  } else {
     let path = Path::new(".").join(&args[1]);

     let mut archivo = match File::create(&path) {
          Err(porque) => panic!("no se puede crear archivo {}:{}",args[1],porque),
          Ok(archivo) => archivo,
     };
  }
}
