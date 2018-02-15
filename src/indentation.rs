use tokenize::{Tok, TokMeta};

pub fn calc_indentation(toks: &[Tok], toks_meta: &[TokMeta]) -> Option<usize> {
   let mut after_new_line = true;
   let mut prev = 0;

   let mut idents: [isize; 4] = [0, 0, 0, 0];

   for (tok, tok_meta) in toks.iter().zip(toks_meta.iter()) {
      if after_new_line && tok == &Tok::Space {
         let delta = tok_meta.span as isize - prev as isize;

         if delta == 0 {
            continue;
         }

         for (i, count) in idents.iter_mut().enumerate() {
            let ident = i as isize + 1;
            if delta % ident != 0 || delta > ident * 2 {
               *count -= 1;
            } else {
               *count += 1;
            }
         }

         prev = tok_meta.span;
      }

      after_new_line = tok == &Tok::NewLine;
   }

   for (i, count) in idents.iter().rev().enumerate() {
      if *count > 0 {
         return Some(4 - i);
      }
   }

   None
}
