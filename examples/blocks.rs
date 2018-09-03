extern crate lax;

use std::fs::File;
use std::io::prelude::*;

use lax::indentation::estimate_indentation;
use lax::tokenize::*;

macro_rules! printi {
   ($fmt:expr, $pos:expr, $($arg:tt)*) => {
      println!(concat!("[{:03}] ", $fmt), $pos, $($arg)*);
   };
}

fn main() {
   let mut f = File::open("lax/block.lax").expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("something went wrong reading the file");

   let chars: Vec<_> = source.chars().collect();

   println!("Length: {}", chars.len(),);

   println!("----------------");

   let (toks, toks_meta, sol_indices) = tokenize(&chars);

   toks
      .iter()
      .enumerate()
      .for_each(|(i, tok)| printi!("{:?}", i, tok));

   println!("----------------");

   toks_meta
      .iter()
      .enumerate()
      .for_each(|(i, tok_meta)| printi!("{:?}", i, tok_meta));

   println!("----------------");

   sol_indices.iter().for_each(|pos| println!("{}", pos));

   println!("----------------");

   let module_indentation = estimate_indentation(&toks, &toks_meta, &sol_indices);

   println!("Indentation: {}", module_indentation);

   println!("================");
}
