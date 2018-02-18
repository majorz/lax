#[macro_use]
extern crate criterion;

extern crate nel;

use criterion::Criterion;

use nel::advancer::*;

fn advancer_benchmark(c: &mut Criterion) {
   let long_chars: &[char] = &"aaaaabbbbb"
      .chars()
      .cycle()
      .take(100_000)
      .collect::<Vec<_>>();
   let short_chars: &[char] = &"ab".chars().cycle().take(100_000).collect::<Vec<_>>();

   c.bench_function("zero_or_more_1", |b| {
      b.iter(|| {
         let mut advancer = Advancer::new(long_chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.zero_or_more(|c| c == 'a');
               advancer.zero_or_more(|c| c == 'b');
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("zero_or_more_2", |b| {
      b.iter(|| {
         let mut advancer = Advancer2::new(long_chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.zero_or_more(|c| c == 'a');
               advancer.zero_or_more(|c| c == 'b');
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("one_or_more_1", |b| {
      b.iter(|| {
         let mut advancer = Advancer::new(&long_chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.one_or_more(|c| c == 'a').unwrap();
               advancer.one_or_more(|c| c == 'b').unwrap();
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("one_or_more_2", |b| {
      b.iter(|| {
         let mut advancer = Advancer2::new(&long_chars);

         while !advancer.completed() {
            for _ in 0..10 {
               advancer.one_or_more(|c| c == 'a').unwrap();
               advancer.one_or_more(|c| c == 'b').unwrap();
            }
            advancer.consume();
         }
      })
   });

   c.bench_function("one_1", |b| {
      b.iter(|| {
         let mut advancer = Advancer::new(&long_chars);

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

   c.bench_function("one_2", |b| {
      b.iter(|| {
         let mut advancer = Advancer2::new(&long_chars);

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

   c.bench_function("zero_or_one_1", |b| {
      b.iter(|| {
         let mut advancer = Advancer::new(&long_chars);

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

   c.bench_function("zero_or_one_2", |b| {
      b.iter(|| {
         let mut advancer = Advancer2::new(&long_chars);

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
