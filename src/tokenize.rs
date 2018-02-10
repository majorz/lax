
const INDENT: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tok {
   Indent,
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
   Identifier,
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

const KEYWORDS: [(&'static str, Tok); 13] = [
   ("fn",     Tok::Fn),
   ("loop",   Tok::Loop),
   ("match",  Tok::Match),
   ("if",     Tok::If),
   ("ef",     Tok::Ef),
   ("el",     Tok::El),
   ("break",  Tok::Break),
   ("ret",    Tok::Ret),
   ("for",    Tok::For),
   ("in",     Tok::In),
   ("and",    Tok::And),
   ("or",     Tok::Or),
   ("not",    Tok::Not),
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
   pub tok: Tok,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type TokMatch = Option<(Tok, usize, usize)>;

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

fn string(peeker: &mut StrPeeker) -> TokMatch {
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
   Some((Tok::String, span, chars + 1))
}

fn symbol(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(peeker.has_more());

   if !peeker.has_at_least(2) {
      return None;
   }

   peeker.require(|b| b == b'^')?;

   advance_identifier(peeker)?;

   if Tok::Identifier == tok_from_identifier(&peeker.reveal()[1..]) {
      let span = peeker.commit();
      Some((Tok::Symbol, span, span))
   } else {
      None
   }
}

fn identifier(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(peeker.has_more());

   advance_identifier(peeker)?;

   let tok = tok_from_identifier(peeker.reveal());
   let span = peeker.commit();
   Some((tok, span, span))
}

fn advance_identifier(peeker: &mut StrPeeker) -> Option<()> {
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

fn tok_from_identifier(identifier: &str) -> Tok {
   for &(keyword, tok) in KEYWORDS.iter() {
      if keyword == identifier {
         return tok;
      }
   }

   Tok::Identifier
}

fn digits(peeker: &mut StrPeeker) -> TokMatch {
   peeker.multiple(|b| b >= b'0' && b <= b'9')?;

   let span = peeker.commit();
   Some((Tok::Digits, span, span))
}

macro_rules! exact {
   ($string:expr, $func:ident, $token_type:expr) => {
      fn $func(peeker: &mut StrPeeker) -> TokMatch {
         debug_assert!(peeker.has_more());
         peeker.exact($string)?;
         let span = peeker.commit();
         Some(($token_type, span, span))
      }
   }
}

exact!("\n", new_line_n, Tok::NewLine);
exact!("\r\n", new_line_rn, Tok::NewLine);
exact!("\r", new_line_r, Tok::NewLine);
exact!("**", power, Tok::Power);
exact!("==", equal, Tok::Equal);
exact!("!=", unequal, Tok::Unequal);
exact!("<=", less_equal, Tok::LessEqual);
exact!(">=", greater_equal, Tok::GreaterEqual);
exact!("+=", add_assign, Tok::AddAssign);
exact!("-=", subtract_assign, Tok::SubtractAssign);
exact!("*=", multiply_assign, Tok::MultiplyAssign);
exact!("/=", divide_assign, Tok::DivideAssign);
exact!("..", range, Tok::Range);
exact!(".", dot, Tok::Dot);
exact!("=", assign, Tok::Assign);
exact!("+", add, Tok::Add);
exact!("-", subtract, Tok::Subtract);
exact!("*", multiply, Tok::Multiply);
exact!("/", divide, Tok::Divide);
exact!("|", bar, Tok::Bar);
exact!(":", colon, Tok::Colon);
exact!("(", paren_left, Tok::ParenLeft);
exact!(")", paren_right, Tok::ParenRight);
exact!("[", bracket_left, Tok::BracketLeft);
exact!("]", bracket_right, Tok::BracketRight);
exact!("<", angle_left, Tok::AngleLeft);
exact!(">", angle_right, Tok::AngleRight);
exact!("{", curly_left, Tok::CurlyLeft);
exact!("}", curly_right, Tok::CurlyRight);

const MATCHERS: [fn(peeker: &mut StrPeeker) -> TokMatch; 33] = [
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
   identifier,
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
                     tok: Tok::Indent,
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

      if let Some((tok, span, chars)) = match_tok(&mut peeker) {
         tokens.push(
            Token {
               tok,
               span,
               pos,
               line,
               col,
            }
         );

         after_new_line = tok == Tok::NewLine;
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

fn match_tok(peeker: &mut StrPeeker) -> TokMatch {
   for matcher in MATCHERS.iter() {
      if let Some((tok, span, chars)) = matcher(peeker) {
         return Some((tok, span, chars));
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

      ($matcher:ident, $input:expr, $tok:expr, $span:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!($matcher(&mut peeker), Some(($tok, $span, $span)));
      );

      ($matcher:ident, $input:expr, $tok:expr, $span:expr, $chars:expr) => (
         let mut peeker = StrPeeker::new($input);
         assert_eq!($matcher(&mut peeker), Some(($tok, $span, $chars)));
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
      m!(multiply, "*", Tok::Multiply, 1);
      m!(power, "**", Tok::Power, 2);
      m!(power, "****", Tok::Power, 2);
      m!(new_line_n, "\n\n", Tok::NewLine, 1);
      m!(new_line_rn, "\r\n", Tok::NewLine, 2);
      m!(new_line_r, "\r\r", Tok::NewLine, 1);
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
      m!(symbol, "^_", Tok::Symbol, 2);
      m!(symbol, "^__", Tok::Symbol, 3);
      m!(symbol, "^_.", Tok::Symbol, 2);
      m!(symbol, "^_name", Tok::Symbol, 6);
      m!(symbol, "^name", Tok::Symbol, 5);
      m!(symbol, "^_NAME.", Tok::Symbol, 6);
      m!(symbol, "^NAME.", Tok::Symbol, 5);
      m!(symbol, "^a100", Tok::Symbol, 5);
      m!(symbol, "^a100.", Tok::Symbol, 5);
      m!(symbol, "^a_a_a.", Tok::Symbol, 6);
      m!(symbol, "^aЯ", Tok::Symbol, 2);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_symbol_empty() {
      e!(symbol);
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
   fn test_keyword() {
      m!(identifier, "fn", Tok::Fn, 2);
      m!(identifier, "loop", Tok::Loop, 4);
      m!(identifier, "match", Tok::Match, 5);
      m!(identifier, "if", Tok::If, 2);
      m!(identifier, "ef", Tok::Ef, 2);
      m!(identifier, "el", Tok::El, 2);
      m!(identifier, "break", Tok::Break, 5);
      m!(identifier, "ret", Tok::Ret, 3);
      m!(identifier, "for", Tok::For, 3);
      m!(identifier, "in", Tok::In, 2);
      m!(identifier, "and", Tok::And, 3);
      m!(identifier, "or", Tok::Or, 2);
      m!(identifier, "not", Tok::Not, 3);
      m!(identifier, "for", Tok::For, 3);
      m!(identifier, "break_", Tok::Identifier, 6);
      m!(identifier, "ret100", Tok::Identifier, 6);
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
      m!(string, "-");
      m!(string, "-''");
      m!(string, "'");
      m!(string, "'a");
      m!(string, "'ЯaЯaЯ");
      m!(string, "'a\\'");
      m!(string, "'a\\ '");
      m!(string, "'aaa\\abbb'");
      m!(string, "'aaa\\\"bbb'");
      m!(string, "''", Tok::String, 2);
      m!(string, "'a'", Tok::String, 3);
      m!(string, "'Я'", Tok::String, 4, 3);
      m!(string, "'y̆'", Tok::String, 5, 4);
      m!(string, "'ЯaЯaЯ'", Tok::String, 10, 7);
      m!(string, "'''", Tok::String, 2);
      m!(string, "'aaa bbb'", Tok::String, 9);
      m!(string, "'aaa bbb' ", Tok::String, 9);
      m!(string, "'aaa bbb'ccc", Tok::String, 9);
      m!(string, "'aaa\nbbb\nccc'", Tok::String, 13);
      m!(string, "'aaa\nbbb\nccc'\n", Tok::String, 13);
      m!(string, "'aaa\nbbb\nccc'", Tok::String, 13);
      m!(string, "'aaa\r\nbbb\r\nccc'", Tok::String, 15);
      m!(string, "'aaa\r\nbbb\r\nccc'\r\n", Tok::String, 15);
      m!(string, "'aaa\r\nbbb\r\nccc'", Tok::String, 15);
      m!(string, "'aaa\\nbbb'", Tok::String, 10);
      m!(string, "'aaa\\rbbb'", Tok::String, 10);
      m!(string, "'aaa\\tbbb'", Tok::String, 10);
      m!(string, "'aaa\\\\bbb'", Tok::String, 10);
      m!(string, "'aaa\\\'bbb'", Tok::String, 10);
      m!(string, "'aaa\\0bbb'", Tok::String, 10);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_string_empty() {
      e!(string);
   }
}
