use advancer::Advancer;

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
   Indent,
   Space,
   LineEnd,
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
   Ef,
   El,
   If,
   In,
   Fn,
   Or,
   And,
   For,
   Not,
   Ret,
   Loop,
   True,
   Break,
   False,
   Match,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokMeta {
   pub span: usize,
   pub end: usize,
   pub line: usize,
   pub col: usize,
}

type TokMatch = Option<(Tok, usize)>;

type CharAdvancer<'a> = Advancer<'a, char>;

type FnMatcher = fn(&char) -> bool;

fn space_line_end(advancer: &mut CharAdvancer) -> TokMatch {
   let pos_start = advancer.pos();
   advancer.zero_or_more(' ');

   let pos_after_space = advancer.pos();
   advancer.zero_or_one('\r');

   let pos_after_r = advancer.pos();
   if pos_after_r != pos_after_space {
      advancer.zero_or_one('\n');
      Some((Tok::LineEnd, advancer.consume()))
   } else {
      advancer.zero_or_one('\n');

      if pos_after_r != advancer.pos() {
         Some((Tok::LineEnd, advancer.consume()))
      } else if pos_after_space != pos_start {
         if advancer.cannot_peek() {
            Some((Tok::LineEnd, advancer.consume()))
         } else {
            Some((Tok::Space, advancer.consume()))
         }
      } else {
         None
      }
   }
}

