
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod tokenize;

use tokenize::tokenize;


fn main() {
   let mut f = File::open("nel/tokenize.nel").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

   let _tokens = tokenize(&input);
}
