const INDENT: usize = 3;

#[derive(Debug, Clone, PartialEq)]
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

const KEYWORDS: [(&[char], Tok); 13] = [
   (&['f', 'n'], Tok::Fn),
   (&['l', 'o', 'o', 'p'], Tok::Loop),
   (&['m', 'a', 't', 'c', 'h'], Tok::Match),
   (&['i', 'f'], Tok::If),
   (&['e', 'f'], Tok::Ef),
   (&['e', 'l'], Tok::El),
   (&['b', 'r', 'e', 'a', 'k'], Tok::Break),
   (&['r', 'e', 't'], Tok::Ret),
   (&['f', 'o', 'r'], Tok::For),
   (&['i', 'n'], Tok::In),
   (&['a', 'n', 'd'], Tok::And),
   (&['o', 'r'], Tok::Or),
   (&['n', 'o', 't'], Tok::Not),
];

pub struct Token {
   pub tok: Tok,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type TokMatch = Option<(Tok, usize)>;

pub struct StrPeeker<'s> {
   input: &'s [char],
   peek: &'s [char],
}

impl<'s> StrPeeker<'s> {
   fn new(input: &'s [char]) -> Self {
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
      !self.peek.is_empty()
   }

   fn has_at_least(&mut self, span: usize) -> bool {
      self.peek.len() >= span
   }

   fn exact(&mut self, front: &[char]) -> Option<()> {
      let span = front.len();
      if self.peek.len() >= span && &self.peek[..span] == front {
         self.peek = &self.peek[span..];
         Some(())
      } else {
         self.peek = self.input;
         None
      }
   }

   fn require(&mut self, f: fn(char) -> bool) -> Option<()> {
      if self.peek.is_empty() {
         self.peek = self.input;
      }

      let ch = self.peek[0];
      if f(ch) {
         self.peek = &self.peek[1..];
         Some(())
      } else {
         self.peek = self.input;
         None
      }
   }

   fn next(&mut self) -> Option<char> {
      if let Some(ch) = self.peek.first() {
         self.peek = &self.peek[1..];
         Some(*ch)
      } else {
         self.peek = self.input;
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
         self.peek = self.input;
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

   fn reveal<'p>(&'p self) -> &'s [char] {
      &self.input[..self.input.len() - self.peek.len()]
   }
}

fn space(peeker: &mut StrPeeker) -> Option<usize> {
   peeker.multiple(|c| c == ' ')?;

   Some(peeker.commit())
}

fn string(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(peeker.has_more());

   peeker.require(|c| c == '\'')?;

   loop {
      match peeker.next()? {
         '\\' => {
            peeker.require(|c| {
               c == 'n' || c == '\'' || c == '\\' || c == 'r' || c == 't' || c == '0'
            })?;
         }
         '\'' => {
            break;
         }
         _ => {}
      }
   }

   Some((Tok::String, peeker.commit()))
}

fn symbol(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(peeker.has_more());

   if !peeker.has_at_least(2) {
      return None;
   }

   peeker.require(|c| c == '^')?;

   advance_identifier(peeker)?;

   if Tok::Identifier == tok_from_identifier(&peeker.reveal()[1..]) {
      Some((Tok::Symbol, peeker.commit()))
   } else {
      None
   }
}

fn identifier(peeker: &mut StrPeeker) -> TokMatch {
   debug_assert!(peeker.has_more());

   advance_identifier(peeker)?;

   let tok = tok_from_identifier(peeker.reveal());
   Some((tok, peeker.commit()))
}

