extern crate lax;

use std::fs::File;
use std::io::prelude::*;

use lax::tokenize::tokenize;
use lax::indentation::estimate_indentation;
//use lax::parse::Parser;

fn main() {
   let mut f = File::open("lax/tokenize.lax").expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("something went wrong reading the file");

   let chars: Vec<_> = source.chars().collect();

   let (toks, toks_meta) = tokenize(&chars);

   println!(
      "Indentation is {} spaces",
      estimate_indentation(&toks, &toks_meta)
   );

   /*
   for (i, token) in tokens.iter().enumerate() {
      println!("[{}] {:?} {}, {} - {}, {}", i, token.syn, token.pos, token.span, token.line, token.col);
   }
*/

   //   Parser::init(&tokens, &input).parse();
}
