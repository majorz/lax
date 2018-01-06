use tokenize::{TokenType, Token};

type ConsumeFn = fn(&[Token]) -> Option<&[Token]>;

pub fn parse(tokens: &[Token]) {
   println!("S LEN: {}", tokens.len());

   if let Some(tokens) = consume_empty_lines(tokens) {
      println!("E LEN: {}", tokens.len());
   }
}

fn consume_empty_lines(tokens: &[Token]) -> Option<&[Token]> {
   consume_repeat(tokens, consume_empty_line)
}

fn consume_empty_line(tokens: &[Token]) -> Option<&[Token]> {
   let mut consumed_tokens = tokens;

   if let Some(tokens) = consume_spaces(consumed_tokens) {
      consumed_tokens = tokens;
   }

   if let Some(tokens) = consume_new_line(consumed_tokens) {
      Some(tokens)
   } else {
      None
   }
}

fn consume_spaces(tokens: &[Token]) -> Option<&[Token]> {
   consume_repeat(tokens, consume_space)
}

fn consume_space(tokens: &[Token]) -> Option<&[Token]> {
   if let Some(token) = tokens.first() {
      if token.ty == TokenType::Space || token.ty == TokenType::Tab {
         return Some(&tokens[1..]);
      }
   }

   None
}

fn consume_new_line(tokens: &[Token]) -> Option<&[Token]> {
   if let Some(token) = tokens.first() {
      if token.ty == TokenType::NewLine {
         Some(&tokens[1..])
      } else {
         None
      }
   } else {
      Some(tokens)
   }
}

fn consume_repeat(tokens: &[Token], consume_fn: ConsumeFn) -> Option<&[Token]> {
   let mut consumed_tokens = tokens;
   let mut consumed = false;

   loop {
      if let Some(tokens) = consume_fn(consumed_tokens) {
         consumed_tokens = tokens;
         consumed = true;
         continue;
      }

      return if consumed {
         Some(consumed_tokens)
      } else {
         None
      };
   }
}
