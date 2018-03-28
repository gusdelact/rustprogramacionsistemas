use std::io::Write;


fn main() {
  let byte:u8=0x52u8;
  let dualbyte:u16=0x7573u16;
  let quadbyte:u32=0x74205275u32;
  let octobyte:u64=0x6c65732121210a0du64;
  let buffer: [u8;1] = [byte;1];
  let dualbuffer: [u8;2] = unsafe { std::mem::transmute(dualbyte.to_be())};
  let quadbuffer: [u8;4] = unsafe { std::mem::transmute(quadbyte.to_be())};
  let octobuffer: [u8;8] = unsafe { std::mem::transmute(octobyte.to_be())};
  let mut escritos:usize=0;
  escritos+= std::io::stdout().write(&buffer ).unwrap();
  escritos+= std::io::stdout().write(&dualbuffer ).unwrap();
  escritos+= std::io::stdout().write(&quadbuffer ).unwrap();
  escritos+= std::io::stdout().write(&octobuffer ).unwrap();

  println!("{0}",escritos );
}
