use super::tokens::Literals;
use super::tokens::Token;
use super::tokens::TokenKind;

pub struct Scanner {
  source: String,
  start: usize,
  current: usize,
  line: usize,
  tokens: Vec<Token>,
}

impl Scanner {
  pub fn new(source: String) -> Self {
    Scanner {
      source,
      start: 0,
      current: 0,
      line: 1,
      tokens: vec!(),
    }
  }

  fn is_at_end(&mut self) -> bool {
    self.current >= self.source.len()
  }
  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }
    self.tokens.push(
      Token::new(TokenKind::Eof, "".to_owned(), None, self.line)
    );
    let tokens_clone = &*self.tokens.clone();
    tokens_clone.to_owned()
  }

  fn scan_token(&mut self) {
    let char = self.advance();

    match char {
      '(' => self.add_token(TokenKind::LeftParen, None),
      ')' => self.add_token(TokenKind::RightParen, None),
      '{' => self.add_token(TokenKind::LeftBrace, None),
      '}' => self.add_token(TokenKind::RightBrace, None),
      ',' => self.add_token(TokenKind::Comma, None),
      '.' => self.add_token(TokenKind::Dot, None),
      '-' => self.add_token(TokenKind::Minus, None),
      '+' => self.add_token(TokenKind::Plus, None),
      ';' => self.add_token(TokenKind::Semicolon, None),
      '*' => self.add_token(TokenKind::Star, None),
      _ => {}
    };
    
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source.chars().nth(self.current - 1).unwrap()
  }


  fn add_token(&mut self, token: TokenKind, literal: Option<Box<Literals>>) {
    let text = self.source[self.start..self.current].to_owned();
    self.tokens.push(
      Token::new(token, text, literal, self.line)
    );
  }

  
  
}