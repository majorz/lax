use tokenize::{Tok, TokMeta};

struct IndentationEstimator {
   after_new_line: bool,
   prev_space_span: usize,
   indents: [isize; 4],
}

impl IndentationEstimator {
   fn new() -> Self {
      IndentationEstimator {
         after_new_line: true,
         prev_space_span: 0,
         indents: [0, 0, 0, 0],
      }
   }

   fn count(&mut self, toks: &[Tok], toks_meta: &[TokMeta]) -> &mut Self {
      for (tok, tok_meta) in toks.iter().zip(toks_meta.iter()) {
         if self.after_new_line && tok == &Tok::Space {
            let delta = tok_meta.span as isize - self.prev_space_span as isize;

            self.update(delta);

            self.prev_space_span = tok_meta.span;
         }

         self.after_new_line = tok == &Tok::NewLine;
      }

      self
   }

   fn update(&mut self, delta: isize) {
      if delta == 0 {
         return;
      }

      for (i, count) in self.indents.iter_mut().enumerate() {
         let ident = i as isize + 1;
         if delta % ident != 0 || delta > ident * 2 {
            *count -= 1;
         } else {
            *count += 1;
         }
      }
   }

   fn estimate(&self) -> Option<usize> {
      for (i, count) in self.indents.iter().rev().enumerate() {
         if *count > 0 {
            return Some(4 - i);
         }
      }

      None
   }
}

pub fn estimate_indentation(toks: &[Tok], toks_meta: &[TokMeta]) -> Option<usize> {
   IndentationEstimator::new()
      .count(toks, toks_meta)
      .estimate()
}
