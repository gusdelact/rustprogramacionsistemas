use std::process::Command;

fn main() {
  let mut child= Command::new("recho")
           .env("PATH",".")
           .arg("del hijo")
           .spawn()
           .expect("recho fallo al arrancar");

   println!("Hijo con id {}",child.id());
   let codigo = child.wait()
                     .expect("fallando al esperar al hijo");
    assert!(codigo.success());
    
}
