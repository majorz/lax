extern crate nel;

use std::fs::File;
use std::io::prelude::*;

use nel::tokenize::tokenize;
use nel::indentation::estimate_indentation;
//use nel::parse::Parser;

fn main() {
   let mut f = File::open("nel/tokenize.nel").expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("something went wrong reading the file");

   let chars: Vec<_> = source.chars().collect();

   let (toks, toks_meta) = tokenize(&chars);

   match estimate_indentation(&toks, &toks_meta) {
      Some(indentation) => println!("Indentation is {} spaces", indentation),
      None => println!("No indentation used"),
   }

   /*
   for (i, token) in tokens.iter().enumerate() {
      println!("[{}] {:?} {}, {} - {}, {}", i, token.syn, token.pos, token.span, token.line, token.col);
   }
*/

   //   Parser::init(&tokens, &input).parse();
}
