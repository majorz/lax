
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Syn {
   Space,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
   pub syn: Syn,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type MatchRes = Option<(Syn, usize)>;
type MatchFn = fn(input: &str) -> MatchRes;

const KEYWORD_MAP: [(&'static str, Syn); 13] = [
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

#[inline]
fn consume_start(input: &str, item: &'static str) -> Option<usize> {
   debug_assert!(!input.is_empty());

   let item_len = item.len();
   if input.len() >= item_len && &input[..item_len] == item {
      Some(item_len)
   } else {
      None
   }
}

macro_rules! exact {
   ($string:expr, $func:ident, $token_type:expr) => {
      #[inline]
      fn $func(input: &str) -> MatchRes {
         if let Some(item_len) = consume_start(input, $string) {
            Some(($token_type, item_len))
         } else {
            None
         }
      }
   }
}

exact!("\n", match_new_line_n, Syn::NewLine);
exact!("\r\n", match_new_line_rn, Syn::NewLine);
exact!("\r", match_new_line_r, Syn::NewLine);
exact!("**", match_power, Syn::Power);
exact!("==", match_equal, Syn::Equal);
exact!("!=", match_unequal, Syn::Unequal);
exact!("<=", match_less_equal, Syn::LessEqual);
exact!(">=", match_greater_equal, Syn::GreaterEqual);
exact!("+=", match_add_assign, Syn::AddAssign);
exact!("-=", match_subtract_assign, Syn::SubtractAssign);
exact!("*=", match_multiply_assign, Syn::MultiplyAssign);
exact!("/=", match_divide_assign, Syn::DivideAssign);
exact!("..", match_range, Syn::Range);
exact!(".", match_dot, Syn::Dot);
exact!("=", match_assign, Syn::Assign);
exact!("+", match_add, Syn::Add);
exact!("-", match_subtract, Syn::Subtract);
exact!("*", match_multiply, Syn::Multiply);
exact!("/", match_divide, Syn::Divide);
exact!("|", match_bar, Syn::Bar);
exact!(":", match_colon, Syn::Colon);
exact!("(", match_paren_left, Syn::ParenLeft);
exact!(")", match_paren_right, Syn::ParenRight);
exact!("[", match_bracket_left, Syn::BracketLeft);
exact!("]", match_bracket_right, Syn::BracketRight);
exact!("<", match_angle_left, Syn::AngleLeft);
exact!(">", match_angle_right, Syn::AngleRight);
exact!("{", match_curly_left, Syn::CurlyLeft);
exact!("}", match_curly_right, Syn::CurlyRight);

fn match_space(input: &str) -> MatchRes {
   let mut pos = 0;

   for c in input.bytes() {
      if c == b' ' {
         pos += 1;
      } else {
         break;
      }
   }

   if pos == 0 {
      return None;
   } else {
      return Some((Syn::Space, pos));
   }
}

fn match_symbol(input: &str) -> MatchRes {
   debug_assert!(!input.is_empty());

   if input.len() == 1 {
      return None;
   }

   if input.as_bytes()[0] != b'^' {
      return None;
   }

   if let Some((Syn::Ident, pos)) = match_ident(&input[1..]) {
      Some((Syn::Symbol, pos + 1))
   } else {
      None
   }
}

fn match_ident(input: &str) -> MatchRes {
   debug_assert!(!input.is_empty());

   let c = input.as_bytes()[0];
   if !(
      (c >= b'a' && c <= b'z') ||
      (c >= b'A' && c <= b'Z') ||
      c == b'_') {
      return None;
   }

   let mut pos = 1;

   for c in input[1..].as_bytes() {
      if (*c >= b'a' && *c <= b'z') ||
         (*c >= b'A' && *c <= b'Z') ||
         (*c >= b'0' && *c <= b'9') ||
         *c == b'_' {
         pos += 1;
      } else {
         break;
      }
   }

   for &(keyword, keyword_ty) in KEYWORD_MAP.iter() {
      if keyword == &input[..pos] {
         return Some((keyword_ty, pos));
      }
   }

   Some((Syn::Ident, pos))
}

fn match_digits(input: &str) -> MatchRes {
   let mut pos = 0;

   for c in input.as_bytes() {
      if *c >= b'0' && *c <= b'9' {
         pos += 1;
      } else {
         break;
      }
   }

   if pos != 0 {
      Some((Syn::Digits, pos))
   } else {
      None
   }
}

fn match_accent(input: &str) -> MatchRes {
   debug_assert!(!input.is_empty());

   if input.as_bytes()[0] != b'`' {
      return None;
   }

   let mut indices = input[1..].char_indices();

   let bytes = loop {
      if let Some((i, ch)) = indices.next() {
         if ch == ' ' || ch == '\n' || ch == '\r' || ch == ')' {
            break i;
         }
      } else {
         break input.len() - 1;
      }
   };

   if bytes != 0 {
      Some((Syn::Accent, bytes + 1))
   } else {
      None
   }
}

fn match_string(input: &str) -> MatchRes {
   debug_assert!(!input.is_empty());

   if input.as_bytes()[0] != b'\'' {
      return None;
   }

   let mut indices = input[1..].char_indices();

   let pos = loop {
      if let Some((i, ch)) = indices.next() {
         match ch {
            '\\' => {
               if let Some((_, ch)) = indices.next() {
                  match ch {
                     'n' | 'r' | 't' | '\\' | '\'' | '0' => {},
                     _ => return None
                  }
               } else {
                  return None;
               }
            },
            '\'' => {
               break i;
            },
            _ => {}
         }
      } else {
         return None;
      }
   };

   Some((Syn::String, pos + 2))
}

const MATCH_FNS: [MatchFn; 35] = [
   match_space,
   match_new_line_n,
   match_new_line_rn,
   match_new_line_r,
   match_power,
   match_equal,
   match_unequal,
   match_less_equal,
   match_greater_equal,
   match_add_assign,
   match_subtract_assign,
   match_multiply_assign,
   match_divide_assign,
   match_range,
   match_assign,
   match_add,
   match_subtract,
   match_multiply,
   match_divide,
   match_bar,
   match_colon,
   match_paren_left,
   match_paren_right,
   match_bracket_left,
   match_bracket_right,
   match_angle_left,
   match_angle_right,
   match_curly_left,
   match_curly_right,
   match_ident,
   match_digits,
   match_dot,
   match_symbol,
   match_accent,
   match_string,
];

pub fn tokenize(input: &str) -> Vec<Token> {
   let mut tokens = vec![];

   let mut pos = 0;
   let mut line = 1;
   let mut col = 1;

   while pos < input.len() {
      if let Some((syn, span)) = match_token(&input[pos..]) {
         tokens.push(
            Token {
               syn,
               span,
               pos,
               line,
               col,
            }
         );

         if syn == Syn::NewLine {
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

fn match_token(input: &str) -> MatchRes {
   for matcher in MATCH_FNS.iter() {
      if let Some((syn, span)) = matcher(input) {
         return Some((syn, span));
      }
   }

   None
}

#[cfg(test)]
mod tests {
   use super::*;

   macro_rules! m {
      ($matcher:ident, $input:expr) => (
         assert_eq!($matcher($input), None);
      );

      ($matcher:ident, $input:expr, $syn:expr, $span:expr) => (
         assert_eq!($matcher($input), Some(($syn, $span)));
      );
   }

   #[test]
   fn consume_start_some() {
      assert_eq!(consume_start("brea", "break"), None);
      assert_eq!(consume_start("bbreak", "break"), None);
      assert_eq!(consume_start("break", "break"), Some(5));
      assert_eq!(consume_start("breakb", "break"), Some(5));
      assert_eq!(consume_start("breakЯ", "break"), Some(5));
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn consume_start_empty() {
      consume_start("", "break");
   }

   #[test]
   fn exact() {
      m!(match_power, "*");
      m!(match_power, "-**");
      m!(match_power, "**", Syn::Power, 2);
      m!(match_power, "****", Syn::Power, 2);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn exact_empty() {
      match_power("");
   }

   #[test]
   fn space() {
      m!(match_space, "");
      m!(match_space, "-");
      m!(match_space, "- ");
      m!(match_space, " ", Syn::Space, 1);
      m!(match_space, " -", Syn::Space, 1);
      m!(match_space, "   ", Syn::Space, 3);
      m!(match_space, "   -", Syn::Space, 3);
   }

   #[test]
   fn symbol() {
      m!(match_symbol, "-");
      m!(match_symbol, "-^name");
      m!(match_symbol, "^012abc");
      m!(match_symbol, "^");
      m!(match_symbol, "^-");
      m!(match_symbol, "^Я");
      m!(match_symbol, "^for");
      m!(match_symbol, "^_", Syn::Symbol, 2);
      m!(match_symbol, "^__", Syn::Symbol, 3);
      m!(match_symbol, "^_.", Syn::Symbol, 2);
      m!(match_symbol, "^_name", Syn::Symbol, 6);
      m!(match_symbol, "^name", Syn::Symbol, 5);
      m!(match_symbol, "^_NAME.", Syn::Symbol, 6);
      m!(match_symbol, "^NAME.", Syn::Symbol, 5);
      m!(match_symbol, "^a100", Syn::Symbol, 5);
      m!(match_symbol, "^a100.", Syn::Symbol, 5);
      m!(match_symbol, "^a_a_a.", Syn::Symbol, 6);
      m!(match_symbol, "^aЯ", Syn::Symbol, 2);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn symbol_empty() {
      match_symbol("");
   }

   #[test]
   fn ident() {
      m!(match_ident, "-");
      m!(match_ident, "-name");
      m!(match_ident, "012abc");
      m!(match_ident, "_", Syn::Ident, 1);
      m!(match_ident, "__", Syn::Ident, 2);
      m!(match_ident, "_.", Syn::Ident, 1);
      m!(match_ident, "_name", Syn::Ident, 5);
      m!(match_ident, "name", Syn::Ident, 4);
      m!(match_ident, "_NAME.", Syn::Ident, 5);
      m!(match_ident, "NAME.", Syn::Ident, 4);
      m!(match_ident, "a100", Syn::Ident, 4);
      m!(match_ident, "a100.", Syn::Ident, 4);
      m!(match_ident, "a_a_a.", Syn::Ident, 5);
      m!(match_ident, "aЯ", Syn::Ident, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn ident_empty() {
      match_ident("");
   }

   #[test]
   fn keyword() {
      m!(match_ident, "fn", Syn::Fn, 2);
      m!(match_ident, "loop", Syn::Loop, 4);
      m!(match_ident, "match", Syn::Match, 5);
      m!(match_ident, "if", Syn::If, 2);
      m!(match_ident, "ef", Syn::Ef, 2);
      m!(match_ident, "el", Syn::El, 2);
      m!(match_ident, "break", Syn::Break, 5);
      m!(match_ident, "ret", Syn::Ret, 3);
      m!(match_ident, "for", Syn::For, 3);
      m!(match_ident, "in", Syn::In, 2);
      m!(match_ident, "and", Syn::And, 3);
      m!(match_ident, "or", Syn::Or, 2);
      m!(match_ident, "not", Syn::Not, 3);
      m!(match_ident, "for", Syn::For, 3);
      m!(match_ident, "break_", Syn::Ident, 6);
      m!(match_ident, "ret100", Syn::Ident, 6);
   }

   #[test]
   fn digits() {
      m!(match_digits, "");
      m!(match_digits, " 1");
      m!(match_digits, "0", Syn::Digits, 1);
      m!(match_digits, "1", Syn::Digits, 1);
      m!(match_digits, "0000000000.", Syn::Digits, 10);
      m!(match_digits, "0123456789.", Syn::Digits, 10);
      m!(match_digits, "9876543210.", Syn::Digits, 10);
   }

   #[test]
   fn accent() {
      m!(match_accent, "-");
      m!(match_accent, "`");
      m!(match_accent, "`a", Syn::Accent, 2);
      m!(match_accent, "`Я", Syn::Accent, 3);
      m!(match_accent, "`y̆", Syn::Accent, 4);
      m!(match_accent, "`ЯaЯaЯ ", Syn::Accent, 9);
      m!(match_accent, "````", Syn::Accent, 4);
      m!(match_accent, "````\n", Syn::Accent, 4);
      m!(match_accent, "````\r\n", Syn::Accent, 4);
      m!(match_accent, "`abc) ", Syn::Accent, 4);
      m!(match_accent, "`abc\n ", Syn::Accent, 4);
      m!(match_accent, "`abc\r\n ", Syn::Accent, 4);
      m!(match_accent, "`abc  ", Syn::Accent, 4);
      m!(match_accent, "`abc\\) ", Syn::Accent, 5);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn accent_empty() {
      match_accent("");
   }

   #[test]
   fn string() {
      m!(match_string, "-");
      m!(match_string, "-''");
      m!(match_string, "'");
      m!(match_string, "'a");
      m!(match_string, "'ЯaЯaЯ");
      m!(match_string, "'a\\'");
      m!(match_string, "'a\\ '");
      m!(match_string, "'aaa\\abbb'");
      m!(match_string, "'aaa\\\"bbb'");
      m!(match_string, "''", Syn::String, 2);
      m!(match_string, "'a'", Syn::String, 3);
      m!(match_string, "'Я'", Syn::String, 4);
      m!(match_string, "'y̆'", Syn::String, 5);
      m!(match_string, "'ЯaЯaЯ'", Syn::String, 10);
      m!(match_string, "'''", Syn::String, 2);
      m!(match_string, "'aaa bbb'", Syn::String, 9);
      m!(match_string, "'aaa bbb' ", Syn::String, 9);
      m!(match_string, "'aaa bbb'ccc", Syn::String, 9);
      m!(match_string, "'aaa\nbbb\nccc'", Syn::String, 13);
      m!(match_string, "'aaa\nbbb\nccc'\n", Syn::String, 13);
      m!(match_string, "'aaa\nbbb\nccc'", Syn::String, 13);
      m!(match_string, "'aaa\r\nbbb\r\nccc'", Syn::String, 15);
      m!(match_string, "'aaa\r\nbbb\r\nccc'\r\n", Syn::String, 15);
      m!(match_string, "'aaa\r\nbbb\r\nccc'", Syn::String, 15);
      m!(match_string, "'aaa\\nbbb'", Syn::String, 10);
      m!(match_string, "'aaa\\rbbb'", Syn::String, 10);
      m!(match_string, "'aaa\\tbbb'", Syn::String, 10);
      m!(match_string, "'aaa\\\\bbb'", Syn::String, 10);
      m!(match_string, "'aaa\\\'bbb'", Syn::String, 10);
      m!(match_string, "'aaa\\0bbb'", Syn::String, 10);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn string_empty() {
      match_string("");
   }
}
