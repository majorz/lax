use tokenize::{TokenType, Token};

const INDENT: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Ast {
   Fn,
   FnCall,
   Block,
   Assignment,
   Match,
   If,
   Loop,
   List,
   Ident,
   Number,
}

type Res = Result<Option<(usize, Option<Ast>)>, usize>;

fn pos(pos: usize) -> Res {
   Ok(Some((pos, None)))
}

fn no() -> Res {
   Ok(None)
}

fn okast(pos: usize, ast: Ast) -> Res {
   Ok(Some((pos, Some(ast))))
}

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

   pub fn parse(&self) {
      let mut current = 0;

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      }

      match self.consume_fn(current, 0) {
         Err(pos) => {
            let token = &self.tokens[pos];
            println!(
               "[{}] syntax error at line: {}, col: {}",
               pos, token.line, token.col
            );
            let offset = ::std::cmp::min(10, self.tokens.len() - pos);
            let slice = &self.tokens[pos..pos + offset];

            for (i, token) in slice.iter().enumerate() {
               let name = &self.input[token.pos..token.pos + token.span];
               println!("[{}] {:?} {:?}", pos+i, token.ty, name);
            }
         },
         Ok(Some((pos, Some(ast)))) => {
            println!("[{}] done {:?}", pos, ast);
         }
         Ok(Some((pos, None))) => {
            let token = &self.tokens[pos];
            println!(
               "[{}] error at line: {}, col: {}",
               pos, token.line, token.col
            );
         },
         Ok(None) => {
            println!("Unreachable");
         }
      }

      let _ = current;
   }

   fn consume_fn(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.consume_fn_def(current) {
         current = pos;
      } else {
         return Err(current);
      }

      if let Some((pos, Some(_ast))) = self.consume_block(current, indent + 1)? {
         current = pos;
      } else {
         return Err(current);
      }

      okast(current, Ast::Fn)
   }

   fn consume_fn_def(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.consume_exact_ident(current, "fn") {
         current = pos;
      } else {
         return None;
      }

      current = self.skip_space(current);

      let name = if let Some((pos, ident)) = self.consume_ident(current) {
         current = pos;
         ident
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

   fn consume_loop(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.consume_exact_ident(current, "loop") {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      } else {
         return Err(current);
      }

      println!("[{}-{}] loop", pos, current - 1);

      if let Some((pos, Some(_ast))) = self.consume_block(current, indent + 1)? {
         current = pos;
      } else {
         return Err(current);
      }

      okast(current, Ast::Loop)
   }

   fn consume_if(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.consume_exact_ident(current, "if") {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      if let Some((pos, Some(_ast))) = self.consume_expression(current)? {
         current = pos;
      } else {
         return Err(current);
      }

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      } else {
         return Err(current);
      }

      println!("[{}-{}] if", pos, current - 1);

      if let Some((pos, Some(_ast))) = self.consume_block(current, indent + 1)? {
         current = pos;
      } else {
         return Err(current);
      }

      println!("[{}-{}] if end", pos, current - 1);

      okast(current, Ast::If)
   }

   fn consume_match(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.consume_exact_ident(current, "match") {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      if let Some((pos, Some(_ast))) = self.consume_expression(current)? {
         current = pos;
      } else {
         return Err(current);
      }

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      } else {
         return Err(current);
      }

      println!("[{}-{}] match", pos, current - 1);

      if let Some(pos) = self.consume_match_arms(current, indent + 1) {
         current = pos;
      } else {
         return Err(current);
      }

      okast(current, Ast::Match)
   }

   fn consume_block(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;
      let mut consumed = false;

      loop {
         if let Some((pos, Some(_ast))) = self.consume_statement(current, indent)? {
            current = pos;
            consumed = true;
         } else {
            if consumed {
               return okast(current, Ast::Block);
            } else {
               return Err(current);
            }
         }
      }
   }

   fn consume_match_arms(&self, pos: usize, indent: usize) -> Option<usize> {
      let mut current = pos;
      let mut consumed = false;

      loop {
         if let Some(pos) = self.consume_match_arm(current, indent) {
            current = pos;
            consumed = true;
         } else {
            if consumed {
               return Some(current);
            } else {
               return None;
            }
         }
      }
   }

   fn consume_match_arm(&self, pos: usize, indent: usize) -> Option<usize> {
      let mut current = pos;

      let variable = if let Some((pos, ident)) = self.consume_ident(current) {
         current = pos;
         ident
      } else {
         return None;
      };

      current = self.skip_space(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::Colon) {
         current = pos;
      } else {
         return None;
      }

      println!("[{}-{}] {}: ...", pos, current - 1, variable);

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;

         if let Ok(Some((pos, Some(_ast)))) = self.consume_block(current, indent + 1) {
            current = pos;
         } else {
            return None;
         }
      } else {
         current = self.skip_space(current);

         if let Ok(Some((pos, _))) = self.consume_resulting(current, indent) {
            current = pos;
         } else {
            return None;
         }
      }

      Some(current)
   }

   fn consume_statement(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.consume_indent_space(current, indent) {
         current = pos;
      } else {
         return Err(current);
      }

      let ast = if let Some(pos) = self.consume_assignment(current, indent) {
         current = pos;
         Ast::Assignment
      } else if let Some((pos, Some(ast))) = self.consume_loop(current, indent)? {
         current = pos;
         ast
      } else if let Some((pos, Some(ast))) = self.consume_resulting(current, indent)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some(pos) = self.consume_empty_lines(current) {
         current = pos;
      } else {
         return Err(current);
      }

      okast(current, ast)
   }

   fn consume_assignment(&self, pos: usize, indent: usize) -> Option<usize> {
      let mut current = pos;

      let variable = if let Some((pos, ident)) = self.consume_ident(current) {
         current = pos;
         ident
      } else {
         return None;
      };

      current = self.skip_space(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::Assign) {
         current = pos;
      } else {
         return None;
      }

      current = self.skip_space(current);

      if let Ok(Some((pos, _))) = self.consume_resulting(current, indent) {
         current = pos;
      } else {
         return None;
      }

      println!("[{}-{}] {} = ", pos, current - 1, variable);

      Some(current)
   }

   fn consume_resulting(&self, pos: usize, indent: usize) -> Res {
      if let Some((pos, Some(ast))) = self.consume_if(pos, indent)? {
         okast(pos, ast)
      } else if let Some((pos, Some(ast))) = self.consume_match(pos, indent)? {
         okast(pos, ast)
      } else if let Some((pos, Some(ast))) = self.consume_expression(pos)? {
         okast(pos, ast)
      } else {
         no()
      }
   }

   fn consume_expression(&self, pos: usize) -> Res {
      if let Some(pos) = self.consume_list(pos) {
         okast(pos, Ast::List)
      } else if let Some(pos) = self.consume_fn_call(pos) {
         okast(pos, Ast::FnCall)
      } else if let Some((pos, _ident)) = self.consume_ident(pos) {
         okast(pos, Ast::Ident)
      } else if let Some((pos, _number)) = self.consume_number(pos) {
         okast(pos, Ast::Number)
      } else {
         Err(pos)
      }
   }

   fn consume_fn_call(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      let name = if let Some((pos, ident)) = self.consume_ident(current) {
         current = pos;
         ident
      } else {
         return None;
      };

      current = self.skip_space(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::ParenLeft) {
         current = pos;
      } else {
         return None;
      }

      current = self.consume_list_expressions(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::ParenRight) {
         current = pos;
      } else {
         return None;
      }

      println!("[{}-{}] {}(..)", pos, current - 1, name);

      Some(current)
   }

   fn consume_list(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.consume_token_type(current, TokenType::BracketLeft) {
         current = pos;
      } else {
         return None;
      }

      current = self.consume_list_expressions(current);

      if let Some(pos) = self.consume_token_type(current, TokenType::BracketRight) {
         current = pos;
      } else {
         return None;
      }

      println!("[{}-{}] [..]", pos, current - 1);

      Some(current)
   }

   fn consume_list_expressions(&self, pos: usize) -> usize {
      let mut current = pos;

      current = self.skip_white_space(current);

      loop {
         if let Ok(Some((pos, Some(_ast)))) = self.consume_expression(current) {
            current = pos;

            current = self.skip_white_space(current)
         } else {
            return current;
         }
      }
   }

   fn consume_indent_space(&self, pos: usize, indent: usize) -> Option<usize> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == TokenType::Space && token.span == indent * INDENT {
            println!("[{}] indent {}", pos, indent);
            return Some(pos + 1);
         }
      }

      None
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
      self.consume_token_string(pos, TokenType::Ident)
   }

   fn consume_number(&self, pos: usize) -> Option<(usize, &'b str)> {
      self.consume_token_string(pos, TokenType::Number)
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

   fn consume_token_string(&self, pos: usize, ty: TokenType) -> Option<(usize, &'b str)> {
      if let Some(token) = self.tokens[pos..].first() {
         if token.ty == ty {
            let ident = &self.input[token.pos..token.pos + token.span];
            println!("[{}] {:?} \"{}\"", pos, ty, ident);
            return Some((pos + 1, ident));
         }
      }

      None
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
