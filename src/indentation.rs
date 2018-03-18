use std::collections::BTreeMap;

use tokenize::{Tok, TokMeta};

struct IndentationEstimator {
   deltas: BTreeMap<isize, isize>,
}

impl IndentationEstimator {
   fn new() -> Self {
      IndentationEstimator {
         deltas: BTreeMap::new(),
      }
   }

   fn count(mut self, toks: &[Tok], toks_meta: &[TokMeta]) -> Self {
      let mut prev_space_span = 0;
      let mut after_new_line = true;

      for (tok, tok_meta) in toks.iter().zip(toks_meta.iter()) {
         if after_new_line && tok == &Tok::Space {
            let delta = tok_meta.span as isize - prev_space_span as isize;

            if delta != 0 {
               self
                  .deltas
                  .entry(delta)
                  .and_modify(|e| *e += 1)
                  .or_insert(1);

               prev_space_span = tok_meta.span;
            }
         }

         after_new_line = tok == &Tok::LineEnd;
      }

      self
   }

   fn estimate(&self) -> usize {
      let mut target_max = 0;
      let mut matches_max = ::std::isize::MIN;

      for target in self.deltas.keys().rev() {
         let mut matches: isize = 0;

         let target = target.abs();

         for (delta, count) in &self.deltas {
            let matched = if *delta > 0 {
               *delta == target || *delta == 2 * target
            } else {
               let delta = delta.abs();
               delta % target == 0
            };

            if matched {
               matches += count;
            } else {
               matches -= count;
            }
         }

         if matches > matches_max {
            target_max = target;
            matches_max = matches;
         }
      }

      target_max as usize
   }
}

pub fn estimate_indentation(toks: &[Tok], toks_meta: &[TokMeta]) -> usize {
   IndentationEstimator::new()
      .count(toks, toks_meta)
      .estimate()
}

#[cfg(test)]
mod tests {
   use super::*;

   use tokenize::tokenize;

   macro_rules! assert_indentation {
      ($string:tt, $expected:tt) => {
         let source = indoc!($string);
         let chars: Vec<_> = source.chars().collect();
         let (toks, toks_meta) = tokenize(&chars);
         let estimated = IndentationEstimator::new().count(&toks, &toks_meta).estimate();
         assert_eq!(estimated, $expected);
      }
   }

   #[test]
   fn test_continuation() {
      assert_indentation!(
         "
         x
               x
            x
                  x
               x
                     x
         ",
         3
      );
   }

   #[test]
   fn test_no_indentation() {
      assert_indentation!(
         "
         x
         x
         x
         x
         ",
         0
      );
   }

   #[test]
   fn test_no_inner_spaces() {
      assert_indentation!(
         "
         x x x x x x
            x x x x x x
               x x x x x x
         ",
         3
      );
   }

   #[test]
   fn test_mixed_bigger() {
      assert_indentation!(
         "
         x
            x
              x
         ",
         3
      );
   }

   #[test]
   fn test_mixed_more() {
      assert_indentation!(
         "
         x
            x
              x
            x
         ",
         2
      );
   }

   #[test]
   fn test_mixed_block() {
      assert_indentation!(
         "
         x
           x
           x
           x
           x
              x
         ",
         3
      );
   }

   #[test]
   fn test_primes() {
      assert_indentation!(
         "
         x
           x
              x
                   x
                          x
                    x
         ",
         3
      );
   }

   #[test]
   fn test_one_off() {
      assert_indentation!(
         "
         x
            x
               x
                  x
                   x
         ",
         3
      );
   }

   #[test]
   fn test_triple() {
      assert_indentation!(
         "
         x
            x
                     x
               x
                 x
               x
         ",
         2
      );
   }
}
