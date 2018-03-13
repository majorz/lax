#[derive(Clone)]
pub struct Advancer<'s, T: 's> {
   slice: &'s [T],
   start: usize,
   peek: usize,
}

impl<'s, T> Advancer<'s, T> {
   pub fn new(slice: &'s [T]) -> Self {
      Advancer {
         slice,
         start: 0,
         peek: 0,
      }
   }

   pub fn pos(&self) -> usize {
      self.peek
   }

   pub fn current(&self) -> &'s [T] {
      &self.slice[self.start..self.peek]
   }

   pub fn advance(&mut self, pos: usize) {
      debug_assert!(self.slice.len() >= pos);
      debug_assert!(pos > self.peek);
      self.peek = pos;
      self.start = pos;
   }

   pub fn reset(&mut self) {
      self.peek = self.start;
   }

   pub fn consume(&mut self) -> usize {
      debug_assert!(self.peek != self.start);
      self.start = self.peek;
      self.start
   }

   pub fn completed(&self) -> bool {
      debug_assert!(self.peek == self.start);
      self.peek == self.slice.len()
   }

   pub fn cannot_peek(&self) -> bool {
      self.peek == self.slice.len()
   }

   #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
   pub fn one<M: Matcher<T>>(&mut self, m: M) -> Option<&T> {
      if let Some(item) = self.slice.get(self.peek) {
         if m.matches(item) {
            self.peek += 1;
            return Some(item);
         }
      }

      self.reset();
      None
   }

   #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
   pub fn zero_or_one<M: Matcher<T>>(&mut self, m: M) {
      if let Some(item) = self.slice.get(self.peek) {
         if m.matches(item) {
            self.peek += 1;
         }
      }
   }

   #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
   pub fn one_or_more<M: Matcher<T>>(&mut self, m: M) -> Option<()> {
      debug_assert!(self.peek <= self.slice.len());

      let mut span = 0;
      for item in unsafe { self.slice.get_unchecked(self.peek..) } {
         if !m.matches(item) {
            break;
         }
         span += 1;
      }

      if span > 0 {
         self.peek += span;
         Some(())
      } else {
         self.reset();
         None
      }
   }

   #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
   pub fn zero_or_more<M: Matcher<T>>(&mut self, m: M) {
      debug_assert!(self.peek <= self.slice.len());

      let mut span = 0;
      for item in unsafe { self.slice.get_unchecked(self.peek..) } {
         if !m.matches(item) {
            break;
         }
         span += 1;
      }

      if span != 0 {
         self.peek += span;
      }
   }
}

pub trait Matcher<T> {
   fn matches(&self, &T) -> bool;
}

impl<T> Matcher<T> for T
where
   T: PartialEq,
{
   fn matches(&self, e: &T) -> bool {
      self == e
   }
}

impl<T> Matcher<T> for fn(&T) -> bool {
   fn matches(&self, e: &T) -> bool {
      self(e)
   }
}

impl<'a, T> Matcher<T> for &'a [T]
where
   T: PartialEq,
{
   fn matches(&self, e: &T) -> bool {
      for item in *self {
         if item == e {
            return true;
         }
      }

      false
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_consume_unmoved() {
      let slice: Vec<_> = "abcd".chars().collect();
      let mut advancer = Advancer::new(&slice);
      let _ = advancer.consume();
   }
}
