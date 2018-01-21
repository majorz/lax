#[macro_use]
extern crate criterion;

extern crate nel;

use criterion::Criterion;

use std::fs::File;
use std::io::prelude::*;

use nel::tokenize::tokenize;

fn regex_tokenize_benchmark(c: &mut Criterion) {
   let mut f = File::open("nel/tokenize.nel").expect("file not found");

   let mut input = String::new();
   f.read_to_string(&mut input)
      .expect("something went wrong reading the file");

   c.bench_function("regex tokenize", |b| b.iter(|| tokenize(&input)));
}

criterion_group!(benches, regex_tokenize_benchmark);
criterion_main!(benches);
