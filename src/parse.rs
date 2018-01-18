use tokenize::{TokenType, Token};

const INDENT: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Node {
   Fn,
   FnDef,
   FnCall,
   Body,
   Ret,
   RetList,
   Assign,
   AddAssign,
   SubtractAssign,
   MultiplyAssign,
   DivideAssign,
   Equal,
   Unequal,
   LessEqual,
   GreaterEqual,
   Add,
   Subtract,
   Multiply,
   Divide,
   Match,
   MatchArm,
   MatchBody,
   Map,
   MapItem,
   If,
   El,
   For,
   Loop,
   List,
   Ident,
   Number,
   Symbol,
   String,
   Break,
}

type Res = Result<Option<(usize, Node)>, usize>;

fn no() -> Res {
   Ok(None)
}

fn ok(pos: usize, ast: Node) -> Res {
   Ok(Some((pos, ast)))
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

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      }

      loop {
         match self.fn_(current, 0) {
            Err(pos) => {
               self.error(pos);
               return;
            },
            Ok(Some((pos, ast))) => {
               current = pos;
               println!("[{}] done {:?}", pos, ast);
            }
            Ok(None) => {
               println!("Unreachable");
               self.error(current);
               return;
            }
         }

         if let Some(pos) = self.line_ends(current) {
            current = pos;
         }
      }
   }

   fn error(&self, pos: usize) {
      let token = &self.tokens[pos];
      let error = format!(
         "[{}] syntax error at line: {}, col: {}",
         pos, token.line, token.col
      );
      println!("{}", &error);
      println!("{}", "=".repeat(error.len()));

      let start = pos - ::std::cmp::min(pos, 5);
      let offset = ::std::cmp::min(11, self.tokens.len() - start);
      let slice = &self.tokens[start..start + offset];

      for (i, token) in slice.iter().enumerate() {
         let name = &self.input[token.pos..token.pos + token.span];
         println!("[{}] {:?} {:?}", start+i, token.ty, name);
      }
   }

   fn fn_(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> fn", pos);

      let mut current = pos;

      if let Some((pos, _ast)) = self.fn_def(current)? {
         current = pos;
      } else {
         return no();
      }

      if let Some((pos, _ast)) = self.body(current, indent + 1)? {
         current = pos;
      } else {
         println!("[{}] expected fn body", current);
         return Err(current);
      }

      ok(current, Node::Fn)
   }

   fn fn_def(&self, pos: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Fn) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      let name = if let Some((pos, ident)) = self.ident(current) {
         current = pos;
         ident
      } else {
         println!("[{}] expected function name", current);
         return Err(current);
      };

      current = self.skip_space(current);

      if let Some(pos) = self.token_type(current, TokenType::ParenLeft) {
         current = pos;
      } else {
         println!("[{}] expected (", current);
         return Err(current);
      }

      let (pos, args) = self.fn_args(current);
      current = pos;

      if let Some(pos) = self.token_type(current, TokenType::ParenRight) {
         current = pos;
      } else {
         println!("[{}] expected )", current);
         return Err(current);
      }

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on fn", current);
         return Err(current);
      }

      println!("[{}-{}] fn {} {:?}", pos, current-1, name, args);

      ok(current, Node::FnDef)
   }

   fn fn_args(&self, pos: usize) -> (usize, Vec<&'b str>) {
      let mut current = pos;
      let mut args = Vec::new();

      current = self.skip_white_space(current);

      loop {
         if let Some((pos, ident)) = self.ident(current) {
            current = pos;
            args.push(ident);

            current = self.skip_white_space(current)
         } else {
            return (current, args);
         }
      }
   }

   fn loop_(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> loop", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Loop) {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on loop", current);
         return Err(current);
      }

      println!("[{}-{}] loop", pos, current - 1);

      if let Some((pos, _ast)) = self.body(current, indent + 1)? {
         current = pos;
      } else {
         println!("[{}] expected loop body", current);
         return Err(current);
      }

      ok(current, Node::Loop)
   }

   fn if_(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> if", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::If) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      let ast = if let Some((pos, ast)) = self.end_expression(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      println!("[{}] if {:?}", pos, ast);

      if let Some((pos, _ast)) = self.body(current, indent + 1)? {
         current = pos;
      } else {
         println!("[{}] expected if body", current);
         return Err(current);
      }

      if let Some((pos, _ast)) = self.el(current, indent)? {
         current = pos;
      }

      println!("[{}-{}] if ...", pos, current - 1);

      ok(current, Node::If)
   }

   fn el(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> el", pos);

      let mut current = pos;

      if let Some(pos) = self.indent_space(current, indent) {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.token_type(current, TokenType::El) {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on el", current);
         return Err(current);
      }

      println!("[{}-{}] el", pos, current - 1);

      if let Some((pos, _ast)) = self.body(current, indent + 1)? {
         current = pos;
      } else {
         println!("[{}] expected loop body", current);
         return Err(current);
      }

      ok(current, Node::El)
   }

   fn match_(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> match", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Match) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      if let Some((pos, _ast)) = self.end_expression(current)? {
         current = pos;
      } else {
         return Err(current);
      }

      println!("[{}-{}] match", pos, current - 1);

      if let Some((pos, _ast)) = self.match_arms(current, indent + 1)? {
         current = pos;
      } else {
         return Err(current);
      }

      ok(current, Node::Match)
   }

   fn match_arms(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;
      let mut consumed = false;

      loop {
         if let Some((pos, _ast)) = self.match_arm(current, indent)? {
            current = pos;
            consumed = true;
         } else {
            if consumed {
               return ok(current, Node::MatchBody);
            } else {
               println!("[{}] expected match arms", current);
               return Err(current);
            }
         }
      }
   }

   fn match_arm(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> arm", pos);

      let mut current = pos;

      if let Some(pos) = self.indent_space(current, indent) {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.pattern(current) {
         current = pos;
      } else {
         println!("[{}] expected pattern", current);
         return Err(current);
      }

      current = self.skip_space(current);

      if let Some(pos) = self.token_type(current, TokenType::Colon) {
         current = pos;
      } else {
         println!("[{}] expected :", current);
         return Err(current);
      }

      println!("[{}-{}] pattern: ...", pos, current - 1);

      if let Some(pos) = self.line_ends(current) {
         current = pos;

         if let Some((pos, _ast)) = self.body(current, indent + 1)? {
            current = pos;
         } else {
            println!("[{}] expected fn body", current);
            return Err(current);
         }
      } else {
         current = self.skip_space(current);

         if let Ok(Some((pos, _))) = self.resulting(current, indent) {
            current = pos;
         } else {
            println!("[{}] expected expression", current);
            return Err(current);
         }
      }

      ok(current, Node::MatchArm)
   }

   fn for_(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> for", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::For) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      if let Some(pos) = self.pattern(current) {
         current = pos;
      } else {
         println!("[{}] expected pattern", current);
         return Err(current);
      }

      current = self.skip_space(current);

      if let Some(pos) = self.token_type(current, TokenType::In) {
         current = pos;
      } else {
         println!("[{}] expected in", current);
         return Err(current);
      }

      current = self.skip_space(current);

      let _ast = if let Some((pos, ast)) = self.end_expression(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some((pos, _ast)) = self.body(current, indent + 1)? {
         current = pos;
      } else {
         println!("[{}] expected for body", current);
         return Err(current);
      }

      ok(current, Node::For)
   }

   fn body(&self, pos: usize, indent: usize) -> Res {
      println!("[{}] -> body", pos);

      let mut current = pos;
      let mut last = current;
      let mut consumed = false;

      loop {
         if let Some((pos, _ast)) = self.statement(current, indent)? {
            current = pos;
            consumed = true;
            println!("[{}-{}] statement", last, current - 1);
            last = current;
         } else {
            if consumed {
               println!("[{}-{}] body ...", pos, current - 1);
               return ok(current, Node::Body);
            } else {
               return Err(current);
            }
         }
      }
   }

   fn statement(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      if let Some(pos) = self.indent_space(current, indent) {
         current = pos;
      } else {
         return no();
      }

      let ast = if let Some((pos, ast)) = self.assign(current, indent)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.for_(current, indent)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.loop_(current, indent)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.break_(current)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.ret(current)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.resulting(current, indent)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.ret_list(current)? {
         current = pos;
         ast
      } else {
         println!("[{}] unrecognized", current);
         return Err(current);
      };

      ok(current, ast)
   }

   fn ret(&self, pos: usize) -> Res {
      println!("[{}] -> ret", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Ret) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      let _ast = if let Some((pos, ast)) = self.single_line_list_items(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on ret", current);
         return Err(current);
      }

      ok(current, Node::Ret)
   }

   fn ret_list(&self, pos: usize) -> Res {
      println!("[{}] -> ret list", pos);

      let mut current = pos;

      let _ast = if let Some((pos, ast)) = self.ret_list_items(current)? {
         current = pos;
         ast
      } else {
         return no();
      };

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on ret list", current);
         return Err(current);
      }

      ok(current, Node::RetList)
   }

   fn ret_list_items(&self, pos: usize) -> Res {
      let mut current = pos;
      let mut consumed = 0;

      current = self.skip_space(current);

      loop {
         if let Some((pos, _ast)) = self.expression(current)? {
            current = pos;
            consumed += 1;

            current = self.skip_space(current)
         } else {
            if consumed >= 2 {
               return ok(current, Node::RetList);
            } else {
               return no()
            }
         }
      }
   }

   fn assign(&self, pos: usize, indent: usize) -> Res {
      let mut current = pos;

      let variable = if let Some((pos, ident)) = self.ident(current) {
         current = pos;
         ident
      } else {
         return no();
      };

      current = self.skip_space(current);

      let ast = if let Some((pos, ast)) = self.assign_type(
         current, indent, TokenType::Assign, Node::Assign
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.assign_type(
         current, indent, TokenType::AddAssign, Node::AddAssign
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.assign_type(
         current, indent, TokenType::SubtractAssign, Node::SubtractAssign
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.assign_type(
         current, indent, TokenType::MultiplyAssign, Node::MultiplyAssign
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.assign_type(
         current, indent, TokenType::DivideAssign, Node::DivideAssign
      )? {
         current = pos;
         ast
      } else {
         return no();
      };

      println!("[{}-{}] {:?} {}", pos, current - 1, ast, variable);

      ok(current, ast)
   }

   fn assign_type(&self, pos: usize, indent: usize, ty: TokenType, node: Node) -> Res {
      println!("[{}] -> {:?}", pos, ty);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, ty) {
         current = pos;
      } else {
         return no();
      }

      current = self.skip_space(current);

      if let Some((pos, _ast)) = self.resulting(current, indent)? {
         current = pos;
      } else {
         println!("[{}] expected expression", current);
         return Err(current);
      }

      ok(current, node)
   }

   fn binary_right(&self, pos: usize) -> Res {
      let mut current = pos;

      let ast = if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Equal, Node::Equal
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Unequal, Node::Unequal
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::LessEqual, Node::LessEqual
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::GreaterEqual, Node::GreaterEqual
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Add, Node::Add
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Subtract, Node::Subtract
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Multiply, Node::Multiply
      )? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.binary_type(
         current, TokenType::Divide, Node::Divide
      )? {
         current = pos;
         ast
      } else {
         return no();
      };

      current = self.skip_space(current);

      if let Some((pos, _ast)) = self.expression(current)? {
         current = pos;
      } else {
         println!("[{}] expected expression", current);
         return Err(current);
      }

      ok(current, ast)
   }

   fn binary_type(&self, pos: usize, ty: TokenType, node: Node) -> Res {
      println!("[{}] -> {:?}", pos, ty);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, ty) {
         current = pos;
      } else {
         return no();
      }

      ok(current, node)
   }

   fn resulting(&self, pos: usize, indent: usize) -> Res {
      if let Some((pos, ast)) = self.if_(pos, indent)? {
         ok(pos, ast)
      } else if let Some((pos, ast)) = self.match_(pos, indent)? {
         ok(pos, ast)
      } else if let Some((pos, ast)) = self.end_expression(pos)? {
         ok(pos, ast)
      } else {
         no()
      }
   }

   fn end_expression(&self, pos: usize) -> Res {
      let mut current = pos;

      let ast = if let Some((pos, ast)) = self.expression(current)? {
         current = pos;
         ast
      } else {
         return no();
      };

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         return no();
      }

      ok(current, ast)
   }

   fn expression(&self, pos: usize) -> Res {
      let mut current = pos;

      let ast = if let Some((pos, ast)) = self.fn_call(current)? {
         current = pos;
         ast
      } else if let Some((pos, _ident)) = self.ident(current) {
         current = pos;
         Node::Ident
      } else if let Some((pos, ast)) = self.value(current)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.list(current)? {
         current = pos;
         ast
      } else if let Some((pos, ast)) = self.map(current)? {
         current = pos;
         ast
      } else {
         return no();
      };

      current = self.skip_space(current);

      if let Some((pos, ast)) = self.binary_right(current)? {
         ok(pos, ast)
      } else {
         ok(current, ast)
      }
   }

   fn value(&self, pos: usize) -> Res {
      if let Some((pos, _number)) = self.number(pos) {
         println!("[{}] {}", pos, _number);
         ok(pos, Node::Number)
      } else if let Some((pos, _ident)) = self.symbol(pos) {
         println!("[{}] ^{}", pos, _ident);
         ok(pos, Node::Symbol)
      } else if let Some((pos, _ident)) = self.accent(pos) {
         println!("[{}] `{}", pos, _ident);
         ok(pos, Node::String)
      } else if let Some((pos, _ident)) = self.string(pos) {
         println!("[{}] '{}'", pos, _ident);
         ok(pos, Node::String)
      } else {
         no()
      }
   }

   fn fn_call(&self, pos: usize) -> Res {
      println!("[{}] -> fn call", pos);

      let mut current = pos;

      let name = if let Some((pos, ident)) = self.ident(current) {
         current = pos;
         ident
      } else {
         return no();
      };

      current = self.skip_space(current);

      if let Some(pos) = self.token_type(current, TokenType::ParenLeft) {
         current = pos;
      } else {
         return no();
      }

      let _ast = if let Some((pos, ast)) = self.list_items(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some(pos) = self.token_type(current, TokenType::ParenRight) {
         current = pos;
      } else {
         println!("[{}] expected )", current);
         return Err(current);
      }

      println!("[{}-{}] {}(...)", pos, current - 1, name);

      ok(current, Node::FnCall)
   }

   fn break_(&self, pos: usize) -> Res {
      println!("[{}] -> break", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Break) {
         current = pos;
      } else {
         return no();
      }

      if let Some(pos) = self.line_ends(current) {
         current = pos;
      } else {
         println!("[{}] expected new line on break", current);
         return Err(current);
      }

      ok(current, Node::Break)
   }

   fn list(&self, pos: usize) -> Res {
      println!("[{}] -> list", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::BracketLeft) {
         current = pos;
      } else {
         return no();
      }

      let ast = if let Some((pos, ast)) = self.list_items(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some(pos) = self.token_type(current, TokenType::BracketRight) {
         current = pos;
      } else {
         println!("[{}] expected ]", current);
         return Err(current);
      }

      println!("[{}-{}] [...]", pos, current - 1);

      ok(current, ast)
   }

   fn list_items(&self, pos: usize) -> Res {
      let mut current = pos;

      current = self.skip_white_space(current);

      loop {
         if let Some((pos, _ast)) = self.expression(current)? {
            current = pos;

            current = self.skip_white_space(current)
         } else {
            return ok(current, Node::List);
         }
      }
   }

   fn single_line_list_items(&self, pos: usize) -> Res {
      let mut current = pos;

      current = self.skip_space(current);

      loop {
         if let Some((pos, _ast)) = self.expression(current)? {
            current = pos;

            current = self.skip_space(current)
         } else {
            return ok(current, Node::List);
         }
      }
   }

   fn map(&self, pos: usize) -> Res {
      println!("[{}] -> map", pos);

      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::CurlyLeft) {
         current = pos;
      } else {
         return no();
      }

      let ast = if let Some((pos, ast)) = self.map_items(current)? {
         current = pos;
         ast
      } else {
         return Err(current);
      };

      if let Some(pos) = self.token_type(current, TokenType::CurlyRight) {
         current = pos;
      } else {
         println!("[{}] expected }}", current);
         return Err(current);
      }

      println!("[{}-{}] {{...}}", pos, current - 1);

      ok(current, ast)
   }

   fn map_items(&self, pos: usize) -> Res {
      let mut current = pos;

      current = self.skip_white_space(current);

      loop {
         if let Some((pos, _ast)) = self.map_key_value(current)? {
            current = pos;
         } else {
            return ok(current, Node::Map);
         }
      }
   }

   fn map_key_value(&self, pos: usize) -> Res {
      let mut current = pos;

      let _key_ast = if let Some((pos, ast)) = self.expression(current)? {
         current = pos;
         ast
      } else {
         return no();
      };

      current = self.skip_space(current);

      if let Some(pos) = self.token_type(current, TokenType::Colon) {
         current = pos;
      } else {
         println!("[{}] expected :", current);
         return Err(current);
      }

      current = self.skip_space(current);

      let _value_ast = if let Some((pos, ast)) = self.expression(current)? {
         current = pos;
         ast
      } else {
         println!("[{}] expected expression", current);
         return Err(current);
      };

      current = self.skip_white_space(current);

      ok(current, Node::MapItem)
   }

   fn pattern(&self, pos: usize) -> Option<usize> {
      let mut current = pos;
      let mut consumed = false;

      current = self.skip_white_space(current);

      loop {
         if let Some((pos, _ident)) = self.ident(current) {
            current = pos;
            consumed = true;
            current = self.skip_white_space(current)
         } else if let Ok(Some((pos, _ident))) = self.value(current) {
            current = pos;
            consumed = true;
            current = self.skip_white_space(current)
         } else {
            if consumed {
               return Some(current);
            } else {
               return None;
            }
         }
      }
   }

   fn indent_space(&self, pos: usize, indent: usize) -> Option<usize> {
      if let Some(token) = self.tokens.get(pos) {
         if token.ty == TokenType::Space && token.span == indent * INDENT {
            println!("[{}] indent {}", pos, indent);
            return Some(pos + 1);
         } else {
            println!("[{}] indent {} != {} * {}", pos, token.span, indent, INDENT);
         }
      }

      None
   }

   fn ident(&self, pos: usize) -> Option<(usize, &'b str)> {
      println!("[{}] -> ident", pos);

      self.token_string(pos, TokenType::Ident)
   }

   fn number(&self, pos: usize) -> Option<(usize, &'b str)> {
      println!("[{}] -> number", pos);

      self.token_string(pos, TokenType::Number)
   }

   fn line_ends(&self, pos: usize) -> Option<usize> {
      let mut current = pos;
      let mut consumed = false;

      loop {
         if let Some(pos) = self.line_end(current) {
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

   fn line_end(&self, pos: usize) -> Option<usize> {
      let mut current = pos;

      if let Some(pos) = self.token_type(current, TokenType::Space) {
         current = pos;
      }

      if let Some(pos) = self.eol_eof(current) {
         Some(pos)
      } else {
         None
      }
   }

   fn eol_eof(&self, pos: usize) -> Option<usize> {
      if let Some(token) = self.tokens.get(pos) {
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

   fn accent(&self, pos: usize) -> Option<(usize, &'b str)> {
      println!("[{}] -> accent", pos);

      if let Some(token) = self.tokens.get(pos) {
         if token.ty == TokenType::Accent {
            let string = &self.input[token.pos + 1..token.pos + token.span];
            println!("[{}] `{}", pos, string);
            return Some((pos + 1, string));
         }
      }

      None
   }

   fn string(&self, pos: usize) -> Option<(usize, &'b str)> {
      println!("[{}] -> string", pos);

      if let Some(token) = self.tokens.get(pos) {
         if token.ty == TokenType::String {
            let string = &self.input[token.pos + 1..token.pos + token.span - 1];
            println!("[{}] '{}'", pos, string);
            return Some((pos + 1, string));
         }
      }

      None
   }

   fn symbol(&self, pos: usize) -> Option<(usize, &'b str)> {
      println!("[{}] -> symbol", pos);

      if let Some(token) = self.tokens.get(pos) {
         if token.ty == TokenType::Symbol {
            let ident = &self.input[token.pos + 1..token.pos + token.span];
            println!("[{}] ^{}", pos, ident);
            return Some((pos + 1, ident));
         }
      }

      None
   }

   fn token_string(&self, pos: usize, ty: TokenType) -> Option<(usize, &'b str)> {
      if let Some(token) = self.tokens.get(pos) {
         if token.ty == ty {
            let ident = &self.input[token.pos..token.pos + token.span];
            println!("[{}] {:?} \"{}\"", pos, ty, ident);
            return Some((pos + 1, ident));
         }
      }

      None
   }

   fn token_type(&self, pos: usize, ty: TokenType) -> Option<usize> {
      if let Some(token) = self.tokens.get(pos) {
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
      if let Some(token) = self.tokens.get(pos) {
         if token.ty == ty {
            println!("[{}] -> {:?}", pos, ty);
            return pos + 1;
         }
      }

      pos
   }
}
