#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
   Indent,
   Space,
   NewLine,
   DoubleAsterisk,
   DoubleEquals,
   ExclamationEquals,
   LessThanEquals,
   GreaterThanEquals,
   PlusEquals,
   MinusEquals,
   AsteriskEquals,
   SlashEquals,
   DoubleFullStop,
   FullStop,
   Equals,
   Plus,
   Minus,
   Asterisk,
   Slash,
   VerticalBar,
   Colon,
   Caret,
   ParenLeft,
   ParenRight,
   SquareBracketLeft,
   SquareBracketRight,
   LessThan,
   GreaterThan,
   CurlyBracketLeft,
   CurlyBracketRight,
   Comment,
   Apostrophe,
   Text,
   Identifier,
   Digits,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokMeta {
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type TokMatch = Option<(Tok, usize)>;

pub struct StrPeeker<'s> {
   chars: &'s [char],
   peek: &'s [char],
}

impl<'s> StrPeeker<'s> {
   fn new(chars: &'s [char]) -> Self {
      StrPeeker {
         chars: chars,
         peek: chars,
      }
   }

   fn commit(&mut self) -> usize {
      debug_assert!(self.peek.len() != self.chars.len());

      let span = self.chars.len() - self.peek.len();
      self.chars = self.peek;
      span
   }

   fn is_empty(&self) -> bool {
      debug_assert!(self.peek.len() == self.chars.len());

      self.chars.is_empty()
   }

   fn exact(&mut self, front: &[char]) -> Option<()> {
      let span = front.len();
      if self.peek.len() >= span && &self.peek[..span] == front {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.chars;
         None
      }
   }

   fn require(&mut self, f: fn(char) -> bool) -> Option<()> {
      if self.peek.is_empty() {
         self.peek = self.chars;
      }

      let ch = self.peek[0];
      if f(ch) {
         self.peek = &self.peek[1..];
         Some(())
      } else {
         self.peek = self.chars;
         None
      }
   }

   fn next(&mut self) -> Option<char> {
      if let Some(ch) = self.peek.first() {
         self.peek = &self.peek[1..];
         Some(*ch)
      } else {
         self.peek = self.chars;
         None
      }
   }

