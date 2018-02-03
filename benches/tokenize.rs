#[macro_use]
extern crate criterion;

extern crate nel;

use criterion::Criterion;

use std::iter;
use std::fs::File;
use std::io::prelude::*;

use nel::tokenize::tokenize;

fn regex_tokenize_benchmark(c: &mut Criterion) {
   let mut f = File::open("nel/tokenize.nel")
      .expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("reading the file failed");

   let min = 2usize.pow(18);
   let count = 1 + (min - 1) / source.len();

   assert!(count * source.len() >= min && (count - 1) * source.len() < min);

   let input = iter::repeat(source)
      .take(count)
      .collect::<Vec<_>>()
      .join("\n");

   let lines = input.matches('\n').count();
   println!("{} lines/{} kb", lines, input.len() / 1024);

   c.bench_function("regex tokenize", |b| b.iter(|| tokenize(&input)));
}

criterion_group!(benches, regex_tokenize_benchmark);
criterion_main!(benches);
