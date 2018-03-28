use std::io::Read;
fn main() {
  let mut stdin = std::io::stdin();
  let mut buffer: [u8;16] = [0;16];
  let mut len=0;

  while let Ok(n) = stdin.read(&mut buffer ) {
      if n == 0 {
         break;
      }
      len +=n;
  }
  println!("{}", len);
}
