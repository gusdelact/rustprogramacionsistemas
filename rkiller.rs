use std::process::Command;

fn main() {
  let mut child= Command::new("ryes")
           .env("PATH",".")
           .spawn()
           .expect("ryes fallo al arrancar");

   println!("Hijo con id {}",child.id());
   child.kill().expect("ryes no esta ejecutandose"); 
   println!("ryes murio!");
}
