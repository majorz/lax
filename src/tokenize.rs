
const INDENT: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Syn {
   Indentation,
   NewLine,
   Power,
   Equal,
   Unequal,
   LessEqual,
   GreaterEqual,
   AddAssign,
   SubtractAssign,
   MultiplyAssign,
   DivideAssign,
   Range,
   Dot,
   Assign,
   Add,
   Subtract,
   Multiply,
   Divide,
   And,
   Or,
   Not,
   Bar,
   Colon,
   ParenLeft,
   ParenRight,
   BracketLeft,
   BracketRight,
   AngleLeft,
   AngleRight,
   CurlyLeft,
   CurlyRight,
   Comment,
   Accent,
   String,
   Ident,
   Symbol,
   Digits,
   Fn,
   Loop,
   Match,
   If,
   Ef,
   El,
   Break,
   Ret,
   For,
   In,
}

const KEYWORDS: [(&'static str, Syn); 13] = [
   ("fn",     Syn::Fn),
   ("loop",   Syn::Loop),
   ("match",  Syn::Match),
   ("if",     Syn::If),
   ("ef",     Syn::Ef),
   ("el",     Syn::El),
   ("break",  Syn::Break),
   ("ret",    Syn::Ret),
   ("for",    Syn::For),
   ("in",     Syn::In),
   ("and",    Syn::And),
   ("or",     Syn::Or),
   ("not",    Syn::Not),
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
   pub syn: Syn,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type SynMatch = Option<(Syn, usize, usize)>;

pub struct StrPeeker<'s> {
   input: &'s str,
   peek: &'s str,
}

impl<'s> StrPeeker<'s> {
   fn new(input: &'s str) -> Self {
      StrPeeker {
         input: input,
         peek: input,
      }
   }

   fn commit(&mut self) -> usize {
      debug_assert!(self.peek.len() != self.input.len());

      let span = self.input.len() - self.peek.len();
      self.input = self.peek;
      span
   }

   fn has_more(&mut self) -> bool {
      self.peek.len() > 0
   }

   fn has_at_least(&mut self, span: usize) -> bool {
      self.peek.len() >= span
   }

   fn exact(&mut self, front: &'static str) -> Option<()> {
      let span = front.len();
      if self.peek.len() >= span && &self.peek[..span] == front {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.input;
         None
      }
   }

   fn require(&mut self, f: fn(b: u8) -> bool) -> Option<()> {
      if self.peek.is_empty() {
         self.peek = self.input;
      }

      let byte = self.peek.as_bytes()[0];
      if f(byte) {
         self.peek = &self.peek[1..];
         Some(())
      } else {
         self.peek = self.input;
         None
      }
   }

   fn multiple(&mut self, f: fn(b: u8) -> bool) -> Option<()> {
      let mut span = 0;
      for byte in self.peek.as_bytes() {
         if !f(*byte) {
            break;
         }
         span += 1;
      };

      if span > 0 {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.input;
         None
      }
   }

   fn any(&mut self, f: fn(b: u8) -> bool) {
      let mut span = 0;
      for byte in self.peek.as_bytes() {
         if !f(*byte) {
            break;
         }
         span += 1;
      };

      self.peek = &self.peek[span..];
   }

   fn char_advancer<'p>(&'p mut self) -> CharAdvancer<'p, 's> {
      CharAdvancer::new(self)
   }

   fn reveal<'p>(&'p self) -> &'s str {
      &self.input[..self.input.len() - self.peek.len()]
   }

   fn char_indices<'p>(&'p self) -> ::std::str::CharIndices<'s> {
      self.peek.char_indices()
   }

   fn reset(&mut self) {
      self.peek = self.input;
   }

   fn advance(&mut self, delta: usize) {
      self.peek = &self.peek[delta..];
   }
}

pub struct CharAdvancer<'p, 's: 'p> {
   peeker: &'p mut StrPeeker<'s>,
   indices: ::std::str::CharIndices<'s>,
   end: usize,
   chars: usize,
}

impl<'p, 's> CharAdvancer<'p, 's> {
   fn new(peeker: &'p mut StrPeeker<'s>) -> Self {
      let indices = peeker.char_indices();
      CharAdvancer {
         peeker: peeker,
         indices: indices,
         end: 0,
         chars: 0,
      }
   }

   fn next(&mut self) -> Option<char> {
      if let Some((pos, ch)) = self.indices.next() {
         self.end = pos + 1;
         self.chars += 1;
         Some(ch)
      } else {
         self.peeker.reset();
         None
      }
   }

