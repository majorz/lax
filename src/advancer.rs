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

   pub fn one(&mut self, f: fn(&T) -> bool) -> Option<&T> {
      if let Some(ch) = self.slice.get(self.peek) {
         if f(ch) {
            self.peek += 1;
            return Some(ch);
         }
      }

      self.peek = self.start;
      None
   }

   pub fn zero_or_one(&mut self, f: fn(&T) -> bool) -> Option<()> {
      if let Some(ch) = self.slice.get(self.peek) {
         if f(ch) {
            self.peek += 1;
            return Some(());
         }
      }

      None
   }

   pub fn one_or_more(&mut self, f: fn(&T) -> bool) -> Option<()> {
      debug_assert!(self.peek <= self.slice.len());

      let mut span = 0;
      for ch in unsafe { self.slice.get_unchecked(self.peek..) } {
         if !f(ch) {
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

   pub fn zero_or_more(&mut self, f: fn(&T) -> bool) {
      debug_assert!(self.peek <= self.slice.len());

      let mut span = 0;
      for ch in unsafe { self.slice.get_unchecked(self.peek..) } {
         if !f(ch) {
            break;
         }
         span += 1;
      }

      self.peek += span;
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