fn identifier(advancer: &mut CharAdvancer) -> TokMatch {
   debug_assert!(!advancer.completed());

   advancer
      .one((|c| (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || *c == '_') as FnMatcher)?;

   advancer.zero_or_more(
      (|c| {
         (*c >= 'a' && *c <= 'z')
            || (*c >= 'A' && *c <= 'Z')
            || (*c >= '0' && *c <= '9')
            || *c == '_'
      }) as FnMatcher,
   );

   let tok = try_keyword(advancer);

   Some((tok, advancer.consume()))
}

fn try_keyword(advancer: &CharAdvancer) -> Tok {
   let w = advancer.current();
   let len = w.len();
   match len {
      2 => match unsafe { *w.get_unchecked(0) } {
         'e' => match unsafe { *w.get_unchecked(1) } {
            'f' => return Tok::Ef,
            'l' => return Tok::El,
            _ => {}
         },
         'i' => match unsafe { *w.get_unchecked(1) } {
            'f' => return Tok::If,
            'n' => return Tok::In,
            _ => {}
         },
         'f' => if unsafe { *w.get_unchecked(1) == 'n' } {
            return Tok::Fn;
         },
         'o' => if unsafe { *w.get_unchecked(1) == 'r' } {
            return Tok::Or;
         },
         _ => {}
      },
      3 => {
         if unsafe {
            *w.get_unchecked(0) == 'a' && *w.get_unchecked(1) == 'n' && *w.get_unchecked(2) == 'd'
         } {
            return Tok::And;
         }
         if unsafe {
            *w.get_unchecked(0) == 'f' && *w.get_unchecked(1) == 'o' && *w.get_unchecked(2) == 'r'
         } {
            return Tok::For;
         }
         if unsafe {
            *w.get_unchecked(0) == 'n' && *w.get_unchecked(1) == 'o' && *w.get_unchecked(2) == 't'
         } {
            return Tok::Not;
         }
         if unsafe {
            *w.get_unchecked(0) == 'r' && *w.get_unchecked(1) == 'e' && *w.get_unchecked(2) == 't'
         } {
            return Tok::Ret;
         }
      }
      4 => {
         if unsafe {
            *w.get_unchecked(0) == 'l'
               && *w.get_unchecked(1) == 'o'
               && *w.get_unchecked(2) == 'o'
               && *w.get_unchecked(3) == 'p'
         } {
            return Tok::Loop;
         }
         if unsafe {
            *w.get_unchecked(0) == 't'
               && *w.get_unchecked(1) == 'r'
               && *w.get_unchecked(2) == 'u'
               && *w.get_unchecked(3) == 'e'
         } {
            return Tok::True;
         }
      }
      5 => {
         if unsafe {
            *w.get_unchecked(0) == 'b'
               && *w.get_unchecked(1) == 'r'
               && *w.get_unchecked(2) == 'e'
               && *w.get_unchecked(3) == 'a'
               && *w.get_unchecked(4) == 'k'
         } {
            return Tok::Break;
         }
         if unsafe {
            *w.get_unchecked(0) == 'f'
               && *w.get_unchecked(1) == 'a'
               && *w.get_unchecked(2) == 'l'
               && *w.get_unchecked(3) == 's'
               && *w.get_unchecked(4) == 'e'
         } {
            return Tok::False;
         }
         if unsafe {
            *w.get_unchecked(0) == 'm'
               && *w.get_unchecked(1) == 'a'
               && *w.get_unchecked(2) == 't'
               && *w.get_unchecked(3) == 'c'
               && *w.get_unchecked(4) == 'h'
         } {
            return Tok::Match;
         }
      }
      _ => {}
   }

   Tok::Identifier
}

fn digits(advancer: &mut CharAdvancer) -> TokMatch {
   advancer.one_or_more((|c| *c >= '0' && *c <= '9') as FnMatcher)?;

   Some((Tok::Digits, advancer.consume()))
}

macro_rules! exact {
   ($c1:expr, $func:ident, $token_type:expr) => {
      fn $func(advancer: &mut CharAdvancer) -> TokMatch {
         debug_assert!(!advancer.completed());
         advancer.one($c1)?;
         Some(($token_type, advancer.consume()))
      }
   };

   ($c1:expr, $c2:expr, $func:ident, $token_type:expr) => {
      fn $func(advancer: &mut CharAdvancer) -> TokMatch {
         debug_assert!(!advancer.completed());
         advancer.one($c1)?;
         advancer.one($c2)?;
         Some(($token_type, advancer.consume()))
      }
   };
}

exact!('*', '*', double_asterisk, Tok::DoubleAsterisk);
exact!('=', '=', double_equals, Tok::DoubleEquals);
exact!('!', '=', exclamation_equals, Tok::ExclamationEquals);
exact!('<', '=', less_than_equals, Tok::LessThanEquals);
exact!('>', '=', greater_than_equals, Tok::GreaterThanEquals);
exact!('+', '=', plus_equals, Tok::PlusEquals);
exact!('-', '=', minus_equals, Tok::MinusEquals);
exact!('*', '=', asterisk_equals, Tok::AsteriskEquals);
exact!('/', '=', slash_equals, Tok::SlashEquals);
exact!('.', '.', double_full_stop, Tok::DoubleFullStop);
exact!('.', full_stop, Tok::FullStop);
exact!('=', equals, Tok::Equals);
exact!('+', plus, Tok::Plus);
exact!('-', minus, Tok::Minus);
exact!('*', asterisk, Tok::Asterisk);
exact!('/', slash, Tok::Slash);
exact!('|', vertical_bar, Tok::VerticalBar);
exact!(':', colon, Tok::Colon);
exact!('^', caret, Tok::Caret);
exact!('(', paren_left, Tok::ParenLeft);
exact!(')', paren_right, Tok::ParenRight);
exact!('[', square_bracket_left, Tok::SquareBracketLeft);
exact!(']', square_bracket_right, Tok::SquareBracketRight);
exact!('<', less_than, Tok::LessThan);
exact!('>', greater_than, Tok::GreaterThan);
exact!('{', curly_bracket_left, Tok::CurlyBracketLeft);
exact!('}', curly_backet_right, Tok::CurlyBracketRight);

const MATCHERS: &[fn(advancer: &mut CharAdvancer) -> TokMatch] = &[
   space_line_end,
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

fn run_matchers(advancer: &mut CharAdvancer) -> TokMatch {
   for matcher in MATCHERS {
      if let Some((tok, end)) = matcher(advancer) {
         return Some((tok, end));
      }
   }

   None
}

struct Tokenizer<'s> {
   toks: Vec<Tok>,
   toks_meta: Vec<TokMeta>,
   end: usize,
   line: usize,
   col: usize,
   advancer: CharAdvancer<'s>,
}

impl<'s> Tokenizer<'s> {
   fn new(chars: &'s [char]) -> Self {
      let toks = vec![];
      let toks_meta = vec![];

      let end = 0;
      let line = 1;
      let col = 1;

      let advancer = CharAdvancer::new(chars);

      Tokenizer {
         toks,
         toks_meta,
         end,
         line,
         col,
         advancer,
      }
   }

   fn destructure(self) -> (Vec<Tok>, Vec<TokMeta>) {
      let Self {
         toks, toks_meta, ..
      } = self;

      (toks, toks_meta)
   }

   fn push(&mut self, tok: Tok, end: usize) {
      self.toks.push(tok);

      let span = end - self.end;

      self.toks_meta.push(TokMeta {
         span,
         end,
         line: self.line,
         col: self.col,
      });

      self.col += span;
      self.end = end;
   }

   fn tokenize(mut self) -> Self {
      while !self.advancer.completed() {
         if self.match_string().is_none() {
            self.match_tok()
         }
      }

      let line_end = if let Some(tok) = self.toks.last() {
         tok == &Tok::LineEnd
      } else {
         false
      };

      if !line_end {
         let end = self.end;
         self.push(Tok::LineEnd, end);
      }

      self
   }

   fn match_tok(&mut self) {
      if let Some((tok, end)) = run_matchers(&mut self.advancer) {
         let after_new_line = tok == Tok::LineEnd;

         self.push(tok, end);

         if after_new_line {
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

   fn match_string(&mut self) -> Option<()> {
      self.advancer.one('\'')?;

      let start = self.advancer.pos();
      self.push(Tok::Apostrophe, start);

      loop {
         match *(self.advancer.one((|_| true) as FnMatcher)?) {
            '\\' => {
               self
                  .advancer
                  .one(&['n', '\'', '\\', 'r', 't', '0'] as &[char])?;
            }
            '\'' => {
               break;
            }
            '\n' | '\r' => {
               self.end = self.advancer.pos();
               self.col += self.end - start;

               panic!(
                  "New line in string at line: {}, col: {}",
                  self.line, self.col
               );
            }
            _ => {}
         }
      }

      let after = self.advancer.pos();

      if start != after - 1 {
         self.push(Tok::Text, after - 1);
      }

      self.push(Tok::Apostrophe, after);

      self.advancer.consume();

      Some(())
   }
}

pub fn tokenize(chars: &[char]) -> (Vec<Tok>, Vec<TokMeta>) {
   Tokenizer::new(chars).tokenize().destructure()
}

#[cfg(test)]
mod tests {
   use super::*;

   fn as_chars(source: &str) -> Vec<char> {
      source.chars().collect()
   }

   macro_rules! m {
      ($matcher:ident, $input:expr) => {
         let chars = as_chars($input);
         let mut advancer = CharAdvancer::new(&chars);
         assert_eq!($matcher(&mut advancer), None);
      };

      ($matcher:ident, $input:expr, $tok:expr, $end:expr) => {
         let chars = as_chars($input);
         let mut advancer = CharAdvancer::new(&chars);
         assert_eq!($matcher(&mut advancer), Some(($tok, $end)));
      };
   }

   #[cfg(debug_assertions)]
   macro_rules! e {
      ($matcher:ident) => {
         $matcher(&mut CharAdvancer::new(&[]));
      };
   }

   macro_rules! string {
      ($input:expr) => {
         let chars = as_chars($input);
         let mut tokenizer = Tokenizer::new(&chars);
         assert!(tokenizer.match_string().is_none());
      };

      ($input:expr, $span:expr) => {
         let chars = as_chars($input);
         let mut tokenizer = Tokenizer::new(&chars);
         assert!(tokenizer.match_string().is_some());
         let (toks, toks_meta) = tokenizer.destructure();
         if $span == 0 {
            assert_eq!(toks.len(), 2);
            assert_eq!(toks[0], Tok::Apostrophe);
            assert_eq!(toks[1], Tok::Apostrophe);
            assert_eq!(toks_meta[0].end, 1);
            assert_eq!(toks_meta[0].span, 1);
            assert_eq!(toks_meta[1].end, 2);
            assert_eq!(toks_meta[1].span, 1);
         } else {
            assert_eq!(toks.len(), 3);
            assert_eq!(toks[0], Tok::Apostrophe);
            assert_eq!(toks[1], Tok::Text);
            assert_eq!(toks[2], Tok::Apostrophe);
            assert_eq!(toks_meta[0].end, 1);
            assert_eq!(toks_meta[0].span, 1);
            assert_eq!(toks_meta[1].end, $span + 1);
            assert_eq!(toks_meta[1].span, $span);
            assert_eq!(toks_meta[2].end, $span + 2);
            assert_eq!(toks_meta[2].span, 1);
         }
      };
   }

   #[test]
   fn test_exact() {
      m!(double_asterisk, "*");
      m!(double_asterisk, "-**");
      m!(asterisk, "*", Tok::Asterisk, 1);
      m!(double_asterisk, "**", Tok::DoubleAsterisk, 2);
      m!(double_asterisk, "****", Tok::DoubleAsterisk, 2);
   }

   #[test]
   #[should_panic]
   #[cfg(debug_assertions)]
   fn test_exact_empty() {
      e!(double_asterisk);
      e!(asterisk);
   }

   #[test]
   fn test_space_line_end() {
      m!(space_line_end, "");
      m!(space_line_end, "  \n", Tok::LineEnd, 3);
      m!(space_line_end, "\n  ", Tok::LineEnd, 1);
      m!(space_line_end, "\n\n", Tok::LineEnd, 1);
      m!(space_line_end, "\r\n", Tok::LineEnd, 2);
      m!(space_line_end, "\r\r", Tok::LineEnd, 1);
      m!(space_line_end, "\r\n", Tok::LineEnd, 2);
      m!(space_line_end, "  \n    ", Tok::LineEnd, 3);
      m!(space_line_end, "  \r\n\n", Tok::LineEnd, 4);
      m!(space_line_end, "-");
      m!(space_line_end, "- ");
      m!(space_line_end, " ", Tok::LineEnd, 1);
      m!(space_line_end, " -", Tok::Space, 1);
      m!(space_line_end, "   ", Tok::LineEnd, 3);
      m!(space_line_end, "   -", Tok::Space, 3);
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
      m!(identifier, "efx", Tok::Identifier, 3);
      m!(identifier, "ef ", Tok::Ef, 2);
      m!(identifier, "ef/", Tok::Ef, 2);
      m!(identifier, "ef", Tok::Ef, 2);
      m!(identifier, "el", Tok::El, 2);
      m!(identifier, "if", Tok::If, 2);
      m!(identifier, "in", Tok::In, 2);
      m!(identifier, "fn", Tok::Fn, 2);
      m!(identifier, "or", Tok::Or, 2);
      m!(identifier, "and", Tok::And, 3);
      m!(identifier, "for", Tok::For, 3);
      m!(identifier, "not", Tok::Not, 3);
      m!(identifier, "ret", Tok::Ret, 3);
      m!(identifier, "loop", Tok::Loop, 4);
      m!(identifier, "true", Tok::True, 4);
      m!(identifier, "break", Tok::Break, 5);
      m!(identifier, "false", Tok::False, 5);
      m!(identifier, "match", Tok::Match, 5);
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
