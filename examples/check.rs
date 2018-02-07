extern crate nel;

use std::fmt::Debug;

use nel::tokenize::*;

fn main() {
   let string = "(+100.0)";
   let tokens = tokenize(string);

/*
   tokens.iter().enumerate().for_each(
      |(i, token)| println!(
         "[{}] {:?} - {}", i, token.syn, token.span
      )
   );
*/

   let syns = tokens
      .iter()
      .map(|token| token.syn)
      .collect::<Vec<_>>();

   let mut peeker = Peeker::new(&syns);

   expression(&mut peeker);

   println!("remaining: {}", peeker.remaining());
   let count = syns.len() - peeker.remaining();
   syns[..count].iter().enumerate().for_each(
      |(i, item)|
         println!("[{}] {:?}", i, item)
   );

   println!("=======");

   syns[count..].iter().enumerate().for_each(
      |(i, item)|
         println!("[{}] {:?}", count + i, item)
   );
}

type Match = Option<(Syn, usize, usize)>;

fn expression(peeker: &mut Peeker<Syn>) -> Option<()> {
   single(peeker)?;

   let operators = [
      &Syn::Add,
      &Syn::Subtract,
      &Syn::Multiply,
      &Syn::Divide,
   ];

/*
   ((+100.0 - -100) + 100 + 400 + (30 - 10)) 100 600
   123    3   3  32   2 2   2 2   233   3321 1 1 1 1

*/
   peeker.optional(&Syn::Space);

   if peeker.optional_from_slice(&operators) {

      peeker.optional(&Syn::Space);

      single(peeker)?;
   }

   peeker.commit()
}

fn parens(peeker: &mut Peeker<Syn>) -> Option<()> {
   peeker.debug("pre-paren-left");

   peeker.next(&Syn::ParenLeft)?;

   peeker.optional(&Syn::Space);

   peeker.debug("pre-expression");

   expression(peeker)?;

   peeker.debug("post-expression");

   peeker.optional(&Syn::Space);

   peeker.next(&Syn::ParenRight)?;

   peeker.debug("post-paren-right");

   peeker.commit()
}

fn single(peeker: &mut Peeker<Syn>) -> Option<()> {
   peeker.push();

   let fns = [
      ident,
      number,
      parens,
   ];

   for f in &fns {
      if f(peeker).is_some() {
         return peeker.pop();
      }
   }

   peeker.pop()
}

fn ident(peeker: &mut Peeker<Syn>) -> Option<()> {
   peeker.next(&Syn::Ident)?;
   peeker.commit()
}

fn number(peeker: &mut Peeker<Syn>) -> Option<()> {
   if !peeker.optional(&Syn::Add) {
      peeker.optional(&Syn::Subtract);
   }

   peeker.next(&Syn::Digits)?;

   if peeker.optional(&Syn::Dot) {
      peeker.optional(&Syn::Digits);
   }

   peeker.commit()
}

pub struct Peeker<'s, T: 's> {
   stack: Vec<&'s [T]>,
   peek: &'s [T],
}

impl<'s, T> Peeker<'s, T> where T: PartialEq + Debug {
   fn new(input: &'s [T]) -> Self {
      Peeker {
         stack: vec![input],
         peek: input,
      }
   }

   fn push(&mut self) {
      self.stack.push(self.peek);
   }

   fn pop(&mut self) -> Option<()> {
      self.stack.pop();
      Some(())
   }

   fn commit(&mut self) -> Option<()> {
//      debug_assert!(self.peek.len() != self.stack.len());
      //let span = self.stack.len() - self.peek.len();
      debug_assert!(!self.stack.is_empty());
      *self.stack.last_mut().unwrap() = self.peek;
      Some(())
   }

   fn reset(&mut self) -> Option<()> {
      debug_assert!(!self.stack.is_empty());
      self.peek = self.stack.last().unwrap();
      None
   }

   fn debug(&self, moment: &str) {
      if let Some(first) = self.peek.first() {
         println!("{} / {:?}", moment, first);
      } else {
         println!("{} / none", moment);
      }
   }

   fn advance(&mut self, delta: usize) -> Option<()> {
      self.peek = &self.peek[delta..];
      Some(())
   }

   fn remaining(&self) -> usize {
      debug_assert!(!self.stack.is_empty());
      self.stack.last().unwrap().len()
   }

   fn next_fn(&mut self, f: fn(item: &T) -> bool) -> Option<()> {
      if let Some(first) = self.peek.first() {
         if f(first) {
            return self.advance(1);
         }
      }
      self.reset()
   }

   fn next(&mut self, item: &T) -> Option<()> {
      if let Some(first) = self.peek.first() {
         if *first == *item {
            return self.advance(1);
         }
      }
      self.reset()
   }

   fn optional_from_slice(&mut self, items: &[&T]) -> bool {
      if let Some(first) = self.peek.first() {
         for item in items {
            if *first == **item {
               self.advance(1);
               return true;
            }
         }
      }
      false
   }

   fn optional(&mut self, item: &T) -> bool {
      if let Some(first) = self.peek.first() {
         if *first == *item {
            self.advance(1);
            return true;
         }
      }
      false
   }
}
