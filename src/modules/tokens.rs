use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum TokenKind {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Clone, Debug)]
pub enum Literals {
  
}

impl Display for Literals {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}



#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: Option<Box<Literals>>,
  pub lexeme: String,
  pub line: usize,
}


impl Token {
  pub fn new (kind: TokenKind, lexeme: String, literal: Option<Box<Literals>>, line: usize) -> Token {
    Token { kind, lexeme, line, literal }
  }
}

impl ToString for Token {

    fn to_string(&self) -> String {
      let literal: String = match &self.literal {
        Some(lit) => lit.to_string(),
        None => "No literal value".to_string(),
      };
      format!("{} {} {}", &self.kind, &self.lexeme, literal)
    }
}