   fn require(&mut self, f: fn(ch: char) -> bool) -> Option<()> {
      if let Some((pos, ch)) = self.indices.next() {
         if f(ch) {
            self.end = pos + 1;
            self.chars += 1;
            return Some(());
         }
      }

      self.peeker.reset();
      None
   }

   fn commit(&mut self) -> usize {
      self.peeker.advance(self.end);
      let chars = self.chars;
      self.end = 0;
      self.chars = 0;
      chars
   }
}

fn space(peeker: &mut StrPeeker) -> Option<usize> {
   peeker.multiple(|b| b == b' ')?;

   Some(peeker.commit())
}

fn string(peeker: &mut StrPeeker) -> SynMatch {
   debug_assert!(peeker.has_more());

   peeker.require(|b| b == b'\'')?;

   let chars = {
      let mut advancer = peeker.char_advancer();
      loop {
         match advancer.next()? {
            '\\' => {
               advancer.require(
                  |c| {
                     c == 'n' || c == '\'' || c == '\\' ||
                     c == 'r' || c == 't' || c == '0'
                  }
               )?;
            },
            '\'' => {
               break;
            },
            _ => {}
         }
      }
      advancer.commit()
   };

   let span = peeker.commit();
   Some((Syn::String, span, chars + 1))
}

fn symbol(peeker: &mut StrPeeker) -> SynMatch {
   debug_assert!(peeker.has_more());

   if !peeker.has_at_least(2) {
      return None;
   }

   peeker.require(|b| b == b'^')?;

   advance_ident(peeker)?;

   if Syn::Ident == syn_from_ident(&peeker.reveal()[1..]) {
      let span = peeker.commit();
      Some((Syn::Symbol, span, span))
   } else {
      None
   }
}

fn ident(peeker: &mut StrPeeker) -> SynMatch {
   debug_assert!(peeker.has_more());

   advance_ident(peeker)?;

   let syn = syn_from_ident(peeker.reveal());
   let span = peeker.commit();
   Some((syn, span, span))
}

fn advance_ident(peeker: &mut StrPeeker) -> Option<()> {
   peeker.require(
      |b| {
         (b >= b'a' && b <= b'z') ||
         (b >= b'A' && b <= b'Z') ||
         b == b'_'
      }
   )?;

   peeker.any(
      |b| {
         (b >= b'a' && b <= b'z') ||
         (b >= b'A' && b <= b'Z') ||
         (b >= b'0' && b <= b'9') ||
         b == b'_'
      }
   );

   Some(())
}

fn syn_from_ident(ident: &str) -> Syn {
   for &(keyword, syn) in KEYWORDS.iter() {
      if keyword == ident {
         return syn;
      }
   }

   Syn::Ident
}

fn digits(peeker: &mut StrPeeker) -> SynMatch {
   peeker.multiple(|b| b >= b'0' && b <= b'9')?;

   let span = peeker.commit();
   Some((Syn::Digits, span, span))
}

macro_rules! exact {
   ($string:expr, $func:ident, $token_type:expr) => {
      fn $func(peeker: &mut StrPeeker) -> SynMatch {
         debug_assert!(peeker.has_more());
         peeker.exact($string)?;
         let span = peeker.commit();
         Some(($token_type, span, span))
      }
   }
}

exact!("\n", new_line_n, Syn::NewLine);
exact!("\r\n", new_line_rn, Syn::NewLine);
exact!("\r", new_line_r, Syn::NewLine);
exact!("**", power, Syn::Power);
exact!("==", equal, Syn::Equal);
exact!("!=", unequal, Syn::Unequal);
exact!("<=", less_equal, Syn::LessEqual);
exact!(">=", greater_equal, Syn::GreaterEqual);
exact!("+=", add_assign, Syn::AddAssign);
exact!("-=", subtract_assign, Syn::SubtractAssign);
exact!("*=", multiply_assign, Syn::MultiplyAssign);
exact!("/=", divide_assign, Syn::DivideAssign);
exact!("..", range, Syn::Range);
exact!(".", dot, Syn::Dot);
exact!("=", assign, Syn::Assign);
exact!("+", add, Syn::Add);
exact!("-", subtract, Syn::Subtract);
exact!("*", multiply, Syn::Multiply);
exact!("/", divide, Syn::Divide);
exact!("|", bar, Syn::Bar);
exact!(":", colon, Syn::Colon);
exact!("(", paren_left, Syn::ParenLeft);
exact!(")", paren_right, Syn::ParenRight);
exact!("[", bracket_left, Syn::BracketLeft);
exact!("]", bracket_right, Syn::BracketRight);
exact!("<", angle_left, Syn::AngleLeft);
exact!(">", angle_right, Syn::AngleRight);
exact!("{", curly_left, Syn::CurlyLeft);
exact!("}", curly_right, Syn::CurlyRight);

