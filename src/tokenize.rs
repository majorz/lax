use regex::Regex;
use std::cmp;

#[derive(Debug, Copy, Clone)]
pub enum Token {
   Space,
   Tab,
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

const REGEX_MAP: [(&'static str, Token); 32] = [
   (r"^ +",    Token::Space),
   (r"^\t+",   Token::Tab),
   (r"^\n",    Token::NewLine),
   (r"^\*\*",  Token::Power),
   (r"^==",    Token::Equal),
   (r"^!=",    Token::Unequal),
   (r"^<=",    Token::LessEqual),
   (r"^>=",    Token::GreaterEqual),
   (r"^\+=",   Token::Increase),
   (r"^-=",    Token::Decrease),
   (r"^\.",    Token::Dot),
   (r"^\^",    Token::Caret),
   (r"^=",     Token::Assign),
   (r"^\+",    Token::Add),
   (r"^-",     Token::Subtract),
   (r"^\*",    Token::Multiply),
   (r"^/",     Token::Divide),
   (r"^\|",    Token::Bar),
   (r"^:",     Token::Colon),
   (r"^\(",    Token::ParenLeft),
   (r"^\)",    Token::ParenRight),
   (r"^\[",    Token::BracketLeft),
   (r"^\]",    Token::BracketRight),
   (r"^<",     Token::AngleLeft),
   (r"^>",     Token::AngleRight),
   (r"^\{",    Token::CurlyLeft),
   (r"^}",     Token::CurlyRight),
   (r"^#[^\n]",                  Token::Comment),
   (r"^`(?:\\\)|[^)\s])+",       Token::Accent),
   (r"^'(?:\\'|[^'])*'",         Token::String),
   (r"^[_A-Za-z]+[_A-Za-z0-9]*", Token::Ident),
   (r"^[0-9]+\.*[0-9]*",         Token::Number),
];

pub fn tokenize(mut input: &str) -> Vec<Token> {
   println!("{:?}", &input[..cmp::min(30, input.len())]);

   let mut count = 0;
   let mut tokens = vec![];

   let regex_map = get_regex_map();

   while !input.is_empty() {
      match match_token(input, &regex_map) {
         Some((token, span)) => {
            println!("{:?} {}", token, span);
            tokens.push(token);
            input = &input[span..];
            println!("{:?}", &input[..cmp::min(30, input.len())]);
         },
         None => panic!("Unrecognized: {:?}", input),
      }

      count += 1;
      if count == 10000 {
         break;
      }
   }

   tokens
}

fn get_regex_map() -> Vec<(Regex, Token)> {
   let mut regex_map = vec![];

   for &(expression, token) in &REGEX_MAP {
      regex_map.push((Regex::new(expression).unwrap(), token));
   }

   regex_map
}

fn match_token(input: &str, regex_map: &[(Regex, Token)]) -> Option<(Token, usize)> {
   for &(ref re, token) in regex_map {
      if let Some(res) = re.find(input) {
         return Some((token, res.end()));
      }
   }

   None
}
