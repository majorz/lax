use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
   Space,
   NewLine,
   Power,
   Equal,
   Unequal,
   LessEqual,
   GreaterEqual,
   Increase,
   Decrease,
   Dot,
   Caret,
   Assign,
   Add,
   Subtract,
   Multiply,
   Divide,
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
   Number,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
   pub ty: TokenType,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

const REGEX_MAP: [(&'static str, TokenType); 31] = [
   (r"^ +",    TokenType::Space),
   (r"^\n",    TokenType::NewLine),
   (r"^\*\*",  TokenType::Power),
   (r"^==",    TokenType::Equal),
   (r"^!=",    TokenType::Unequal),
   (r"^<=",    TokenType::LessEqual),
   (r"^>=",    TokenType::GreaterEqual),
   (r"^\+=",   TokenType::Increase),
   (r"^-=",    TokenType::Decrease),
   (r"^\.",    TokenType::Dot),
   (r"^\^",    TokenType::Caret),
   (r"^=",     TokenType::Assign),
   (r"^\+",    TokenType::Add),
   (r"^-",     TokenType::Subtract),
   (r"^\*",    TokenType::Multiply),
   (r"^/",     TokenType::Divide),
   (r"^\|",    TokenType::Bar),
   (r"^:",     TokenType::Colon),
   (r"^\(",    TokenType::ParenLeft),
   (r"^\)",    TokenType::ParenRight),
   (r"^\[",    TokenType::BracketLeft),
   (r"^\]",    TokenType::BracketRight),
   (r"^<",     TokenType::AngleLeft),
   (r"^>",     TokenType::AngleRight),
   (r"^\{",    TokenType::CurlyLeft),
   (r"^}",     TokenType::CurlyRight),
   (r"^#[^\n]",                  TokenType::Comment),
   (r"^`(?:\\\)|[^)\s])+",       TokenType::Accent),
   (r"^'(?:\\'|[^'])*'",         TokenType::String),
   (r"^[_A-Za-z]+[_A-Za-z0-9]*", TokenType::Ident),
   (r"^[0-9]+\.*[0-9]*",         TokenType::Number),
];

pub fn tokenize(input: &str) -> Vec<Token> {
   let mut tokens = vec![];

   let regex_map = get_regex_map();

   let mut pos = 0;
   let mut line = 1;
   let mut col = 1;

   while pos < input.len() {
      match match_token(&input[pos..], &regex_map) {
         Some((ty, span)) => {
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
         },
         None => panic!("Unrecognized token at line: {}, col: {}", line, col),
      }
   }

   tokens
}

fn get_regex_map() -> Vec<(Regex, TokenType)> {
   let mut regex_map = vec![];

   for &(expression, ty) in &REGEX_MAP {
      regex_map.push((Regex::new(expression).unwrap(), ty));
   }

   regex_map
}

fn match_token(input: &str, regex_map: &[(Regex, TokenType)]) -> Option<(TokenType, usize)> {
   for &(ref re, ty) in regex_map {
      if let Some(res) = re.find(input) {
         return Some((ty, res.end()));
      }
   }

   None
}
