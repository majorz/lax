pub struct Advancer2<'s> {
   chars: &'s [char],
   peek: &'s [char],
}

impl<'s> Advancer2<'s> {
   pub fn new(chars: &'s [char]) -> Self {
      Advancer2 {
         chars: chars,
         peek: chars,
      }
   }

   pub fn consume(&mut self) -> usize {
      debug_assert!(self.peek.len() != self.chars.len());

      let span = self.chars.len() - self.peek.len();
      self.chars = self.peek;
      span
   }

   pub fn completed(&self) -> bool {
      debug_assert!(self.peek.len() == self.chars.len());

      self.chars.is_empty()
   }

   pub fn one(&mut self, f: fn(char) -> bool) -> Option<char> {
      if let Some(ch) = self.peek.first() {
         if f(*ch) {
            self.peek = &self.peek[1..];
            return Some(*ch);
         }
      }

      self.peek = self.chars;
      None
   }

   pub fn zero_or_one(&mut self, f: fn(char) -> bool) -> Option<char> {
      if let Some(ch) = self.peek.first() {
         if f(*ch) {
            self.peek = &self.peek[1..];
            return Some(*ch);
         }
      }

      None
   }

   pub fn one_or_more(&mut self, f: fn(char) -> bool) -> Option<()> {
      let mut span = 0;
      for ch in self.peek {
         if !f(*ch) {
            break;
         }
         span += 1;
      }

      if span > 0 {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.chars;
         None
      }
   }

   pub fn zero_or_more(&mut self, f: fn(char) -> bool) {
      let mut span = 0;
      for ch in self.peek {
         if !f(*ch) {
            break;
         }
         span += 1;
      }

      self.peek = &self.peek[span..];
   }
}

pub struct Advancer<'s> {
   chars: &'s [char],
   start: usize,
   peek: usize,
}

impl<'s> Advancer<'s> {
   pub fn new(chars: &'s [char]) -> Self {
      Advancer {
         chars: chars,
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

      self.peek == self.chars.len()
   }

   pub fn one(&mut self, f: fn(char) -> bool) -> Option<char> {
      if let Some(ch) = self.chars.get(self.peek) {
         if f(*ch) {
            self.peek += 1;
            return Some(*ch);
         }
      }

      self.peek = self.start;
      None
   }

   pub fn zero_or_one(&mut self, f: fn(char) -> bool) -> Option<char> {
      if let Some(ch) = self.chars.get(self.peek) {
         if f(*ch) {
            self.peek += 1;
            return Some(*ch);
         }
      }

      None
   }

   pub fn one_or_more(&mut self, f: fn(char) -> bool) -> Option<()> {
      debug_assert!(self.peek < self.chars.len());

      let mut span = 0;
      for ch in unsafe { self.chars.get_unchecked(self.peek..) } {
         if !f(*ch) {
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

   pub fn zero_or_more(&mut self, f: fn(char) -> bool) {
      debug_assert!(self.peek < self.chars.len());

      let mut span = 0;
      for ch in unsafe { self.chars.get_unchecked(self.peek..) } {
         if !f(*ch) {
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
      let chars: Vec<_> = "abcd".chars().collect();
      let mut advancer = Advancer::new(&chars);
      let _ = advancer.consume();
   }
}
