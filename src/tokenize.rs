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

   #[test]
   fn consume_start_() {
      assert_eq!(consume_start("", "break"), None);
      assert_eq!(consume_start("brea", "break"), None);
      assert_eq!(consume_start("bbreak", "break"), None);
      assert_eq!(consume_start("break", "break"), Some(5));
      assert_eq!(consume_start("breakb", "break"), Some(5));
      assert_eq!(consume_start("breakЯ", "break"), Some(5));
   }

   #[test]
   fn exact() {
      assert_eq!(match_power(""), None);
      assert_eq!(match_power("*"), None);
      assert_eq!(match_power("-**"), None);
      assert_eq!(match_power("**"), Some((TokenType::Power, 2)));
      assert_eq!(match_power("****"), Some((TokenType::Power, 2)));
   }

   #[test]
   fn space() {
      assert_eq!(match_space(""), None);
      assert_eq!(match_space("-"), None);
      assert_eq!(match_space("- "), None);
      assert_eq!(match_space(" "), Some((TokenType::Space, 1)));
      assert_eq!(match_space(" -"), Some((TokenType::Space, 1)));
      assert_eq!(match_space("   "), Some((TokenType::Space, 3)));
      assert_eq!(match_space("   -"), Some((TokenType::Space, 3)));
   }
}