const MATCHERS: [fn(peeker: &mut StrPeeker) -> SynMatch; 33] = [
   new_line_n,
   new_line_rn,
   new_line_r,
   power,
   equal,
   unequal,
   less_equal,
   greater_equal,
   add_assign,
   subtract_assign,
   multiply_assign,
   divide_assign,
   range,
   assign,
   add,
   subtract,
   multiply,
   divide,
   bar,
   colon,
   paren_left,
   paren_right,
   bracket_left,
   bracket_right,
   angle_left,
   angle_right,
   curly_left,
   curly_right,
   ident,
   digits,
   dot,
   symbol,
   string,
];

pub fn tokenize(input: &str) -> Vec<Token> {
   let mut tokens = vec![];

   let mut after_new_line = true;

   let mut pos = 0;
   let mut line = 1;
   let mut col = 1;

   let mut peeker = StrPeeker::new(input);

   while pos < input.len() {
      if let Some(span) = space(&mut peeker) {
         if after_new_line {
            assert!(span % INDENT == 0);
            let indents = span / INDENT;
            for _ in 0..indents {
               pos += INDENT;
               col += INDENT;
               tokens.push(
                  Token {
                     syn: Syn::Indentation,
                     span: INDENT,
                     pos,
                     line,
                     col,
                  }
               );
            }
         } else {
            pos += span;
            col += span;
         }
      }

      if let Some((syn, span, chars)) = match_syn(&mut peeker) {
         tokens.push(
            Token {
               syn,
               span,
               pos,
               line,
               col,
            }
         );

         after_new_line = syn == Syn::NewLine;
         if after_new_line {
            line += 1;
            col = 1;
         }

         pos += span;
         col += chars;
      } else {
         panic!("Unrecognized token at line: {}, col: {}", line, col);
      }
   }

   tokens
}

fn match_syn(peeker: &mut StrPeeker) -> SynMatch {
   for matcher in MATCHERS.iter() {
      if let Some((syn, span, chars)) = matcher(peeker) {
         return Some((syn, span, chars));
      }
   }

   None
}

#[cfg(test)]
mod tests {
   use super::*;

   macro_rules! m {
      ($matcher:ident, $input:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!($matcher(&mut peeker), None);
      );

      ($matcher:ident, $input:expr, $syn:expr, $span:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!($matcher(&mut peeker), Some(($syn, $span, $span)));
      );

      ($matcher:ident, $input:expr, $syn:expr, $span:expr, $chars:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!($matcher(&mut peeker), Some(($syn, $span, $chars)));
      );
   }

   macro_rules! space {
      ($input:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!(space(&mut peeker), None);
      );

      ($input:expr, $span:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!(space(&mut peeker), Some($span));
      );
   }

   #[cfg(debug_assertions)]
   macro_rules! e {
      ($matcher:ident) => (
         $matcher(&mut StrPeeker::new(""));
      )
   }