   fn multiple(&mut self, f: fn(char) -> bool) -> Option<()> {
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

   fn any(&mut self, f: fn(char) -> bool) {
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

fn space(peeker: &mut StrPeeker) -> TokMatch {
   peeker.multiple(|c| c == ' ')?;

   Some((Tok::Space, peeker.commit()))
}

fn identifier(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(!peeker.is_empty());

   peeker.require(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_')?;

   peeker.any(|c| {
      (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
   });

   Some((Tok::Identifier, peeker.commit()))
}

fn digits(peeker: &mut StrPeeker) -> TokMatch {
   peeker.multiple(|c| c >= '0' && c <= '9')?;

   Some((Tok::Digits, peeker.commit()))
}

macro_rules! exact {
   ($string:expr, $func:ident, $token_type:expr) => {
      fn $func(peeker: &mut StrPeeker) -> TokMatch {
         debug_assert!(!peeker.is_empty());
         peeker.exact($string)?;
         Some(($token_type, peeker.commit()))
      }
   }
}

exact!(&['\n'], new_line_n, Tok::NewLine);
exact!(&['\r', '\n'], new_line_rn, Tok::NewLine);
exact!(&['\r'], new_line_r, Tok::NewLine);
exact!(&['*', '*'], double_asterisk, Tok::DoubleAsterisk);
exact!(&['=', '='], double_equals, Tok::DoubleEquals);
exact!(&['!', '='], exclamation_equals, Tok::ExclamationEquals);
exact!(&['<', '='], less_than_equals, Tok::LessThanEquals);
exact!(&['>', '='], greater_than_equals, Tok::GreaterThanEquals);
exact!(&['+', '='], plus_equals, Tok::PlusEquals);
exact!(&['-', '='], minus_equals, Tok::MinusEquals);
exact!(&['*', '='], asterisk_equals, Tok::AsteriskEquals);
exact!(&['/', '='], slash_equals, Tok::SlashEquals);
exact!(&['.', '.'], double_full_stop, Tok::DoubleFullStop);
exact!(&['.'], full_stop, Tok::FullStop);
exact!(&['='], equals, Tok::Equals);
exact!(&['+'], plus, Tok::Plus);
exact!(&['-'], minus, Tok::Minus);
exact!(&['*'], asterisk, Tok::Asterisk);
exact!(&['/'], slash, Tok::Slash);
exact!(&['|'], vertical_bar, Tok::VerticalBar);
exact!(&[':'], colon, Tok::Colon);
exact!(&['^'], caret, Tok::Caret);
exact!(&['('], paren_left, Tok::ParenLeft);
exact!(&[')'], paren_right, Tok::ParenRight);
exact!(&['['], square_bracket_left, Tok::SquareBracketLeft);
exact!(&[']'], square_bracket_right, Tok::SquareBracketRight);
exact!(&['<'], less_than, Tok::LessThan);
exact!(&['>'], greater_than, Tok::GreaterThan);
exact!(&['{'], curly_bracket_left, Tok::CurlyBracketLeft);
exact!(&['}'], curly_backet_right, Tok::CurlyBracketRight);

const MATCHERS: &[fn(peeker: &mut StrPeeker) -> TokMatch] = &[
   space,
   new_line_n,
   new_line_rn,
   new_line_r,
   double_asterisk,
   double_equals,
   exclamation_equals,
   less_than_equals,
   greater_than_equals,
   plus_equals,
   minus_equals,
   asterisk_equals,
   slash_equals,
   double_full_stop,
   equals,
   plus,
   minus,
   asterisk,
   slash,
   vertical_bar,
   colon,
   caret,
   paren_left,
   paren_right,
   square_bracket_left,
   square_bracket_right,
   less_than,
   greater_than,
   curly_bracket_left,
   curly_backet_right,
   identifier,
   digits,
   full_stop,
];

pub fn tokenize(chars: &[char]) -> (Vec<Tok>, Vec<TokMeta>) {
   Tokenizer::new(chars).tokenize().destructure()
}

struct Tokenizer<'s> {
   toks: Vec<Tok>,
   toks_meta: Vec<TokMeta>,
   after_new_line: bool,
   pos: usize,
   line: usize,
   col: usize,
   peeker: StrPeeker<'s>,
}

impl<'s> Tokenizer<'s> {
   fn new(chars: &'s [char]) -> Self {
      let toks = vec![];
      let toks_meta = vec![];

      let after_new_line = true;

      let pos = 0;
      let line = 1;
      let col = 1;

      let peeker = StrPeeker::new(chars);

      Tokenizer {
         toks,
         toks_meta,
         after_new_line,
         pos,
         line,
         col,
         peeker,
      }
   }

   fn destructure(self) -> (Vec<Tok>, Vec<TokMeta>) {
      let Self {
         toks, toks_meta, ..
      } = self;

      (toks, toks_meta)
   }

   fn push(&mut self, tok: Tok, span: usize) {
      self.toks.push(tok);

      self.toks_meta.push(TokMeta {
         span: span,
         pos: self.pos,
         line: self.line,
         col: self.col,
      });

      self.col += span;
      self.pos += span;
   }

   fn tokenize(mut self) -> Self {
      while !self.peeker.is_empty() {
         if self.string().is_some() {
            self.after_new_line = false;
            continue;
         }

         if let Some((tok, span)) = match_tok(&mut self.peeker) {
            self.after_new_line = tok == Tok::NewLine;

            self.push(tok, span);

            if self.after_new_line {
               self.line += 1;
               self.col = 1;
            }
         } else {
            panic!(
               "Unrecognized token at line: {}, col: {}",
               self.line, self.col
            );
         }
      }

      self
   }

   fn string(&mut self) -> Option<()> {
      self.peeker.require(|c| c == '\'')?;

      self.push(Tok::Apostrophe, 1);

      let mut span = 0;
      loop {
         match self.peeker.next()? {
            '\\' => {
               self.peeker.require(|c| {
                  c == 'n' || c == '\'' || c == '\\' || c == 'r' || c == 't' || c == '0'
               })?;

               span += 1;
            }
            '\'' => {
               break;
            }
            '\n' | '\r' => {
               self.pos += span;
               self.col += span;

               panic!(
                  "New line in string at line: {}, col: {}",
                  self.line, self.col
               );
            }
            _ => {}
         }

         span += 1;
      }

      if span != 0 {
         self.push(Tok::Text, span);
      }

      self.push(Tok::Apostrophe, 1);

      self.peeker.commit();

      Some(())
   }
}

fn match_tok(peeker: &mut StrPeeker) -> TokMatch {
   for matcher in MATCHERS {
      if let Some((tok, span)) = matcher(peeker) {
         return Some((tok, span));
      }
   }

   None
}

#[cfg(test)]
mod tests {
   use super::*;

   fn as_chars(source: &str) -> Vec<char> {
      source.chars().collect()
   }

   macro_rules! m {
      ($matcher:ident, $input:expr) => (
         let chars = as_chars($input);
         let mut peeker = StrPeeker::new(&chars);
         assert_eq!($matcher(&mut peeker), None);
      );

      ($matcher:ident, $input:expr, $tok:expr, $span:expr) => (
         let chars = as_chars($input);
         let mut peeker = StrPeeker::new(&chars);
         assert_eq!($matcher(&mut peeker), Some(($tok, $span)));
      );
   }

   #[cfg(debug_assertions)]
   macro_rules! e {
      ($matcher:ident) => (
         $matcher(&mut StrPeeker::new(&[]));
      )
   }

   macro_rules! string {
      ($input:expr) => (
         let chars = as_chars($input);
         let mut tokenizer = Tokenizer::new(&chars);
         assert!(tokenizer.string().is_none());
      );

      ($input:expr, $span:expr) => (
         let chars = as_chars($input);
         let mut tokenizer = Tokenizer::new(&chars);
         assert!(tokenizer.string().is_some());
         let (toks, toks_meta) = tokenizer.destructure();
         if $span == 0 {
            assert!(toks.len() == 2);
            assert_eq!(toks[0], Tok::Apostrophe);
            assert_eq!(toks[1], Tok::Apostrophe);
            assert_eq!(toks_meta[0].span, 1);
            assert_eq!(toks_meta[1].span, 1);
         } else {
            assert!(toks.len() == 3);
            assert_eq!(toks[0], Tok::Apostrophe);
            assert_eq!(toks[1], Tok::Text);
            assert_eq!(toks[2], Tok::Apostrophe);
            assert_eq!(toks_meta[0].span, 1);
            assert_eq!(toks_meta[1].span, $span);
            assert_eq!(toks_meta[2].span, 1);
         }
      );
   }

   #[test]
   fn test_exact() {
      m!(double_asterisk, "*");
      m!(double_asterisk, "-**");
      m!(asterisk, "*", Tok::Asterisk, 1);
      m!(double_asterisk, "**", Tok::DoubleAsterisk, 2);
      m!(double_asterisk, "****", Tok::DoubleAsterisk, 2);
      m!(new_line_n, "\n\n", Tok::NewLine, 1);
      m!(new_line_rn, "\r\n", Tok::NewLine, 2);
      m!(new_line_r, "\r\r", Tok::NewLine, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_exact_empty() {
      e!(double_asterisk);
      e!(asterisk);
   }

   #[test]
   fn test_space() {
      m!(space, "");
      m!(space, "-");
      m!(space, "- ");
      m!(space, " ", Tok::Space, 1);
      m!(space, " -", Tok::Space, 1);
      m!(space, "   ", Tok::Space, 3);
      m!(space, "   -", Tok::Space, 3);
   }

   #[test]
   fn test_identifier() {
      m!(identifier, "-");
      m!(identifier, "-name");
      m!(identifier, "012abc");
      m!(identifier, "_", Tok::Identifier, 1);
      m!(identifier, "__", Tok::Identifier, 2);
      m!(identifier, "_.", Tok::Identifier, 1);
      m!(identifier, "_name", Tok::Identifier, 5);
      m!(identifier, "name", Tok::Identifier, 4);
      m!(identifier, "_NAME.", Tok::Identifier, 5);
      m!(identifier, "NAME.", Tok::Identifier, 4);
      m!(identifier, "a100", Tok::Identifier, 4);
      m!(identifier, "a100.", Tok::Identifier, 4);
      m!(identifier, "a_a_a.", Tok::Identifier, 5);
      m!(identifier, "aЯ", Tok::Identifier, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_identifier_empty() {
      e!(identifier);
   }

   #[test]
   fn test_digits() {
      m!(digits, "");
      m!(digits, " 1");
      m!(digits, "0", Tok::Digits, 1);
      m!(digits, "1", Tok::Digits, 1);
      m!(digits, "0000000000.", Tok::Digits, 10);
      m!(digits, "0123456789.", Tok::Digits, 10);
      m!(digits, "9876543210.", Tok::Digits, 10);
   }

   #[test]
   fn test_string() {
      string!("-");
      string!("-''");
      string!("'");
      string!("'a");
      string!("'ЯaЯaЯ");
      string!("'a\\'");
      string!("'a\\ '");
      string!("'aaa\\abbb'");
      string!("'aaa\\\"bbb'");
      string!("''", 0);
      string!("'a'", 1);
      string!("'Я'", 1);
      string!("'y̆'", 2);
      string!("'ЯaЯaЯ'", 5);
      string!("'''", 0);
      string!("'aaa bbb'", 7);
      string!("'aaa bbb' ", 7);
      string!("'aaa bbb'ccc", 7);
      string!("'aaa\\nbbb'", 8);
      string!("'aaa\\rbbb'", 8);
      string!("'aaa\\tbbb'", 8);
      string!("'aaa\\\\bbb'", 8);
      string!("'aaa\\\'bbb'", 8);
      string!("'aaa\\0bbb'", 8);
   }
}
