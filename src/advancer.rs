pub struct Advancer<'s> {
   chars: &'s [char],
   peek: &'s [char],
}

impl<'s> Advancer<'s> {
   pub fn new(chars: &'s [char]) -> Self {
      Advancer {
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

   pub fn exact(&mut self, front: &[char]) -> Option<()> {
      let span = front.len();
      if self.peek.len() >= span && &self.peek[..span] == front {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.chars;
         None
      }
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
