use std::process::Command;

fn main() {
  
  Command::new("recho")
           .env("PATH",".")
           .arg("del hijo")
           .spawn()
           .expect("recho fallo al arrancar");
}
