use std::io::Read;
use std::io::Write;
use std::process;
fn main() {
  let mut stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  let mut  buffer: [u8;16] = [0;16];
  while let Ok(leidos) = stdin.read(&mut buffer ) {
      if leidos == 0 {
         break;
      }
      let escritos=stdout.write(&buffer).unwrap();
      if escritos == 0 {
           break;
      }
  }
  let codigo:i32=0;
  exit(codigo);
}
