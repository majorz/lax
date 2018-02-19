#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

pub struct Advancer<'s, T: 's> {
   slice: &'s [T],
   start: usize,
   peek: usize,
}

impl<'s, T> Advancer<'s, T> {
   pub fn new(slice: &'s [T]) -> Self {
      Advancer {
         slice: slice,
         start: 0,
         peek: 0,
      }
   }

   pub fn consume(&mut self) -> usize {
      debug_assert!(self.peek != self.start);
      let span = self.peek - self.start;
      self.start = self.peek;
      span
   }

   pub fn completed(&self) -> bool {
      debug_assert!(self.peek == self.start);

      self.peek == self.slice.len()
   }

   pub fn one<M>(&mut self, m: M) -> Option<&T>
   where
      M: Matcher<T>,
   {
      if let Some(item) = self.slice.get(self.peek) {
         if m.matches(item) {
            self.peek += 1;
            return Some(item);
         }
      }

      self.peek = self.start;
      None
   }

   pub fn zero_or_one<M>(&mut self, m: M) -> Option<()>
   where
      M: Matcher<T>,
   {
      if let Some(item) = self.slice.get(self.peek) {
         if m.matches(item) {
            self.peek += 1;
            return Some(());
         }
      }

      None
   }

   pub fn one_or_more<M>(&mut self, m: M) -> Option<()>
   where
      M: Matcher<T>,
   {
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
         self.peek = self.start;
         None
      }
   }

   pub fn zero_or_more<M>(&mut self, m: M)
   where
      M: Matcher<T>,
   {
      debug_assert!(self.peek <= self.slice.len());

      let mut span = 0;
      for item in unsafe { self.slice.get_unchecked(self.peek..) } {
         if !m.matches(item) {
            break;
         }
         span += 1;
      }

      self.peek += span;
   }
}

pub trait Matcher<T: ?Sized> {
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
