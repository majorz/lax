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
   AddAssign,
   SubtractAssign,
   MultiplyAssign,
   DivideAssign,
   Dot,
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
   Symbol,
   Number,
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
   And,
   Or,
   Not,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
   pub ty: TokenType,
   pub span: usize,
   pub pos: usize,
   pub line: usize,
   pub col: usize,
}

const REGEX_MAP: [(&'static str, TokenType); 33] = [
   (r"^ +",    TokenType::Space),
   (r"^\n",    TokenType::NewLine),
   (r"^\*\*",  TokenType::Power),
   (r"^==",    TokenType::Equal),
   (r"^!=",    TokenType::Unequal),
   (r"^<=",    TokenType::LessEqual),
   (r"^>=",    TokenType::GreaterEqual),
   (r"^\+=",   TokenType::AddAssign),
   (r"^-=",    TokenType::SubtractAssign),
   (r"^\*=",   TokenType::MultiplyAssign),
   (r"^/=",    TokenType::DivideAssign),
   (r"^\.",    TokenType::Dot),
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
   (r"^#[^\n]",                    TokenType::Comment),
   (r"^`(?:\\\)|[^)\s])+",         TokenType::Accent),
   (r"^'(?:\\'|[^'])*'",           TokenType::String),
   (r"^[_A-Za-z]+[_A-Za-z0-9]*",   TokenType::Ident),
   (r"^\^[_A-Za-z]+[_A-Za-z0-9]*", TokenType::Symbol),
   (r"^[0-9]+\.*[0-9]*",           TokenType::Number),
];

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

pub fn tokenize(input: &str) -> Vec<Token> {
   let mut tokens = vec![];

   let regex_map = get_regex_map();

   let mut pos = 0;
   let mut line = 1;
   let mut col = 1;

   while pos < input.len() {
      match match_token(&input[pos..], &regex_map) {
         Some((ty, span)) => {
            let ty = try_ident_as_keyword(&input[pos..pos+span], ty);

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
         },
         None => panic!("Unrecognized token at line: {}, col: {}", line, col),
      }
   }

   tokens
}

fn get_regex_map() -> Vec<(Regex, TokenType)> {
   let mut regex_map = vec![];

   for &(expression, ty) in REGEX_MAP.iter() {
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

fn try_ident_as_keyword(input: &str, ty: TokenType) -> TokenType {
   if ty != TokenType::Ident {
      return ty;
   }

   for &(keyword, keyword_ty) in KEYWORD_MAP.iter() {
      if keyword == input {
         return keyword_ty;
      }
   }

   ty
}
