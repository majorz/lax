#[macro_use]
extern crate criterion;

extern crate nel;

use criterion::Criterion;

use nel::advancer::*;

fn advancer_benchmark(c: &mut Criterion) {
   let letters: Vec<_> = ('a' as u8..'z' as u8 + 1).map(|u| u as char).collect();
   let chars: Vec<_> = letters.iter().cycle().take(100_000).map(|c| *c).collect();

   c.bench_function("advancer", |b| b.iter(|| {
      let mut advancer = Advancer::new(&chars);
      advancer.zero_or_more(|c| c >= 'a' && c <= 'z');
      advancer.consume();
   }));
}

criterion_group!(advancer_group, advancer_benchmark);
criterion_main!(advancer_group);
