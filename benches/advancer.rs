#[macro_use]
extern crate criterion;

extern crate lax;

use criterion::Criterion;

use lax::advancer::*;

fn chars() -> Vec<char> {
   "aaaaabbbbb".chars().cycle().take(100_000).collect()
}

fn advancer_benchmark(c: &mut Criterion) {
   c.bench_function("zero_or_more", |b| {
      let chars = chars();
      b.iter(|| {
         let mut advancer = Advancer::new(&chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.zero_or_more('a');
               advancer.zero_or_more('b');
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("one_or_more", |b| {
      let chars = chars();
      b.iter(|| {
         let mut advancer = Advancer::new(&chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.one_or_more('a').unwrap();
               advancer.one_or_more('b').unwrap();
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("one", |b| {
      let chars = chars();
      b.iter(|| {
         let mut advancer = Advancer::new(&chars);

         while !advancer.completed() {
            for _ in 0..5 {
               advancer.one('a').unwrap();
            }
            for _ in 0..5 {
               advancer.one('b').unwrap();
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("zero_or_one", |b| {
      let chars = chars();
      b.iter(|| {
         let mut advancer = Advancer::new(&chars);

         while !advancer.completed() {
            for _ in 0..5 {
               advancer.zero_or_one('a');
            }
            for _ in 0..5 {
               advancer.zero_or_one('b');
            }
            advancer.consume();
         }
      })
   });
}

criterion_group!(advancer_group, advancer_benchmark);
criterion_main!(advancer_group);
