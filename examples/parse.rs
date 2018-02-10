extern crate nel;

use std::fs::File;
use std::io::prelude::*;

use nel::tokenize::tokenize;
//use nel::parse::Parser;


fn main() {
   let mut f = File::open("nel/tokenize.nel").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

   let _ = tokenize(&input);

/*
   for (i, token) in tokens.iter().enumerate() {
      println!("[{}] {:?} {}, {} - {}, {}", i, token.syn, token.pos, token.span, token.line, token.col);
   }
*/

//   Parser::init(&tokens, &input).parse();
}
