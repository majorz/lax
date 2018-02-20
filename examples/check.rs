extern crate nel;

use nel::tokenize::*;
use nel::advancer::*;

type TokAdvancer<'a> = Advancer<'a, Tok>;

type SynFn = fn(&mut TokAdvancer) -> Option<usize>;

fn main() {
   let source = "(100.0+4)*0.2-0.6";

   let chars: Vec<_> = source.chars().collect();

   let (toks, _) = tokenize(&chars);

   let mut advancer = TokAdvancer::new(&toks);

   let result = expression(&mut advancer);

   if let Some(pos) = result {
      println!("Pos: {}", pos);
   }

   println!("Completed: {}", advancer.completed());

   println!("======");

   toks
      .iter()
      .enumerate()
      .for_each(|(i, item)| println!("[{:03}] {:?}", i, item));
}

fn expression(advancer: &mut TokAdvancer) -> Option<usize> {
   single(advancer)?;

   loop {
      if optional_right_hand(advancer).is_none() {
         break;
      }
   }

   Some(advancer.consume())
}

fn optional_right_hand(advancer: &mut TokAdvancer) -> Option<usize> {
   let operators: &[Tok] = &[Tok::Plus, Tok::Minus, Tok::Asterisk, Tok::Slash];

   let mut clone = advancer.clone();
   if clone.zero_or_one(operators).is_some() {
      if let Some(pos) = single(&mut clone) {
         advancer.advance(pos);
         return Some(pos);
      }
   }

   None
}

fn parens(advancer: &mut TokAdvancer) -> Option<usize> {
   advancer.one(Tok::ParenLeft)?;

   expression(advancer)?;

   advancer.one(Tok::ParenRight)?;

   Some(advancer.consume())
}

fn single(advancer: &mut TokAdvancer) -> Option<usize> {
   let fns = [identifier, number, parens];

   from_slice(advancer, &fns)?;

   Some(advancer.consume())
}

fn identifier(advancer: &mut TokAdvancer) -> Option<usize> {
   advancer.one(Tok::Identifier)?;
   Some(advancer.consume())
}

fn number(advancer: &mut TokAdvancer) -> Option<usize> {
   if advancer.zero_or_one(Tok::Digits).is_some() {
      if advancer.zero_or_one(Tok::FullStop).is_some() {
         advancer.zero_or_one(Tok::Digits);
      }
   } else {
      advancer.one(Tok::FullStop)?;
      advancer.one(Tok::Digits)?;
   }

   Some(advancer.consume())
}

fn from_slice(advancer: &mut TokAdvancer, fns: &[SynFn]) -> Option<usize> {
   for f in fns {
      let mut inner = advancer.clone();

      if let Some(pos) = f(&mut inner) {
         advancer.advance(pos);
         return Some(pos);
      }
   }

   advancer.reset();
   None
}