fn advance_identifier(peeker: &mut StrPeeker) -> Option<()> {
   peeker.require(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_')?;

   peeker.any(|c| {
      (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
   });

   Some(())
}

fn tok_from_identifier(identifier: &[char]) -> Tok {
   for &(keyword, ref tok) in &KEYWORDS {
      if keyword == identifier {
         return (*tok).clone();
      }
   }

   Tok::Identifier
}

fn digits(peeker: &mut StrPeeker) -> TokMatch {
   peeker.multiple(|c| c >= '0' && c <= '9')?;

   Some((Tok::Digits, peeker.commit()))
}

macro_rules! exact {
   ($string:expr, $func:ident, $token_type:expr) => {
      fn $func(peeker: &mut StrPeeker) -> TokMatch {
         debug_assert!(peeker.has_more());
         peeker.exact($string)?;
         Some(($token_type, peeker.commit()))
      }
   }
}

exact!(&['\n'], new_line_n, Tok::NewLine);
exact!(&['\r', '\n'], new_line_rn, Tok::NewLine);
exact!(&['\r'], new_line_r, Tok::NewLine);
exact!(&['*', '*'], power, Tok::Power);
exact!(&['=', '='], equal, Tok::Equal);
exact!(&['!', '='], unequal, Tok::Unequal);
exact!(&['<', '='], less_equal, Tok::LessEqual);
exact!(&['>', '='], greater_equal, Tok::GreaterEqual);
exact!(&['+', '='], add_assign, Tok::AddAssign);
exact!(&['-', '='], subtract_assign, Tok::SubtractAssign);
exact!(&['*', '='], multiply_assign, Tok::MultiplyAssign);
exact!(&['/', '='], divide_assign, Tok::DivideAssign);
exact!(&['.', '.'], range, Tok::Range);
exact!(&['.'], dot, Tok::Dot);
exact!(&['='], assign, Tok::Assign);
exact!(&['+'], add, Tok::Add);
exact!(&['-'], subtract, Tok::Subtract);
exact!(&['*'], multiply, Tok::Multiply);
exact!(&['/'], divide, Tok::Divide);
exact!(&['|'], bar, Tok::Bar);
exact!(&[':'], colon, Tok::Colon);
exact!(&['('], paren_left, Tok::ParenLeft);
exact!(&[')'], paren_right, Tok::ParenRight);
exact!(&['['], bracket_left, Tok::BracketLeft);
exact!(&[']'], bracket_right, Tok::BracketRight);
exact!(&['<'], angle_left, Tok::AngleLeft);
exact!(&['>'], angle_right, Tok::AngleRight);
exact!(&['{'], curly_left, Tok::CurlyLeft);
exact!(&['}'], curly_right, Tok::CurlyRight);

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
   let chars: Vec<_> = input.chars().collect();

   let mut tokens = vec![];

   let mut after_new_line = true;

   let mut pos = 0;
   let mut line = 1;
   let mut col = 1;

   let mut peeker = StrPeeker::new(&chars);

   while pos < chars.len() {
      if let Some(span) = space(&mut peeker) {
         if after_new_line {
            assert!(span % INDENT == 0);
            let indents = span / INDENT;
            for _ in 0..indents {
               pos += INDENT;
               col += INDENT;
               tokens.push(Token {
                  tok: Tok::Indent,
                  span: INDENT,
                  pos,
                  line,
                  col,
               });
            }
         } else {
            pos += span;
            col += span;
         }
      }

      if let Some((tok, span)) = match_tok(&mut peeker) {
         after_new_line = tok == Tok::NewLine;

         tokens.push(Token {
            tok,
            span,
            pos,
            line,
            col,
         });

         if after_new_line {
            line += 1;
            col = 1;
         }

         pos += span;
         col += span;
      } else {
         panic!("Unrecognized token at line: {}, col: {}", line, col);
      }
   }

   tokens
}

fn match_tok(peeker: &mut StrPeeker) -> TokMatch {
   for matcher in MATCHERS.iter() {
      if let Some((tok, span)) = matcher(peeker) {
         return Some((tok, span));
      }
   }

   None
}

#[cfg(test)]
mod tests {
   use super::*;

   fn as_chars(input: &str) -> Vec<char> {
      input.chars().collect()
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

   macro_rules! space {
      ($input:expr) => (
         let chars = as_chars($input);
         let mut peeker = StrPeeker::new(&chars);
         assert_eq!(space(&mut peeker), None);
      );

      ($input:expr, $span:expr) => (
         let chars = as_chars($input);
         let mut peeker = StrPeeker::new(&chars);
         assert_eq!(space(&mut peeker), Some($span));
      );
   }

   #[cfg(debug_assertions)]
   macro_rules! e {
      ($matcher:ident) => (
         $matcher(&mut StrPeeker::new(&[]));
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
      m!(string, "'Я'", Tok::String, 3);
      m!(string, "'y̆'", Tok::String, 4);
      m!(string, "'ЯaЯaЯ'", Tok::String, 7);
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
