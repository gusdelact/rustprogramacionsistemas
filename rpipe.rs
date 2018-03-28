use std::process::{Command, Stdio};
use std::io::Write;
use std::io::Read;
use std::error::Error;
static TEXTO: &'static str ="tres tristes tigres tragaron trigo de un trigal\n";
fn main() {
   let hijo =  Command::new("rwc")
                     .env("PATH",".")
                     .stdin(Stdio::piped())
                     .stdout(Stdio::piped())
                     .spawn() 
                     .expect("rwc fallo en ejecucion:");

  match hijo.stdin.unwrap().write_all(TEXTO.as_bytes() ) {
        Err(why) => panic!("no puedo enviar a rwc stdin: {}",
                           why.description()),
        Ok(_) => println!("enviando TEXTO a rwc"),
  }

  let mut s = String::new();
  match hijo.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("no puedo leer rwc stdout: {}",
                           why.description()),
        Ok(_) => print!("rwc respondio :\n{}", s),
    }
}
