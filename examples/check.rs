extern crate nel;

use nel::tokenize::*;

fn main() {
   let source = "(100.0+4)*0.2-0.6";

   let chars: Vec<_> = source.chars().collect();

   let (toks, _) = tokenize(&chars);

   let mut peeker = Peeker::new(&toks);

   let result = expression(&mut peeker);

   if result.is_some() {
      println!("done");
   } else {
      println!("nope");
   }

   println!("remaining: {}", peeker.remaining());

   let count = toks.len() - peeker.remaining();

   toks[..count]
      .iter()
      .enumerate()
      .for_each(|(i, item)| println!("[{:03}] {:?}", i, item));

   println!("======");

   toks[count..]
      .iter()
      .enumerate()
      .for_each(|(i, item)| println!("[{:03}] {:?}", count + i, item));
}

fn expression(peeker: &mut Peeker) -> Option<usize> {
   let mut pos = peeker.descend(single)?;

   loop {
      let operators = [&Tok::Plus, &Tok::Minus, &Tok::Asterisk, &Tok::Slash];

      if peeker.optional_from_slice(&operators).is_some() {
         if let Some(i) = peeker.descend(single) {
            pos = i;
         } else {
            break;
         }
      } else {
         break;
      }
   }

   peeker.adjust(pos)
}

fn parens(peeker: &mut Peeker) -> Option<usize> {
   peeker.next(&Tok::ParenLeft)?;

   peeker.descend(expression)?;

   peeker.next(&Tok::ParenRight)?;

   peeker.commit()
}

fn single(peeker: &mut Peeker) -> Option<usize> {
   let fns = [identifier, number, parens];

   for f in &fns {
      if let Some(pos) = peeker.descend(*f) {
         return peeker.adjust(pos);
      }
   }

   peeker.reset()
}

fn identifier(peeker: &mut Peeker) -> Option<usize> {
   peeker.next(&Tok::Identifier)?;
   peeker.commit()
}

fn number(peeker: &mut Peeker) -> Option<usize> {
   if peeker.optional(&Tok::Digits).is_some() {
      if peeker.optional(&Tok::FullStop).is_some() {
         peeker.optional(&Tok::Digits);
      }
   } else {
      peeker.next(&Tok::FullStop)?;
      peeker.next(&Tok::Digits)?;
   }

   peeker.commit()
}

#[derive(Clone)]
pub struct Peeker<'s> {
   input: &'s [Tok],
   start: usize,
   peek: usize,
}

impl<'s> Peeker<'s> {
   fn new(input: &'s [Tok]) -> Self {
      Peeker {
         input: input,
         start: 0,
         peek: 0,
      }
   }

   fn step(&mut self) -> Option<usize> {
      self.peek += 1;
      Some(self.peek)
   }

   fn current(&self) -> Option<&'s Tok> {
      self.input.get(self.peek)
   }

   fn commit(&mut self) -> Option<usize> {
      debug_assert!(self.peek != self.start);
      self.start = self.peek;
      Some(self.start)
   }

   fn adjust(&mut self, pos: usize) -> Option<usize> {
      debug_assert!(self.start != pos);
      self.start = pos;
      self.peek = pos;
      Some(pos)
   }

   fn reset(&mut self) -> Option<usize> {
      self.peek = self.start;
      None
   }

   fn remaining(&self) -> usize {
      self.input.len() - self.start
   }

   #[allow(dead_code)]
   fn next_fn(&mut self, f: fn(item: &Tok) -> bool) -> Option<usize> {
      if let Some(current) = self.current() {
         if f(current) {
            return self.step();
         }
      }
      self.reset()
   }

   fn next(&mut self, item: &Tok) -> Option<usize> {
      if let Some(current) = self.current() {
         if *current == *item {
            return self.step();
         }
      }
      self.reset()
   }

   fn optional_from_slice(&mut self, items: &[&Tok]) -> Option<usize> {
      if let Some(current) = self.current() {
         for item in items {
            if *current == **item {
               return self.step();
            }
         }
      }
      None
   }

   fn optional(&mut self, item: &Tok) -> Option<usize> {
      if let Some(current) = self.current() {
         if *current == *item {
            return self.step();
         }
      }
      None
   }

   fn descend(&mut self, f: fn(&mut Peeker) -> Option<usize>) -> Option<usize> {
      let mut peeker = self.clone();
      self.peek = f(&mut peeker)?;
      Some(self.peek)
   }
}
