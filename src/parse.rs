use tokenize::{TokenType, Token};

//const INDENT: usize = 3;

pub struct Parser<'a, 'b> {
   tokens: &'a [Token],
   input: &'b str,
}

impl<'a, 'b> Parser<'a, 'b> {
   pub fn init(tokens: &'a [Token], input: &'b str) -> Self {
      Parser {
         tokens: tokens,
         input: input,
      }
   }

   pub fn parse(&mut self) {
      let mut current = 0;

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      }

      if let Some(pos) = self.consume_fn(current, 0) {
         current = pos;
      }

      let _ = current;
   }

   fn consume_fn(&self, pos: usize, _indent: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.consume_fn_def(current) {
         current = pos;
      } else {
         return None;
      }

      if let Some(pos) = self.consume_fn_body(current) {
         current = pos;
      } else {
         return None;
      }

      Some(current)
   }

   fn consume_fn_def(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.consume_exact_ident(current, "fn") {
         current = pos;
      } else {
         return None;
      }

      current = self.skip_space(current);

      let name = if let Some((pos, name)) = self.consume_ident(current) {
         current = pos;
         name
      } else {
         return None;
      };

      current = self.skip_space(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::ParenLeft) {
         current = pos;
      } else {
         return None;
      }

      let (pos, args) = self.consume_fn_args(current);
      current = pos;

      if let Some(pos) = self.consume_token_type(current, TokenType::ParenRight) {
         current = pos;
      } else {
         return None;
      }

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      } else {
         return None;
      }

      println!("[{}-{}] fn {} {:?}", pos, current-1, name, args);

      Some(current)
   }

   fn consume_fn_args(&self, pos: usize) -> (usize, Vec<&'b str>) {
      let mut current = pos;
      let mut args = Vec::new();

      current = self.skip_white_space(current);

      loop {
         if let Some((pos, ident)) = self.consume_ident(current) {
            current = pos;
            args.push(ident);

            current = self.skip_white_space(current)
         } else {
            return (current, args);
         }
      }
   }

   fn consume_fn_body(&self, pos: usize) -> Option<usize> {
      Some(pos)
   }

   fn consume_exact_ident(&self, pos: usize, ident: &str) -> Option<usize> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == TokenType::Ident {
            if ident.len() == token.span && self.input[token.pos..].starts_with(ident) {
               println!("[{}] {}", pos, ident);
               return Some(pos + 1);
            }
         }
      }

      None
   }

   fn consume_ident(&self, pos: usize) -> Option<(usize, &'b str)> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == TokenType::Ident {
            let ident = &self.input[token.pos..token.pos + token.span];
            println!("[{}] \"{}\"", pos, ident);
            return Some((pos + 1, ident));
         }
      }

      None
   }

   fn consume_empty_lines(&self, pos: usize) -> Option<usize> {
      let mut current = pos;
      let mut consumed = false;

      loop {
         if let Some(pos) = self.consume_empty_line(current) {
            current = pos;
            consumed = true;
            continue;
         }

         if consumed {
            return Some(current);
         } else {
            return None;
         }
      }
   }

   fn consume_empty_line(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.consume_token_type(current, TokenType::Space) {
         current = pos;
      }

      if let Some(pos) = self.consume_eol_eof(current) {
         Some(pos)
      } else {
         None
      }
   }

   fn consume_eol_eof(&self, pos: usize) -> Option<usize> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == TokenType::NewLine {
            println!("[{}] eol", pos);
            Some(pos + 1)
         } else {
            None
         }
      } else {
         println!("[{}] eof", pos);
         Some(pos)
      }
   }

   fn consume_token_type(&self, pos: usize, ty: TokenType) -> Option<usize> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == ty {
            println!("[{}] {:?}", pos, ty);
            return Some(pos + 1);
         }
      }

      None
   }

   fn skip_white_space(&self, pos: usize) -> usize {
      let mut pos = pos;

      loop {
         let start = pos;

         pos = self.skip_space(pos);

         pos = self.skip_token_type(pos, TokenType::NewLine);

         if pos == start {
            return pos;
         }
      }
   }

   fn skip_space(&self, pos: usize) -> usize {
      self.skip_token_type(pos, TokenType::Space)
   }

   fn skip_token_type(&self, pos: usize, ty: TokenType) -> usize {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == ty {
            println!("[{}] {:?} sk", pos, ty);
            return pos + 1;
         }
      }

      pos
   }
}