   #[test]
   fn test_exact() {
      m!(power, "*");
      m!(power, "-**");
      m!(multiply, "*", Syn::Multiply, 1);
      m!(power, "**", Syn::Power, 2);
      m!(power, "****", Syn::Power, 2);
      m!(new_line_n, "\n\n", Syn::NewLine, 1);
      m!(new_line_rn, "\r\n", Syn::NewLine, 2);
      m!(new_line_r, "\r\r", Syn::NewLine, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_exact_empty() {
      e!(power);
      e!(multiply);
   }

   #[test]
   fn test_space() {
      space!("");
      space!("-");
      space!("- ");
      space!(" ", 1);
      space!(" -", 1);
      space!("   ", 3);
      space!("   -", 3);
   }

   #[test]
   fn test_symbol() {
      m!(symbol, "-");
      m!(symbol, "-^name");
      m!(symbol, "^012abc");
      m!(symbol, "^");
      m!(symbol, "^-");
      m!(symbol, "^Я");
      m!(symbol, "^for");
      m!(symbol, "^_", Syn::Symbol, 2);
      m!(symbol, "^__", Syn::Symbol, 3);
      m!(symbol, "^_.", Syn::Symbol, 2);
      m!(symbol, "^_name", Syn::Symbol, 6);
      m!(symbol, "^name", Syn::Symbol, 5);
      m!(symbol, "^_NAME.", Syn::Symbol, 6);
      m!(symbol, "^NAME.", Syn::Symbol, 5);
      m!(symbol, "^a100", Syn::Symbol, 5);
      m!(symbol, "^a100.", Syn::Symbol, 5);
      m!(symbol, "^a_a_a.", Syn::Symbol, 6);
      m!(symbol, "^aЯ", Syn::Symbol, 2);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_symbol_empty() {
      e!(symbol);
   }

   #[test]
   fn test_ident() {
      m!(ident, "-");
      m!(ident, "-name");
      m!(ident, "012abc");
      m!(ident, "_", Syn::Ident, 1);
      m!(ident, "__", Syn::Ident, 2);
      m!(ident, "_.", Syn::Ident, 1);
      m!(ident, "_name", Syn::Ident, 5);
      m!(ident, "name", Syn::Ident, 4);
      m!(ident, "_NAME.", Syn::Ident, 5);
      m!(ident, "NAME.", Syn::Ident, 4);
      m!(ident, "a100", Syn::Ident, 4);
      m!(ident, "a100.", Syn::Ident, 4);
      m!(ident, "a_a_a.", Syn::Ident, 5);
      m!(ident, "aЯ", Syn::Ident, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_ident_empty() {
      e!(ident);
   }

   #[test]
   fn test_keyword() {
      m!(ident, "fn", Syn::Fn, 2);
      m!(ident, "loop", Syn::Loop, 4);
      m!(ident, "match", Syn::Match, 5);
      m!(ident, "if", Syn::If, 2);
      m!(ident, "ef", Syn::Ef, 2);
      m!(ident, "el", Syn::El, 2);
      m!(ident, "break", Syn::Break, 5);
      m!(ident, "ret", Syn::Ret, 3);
      m!(ident, "for", Syn::For, 3);
      m!(ident, "in", Syn::In, 2);
      m!(ident, "and", Syn::And, 3);
      m!(ident, "or", Syn::Or, 2);
      m!(ident, "not", Syn::Not, 3);
      m!(ident, "for", Syn::For, 3);
      m!(ident, "break_", Syn::Ident, 6);
      m!(ident, "ret100", Syn::Ident, 6);
   }

   #[test]
   fn test_digits() {
      m!(digits, "");
      m!(digits, " 1");
      m!(digits, "0", Syn::Digits, 1);
      m!(digits, "1", Syn::Digits, 1);
      m!(digits, "0000000000.", Syn::Digits, 10);
      m!(digits, "0123456789.", Syn::Digits, 10);
      m!(digits, "9876543210.", Syn::Digits, 10);
   }

   #[test]
   fn test_string() {
      m!(string, "-");
      m!(string, "-''");
      m!(string, "'");
      m!(string, "'a");
      m!(string, "'ЯaЯaЯ");
      m!(string, "'a\\'");
      m!(string, "'a\\ '");
      m!(string, "'aaa\\abbb'");
      m!(string, "'aaa\\\"bbb'");
      m!(string, "''", Syn::String, 2);
      m!(string, "'a'", Syn::String, 3);
      m!(string, "'Я'", Syn::String, 4, 3);
      m!(string, "'y̆'", Syn::String, 5, 4);
      m!(string, "'ЯaЯaЯ'", Syn::String, 10, 7);
      m!(string, "'''", Syn::String, 2);
      m!(string, "'aaa bbb'", Syn::String, 9);
      m!(string, "'aaa bbb' ", Syn::String, 9);
      m!(string, "'aaa bbb'ccc", Syn::String, 9);
      m!(string, "'aaa\nbbb\nccc'", Syn::String, 13);
      m!(string, "'aaa\nbbb\nccc'\n", Syn::String, 13);
      m!(string, "'aaa\nbbb\nccc'", Syn::String, 13);
      m!(string, "'aaa\r\nbbb\r\nccc'", Syn::String, 15);
      m!(string, "'aaa\r\nbbb\r\nccc'\r\n", Syn::String, 15);
      m!(string, "'aaa\r\nbbb\r\nccc'", Syn::String, 15);
      m!(string, "'aaa\\nbbb'", Syn::String, 10);
      m!(string, "'aaa\\rbbb'", Syn::String, 10);
      m!(string, "'aaa\\tbbb'", Syn::String, 10);
      m!(string, "'aaa\\\\bbb'", Syn::String, 10);
      m!(string, "'aaa\\\'bbb'", Syn::String, 10);
      m!(string, "'aaa\\0bbb'", Syn::String, 10);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_string_empty() {
      e!(string);
   }
}
