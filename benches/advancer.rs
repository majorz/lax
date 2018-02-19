#[macro_use]
extern crate criterion;

extern crate nel;

use criterion::Criterion;

use nel::advancer::*;

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
               advancer.zero_or_more(|c| c == 'a');
               advancer.zero_or_more(|c| c == 'b');
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
               advancer.one_or_more(|c| c == 'a').unwrap();
               advancer.one_or_more(|c| c == 'b').unwrap();
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
               advancer.one(|c| c == 'a').unwrap();
            }
            for _ in 0..5 {
               advancer.one(|c| c == 'b').unwrap();
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
               advancer.zero_or_one(|c| c == 'a').unwrap();
            }
            for _ in 0..5 {
               advancer.zero_or_one(|c| c == 'b').unwrap();
            }
            advancer.consume();
         }
      })
   });
}

criterion_group!(advancer_group, advancer_benchmark);
criterion_main!(advancer_group);
