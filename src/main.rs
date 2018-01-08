extern crate regex;

mod tokenize;
mod parse;

use std::fs::File;
use std::io::prelude::*;

use tokenize::tokenize;
use parse::Parser;


fn main() {
   let mut f = File::open("nel/tokenize.nel").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

   let tokens = tokenize(&input);

/*
   for (i, token) in tokens.iter().enumerate() {
      println!("[{}] {:?}", i, token.ty);
   }
*/

   Parser::init(&tokens, &input).parse();
}
