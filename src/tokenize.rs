#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
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
   pub ty: TokenType,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

type MatchRes = Option<(TokenType, usize)>;
type MatchFn = fn(input: &str) -> MatchRes;

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

exact!("\n", match_new_line, TokenType::NewLine);
exact!("**", match_power, TokenType::Power);
exact!("==", match_equal, TokenType::Equal);
exact!("!=", match_unequal, TokenType::Unequal);
exact!("<=", match_less_equal, TokenType::LessEqual);
exact!(">=", match_greater_equal, TokenType::GreaterEqual);
exact!("+=", match_add_assign, TokenType::AddAssign);
exact!("-=", match_subtract_assign, TokenType::SubtractAssign);
exact!("*=", match_multiply_assign, TokenType::MultiplyAssign);
exact!("/=", match_divide_assign, TokenType::DivideAssign);
exact!("..", match_range, TokenType::Range);
exact!(".", match_dot, TokenType::Dot);
exact!("=", match_assign, TokenType::Assign);
exact!("+", match_add, TokenType::Add);
exact!("-", match_subtract, TokenType::Subtract);
exact!("*", match_multiply, TokenType::Multiply);
exact!("/", match_divide, TokenType::Divide);
exact!("|", match_bar, TokenType::Bar);
exact!(":", match_colon, TokenType::Colon);
exact!("(", match_paren_left, TokenType::ParenLeft);
exact!(")", match_paren_right, TokenType::ParenRight);
exact!("[", match_bracket_left, TokenType::BracketLeft);
exact!("]", match_bracket_right, TokenType::BracketRight);
exact!("<", match_angle_left, TokenType::AngleLeft);
exact!(">", match_angle_right, TokenType::AngleRight);
exact!("{", match_curly_left, TokenType::CurlyLeft);
exact!("}", match_curly_right, TokenType::CurlyRight);

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
      return Some((TokenType::Space, pos));
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

   if let Some((_, pos)) = match_ident(&input[1..]) {
      Some((TokenType::Symbol, pos + 1))
   } else {
      None
   }
}

const KEYWORD_MAP: [(&'static str, TokenType); 13] = [
   ("fn",     TokenType::Fn),
   ("loop",   TokenType::Loop),
   ("match",  TokenType::Match),
   ("if",     TokenType::If),
   ("ef",     TokenType::Ef),
   ("el",     TokenType::El),
   ("break",  TokenType::Break),
   ("ret",    TokenType::Ret),
   ("for",    TokenType::For),
   ("in",     TokenType::In),
   ("and",    TokenType::And),
   ("or",     TokenType::Or),
   ("not",    TokenType::Not),
];

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

   Some((TokenType::Ident, pos))
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
      Some((TokenType::Digits, pos))
   } else {
      None
   }
}

fn match_accent(input: &str) -> MatchRes {
   if input.as_bytes()[0] != b'`' {
      return None;
   }

   let mut indices = input[1..].char_indices();
   let mut pos = 0;

   loop {
      if let Some((i, ch)) = indices.next() {
         pos = i;
         if ch == '\\' {
            if let Some((j, ch)) = indices.next() {
               pos = j;
               if ch == ' ' || ch == '\n' {
                  break;
               }
            } else {
               break;
            }
         } else if ch == ' ' || ch == '\n' || ch == ')' {
            break;
         }
      } else {
         break;
      }
   }

   if pos != 0 {
      Some((TokenType::Accent, pos + 1))
   } else {
      None
   }
}

fn match_string(input: &str) -> MatchRes {
   if input.as_bytes()[0] != b'\'' {
      return None;
   }

   let mut indices = input[1..].char_indices();
   let pos;

   loop {
      if let Some((i, ch)) = indices.next() {
         if ch == '\\' {
            if let Some((_, ch)) = indices.next() {
               if ch == '\n' {
                  return None;
               }
            } else {
               return None;
            }
         } else if ch == '\n' {
            return None;
         } else if ch == '\'' {
            pos = i;
            break;
         }
      } else {
         return None;
      }
   }

   Some((TokenType::Accent, pos + 2))
}

const MATCH_FNS: [MatchFn; 33] = [
   match_space,
   match_new_line,
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
      if let Some((ty, span)) = match_token(&input[pos..]) {
         tokens.push(
            Token {
               ty,
               span,
               pos,
               line,
               col,
            }
         );

         if ty == TokenType::NewLine {
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
      if let Some((ty, span)) = matcher(input) {
         return Some((ty, span));
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

      ($matcher:ident, $input:expr, $ty:expr, $span:expr) => (
         assert_eq!($matcher($input), Some(($ty, $span)));
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
      m!(match_power, "**", TokenType::Power, 2);
      m!(match_power, "****", TokenType::Power, 2);
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
      m!(match_space, " ", TokenType::Space, 1);
      m!(match_space, " -", TokenType::Space, 1);
      m!(match_space, "   ", TokenType::Space, 3);
      m!(match_space, "   -", TokenType::Space, 3);
   }

   #[test]
   fn ident() {
      m!(match_ident, "-");
      m!(match_ident, "-name");
      m!(match_ident, "012abc");
      m!(match_ident, "_", TokenType::Ident, 1);
      m!(match_ident, "__", TokenType::Ident, 2);
      m!(match_ident, "_.", TokenType::Ident, 1);
      m!(match_ident, "_name", TokenType::Ident, 5);
      m!(match_ident, "name", TokenType::Ident, 4);
      m!(match_ident, "_NAME.", TokenType::Ident, 5);
      m!(match_ident, "NAME.", TokenType::Ident, 4);
      m!(match_ident, "a100", TokenType::Ident, 4);
      m!(match_ident, "a100.", TokenType::Ident, 4);
      m!(match_ident, "a_a_a.", TokenType::Ident, 5);
      m!(match_ident, "aЯ", TokenType::Ident, 1);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn ident_empty() {
      match_ident("");
   }

   #[test]
   fn keyword() {
      m!(match_ident, "fn", TokenType::Fn, 2);
      m!(match_ident, "loop", TokenType::Loop, 4);
      m!(match_ident, "match", TokenType::Match, 5);
      m!(match_ident, "if", TokenType::If, 2);
      m!(match_ident, "ef", TokenType::Ef, 2);
      m!(match_ident, "el", TokenType::El, 2);
      m!(match_ident, "break", TokenType::Break, 5);
      m!(match_ident, "ret", TokenType::Ret, 3);
      m!(match_ident, "for", TokenType::For, 3);
      m!(match_ident, "in", TokenType::In, 2);
      m!(match_ident, "and", TokenType::And, 3);
      m!(match_ident, "or", TokenType::Or, 2);
      m!(match_ident, "not", TokenType::Not, 3);
      m!(match_ident, "for", TokenType::For, 3);
      m!(match_ident, "break_", TokenType::Ident, 6);
      m!(match_ident, "ret100", TokenType::Ident, 6);
   }
}
