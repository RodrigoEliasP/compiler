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

#[cfg(test)]
mod tests {
  use std::fs::read_to_string;

  use super::*;
  #[test]
  fn should_scan_string_correctly() {
    let testing_file_contents = read_to_string("./lox_scripts/lexing_test.txt").unwrap();
    let mut scanner = Scanner::new(testing_file_contents);
    scanner.scan_tokens();

    let tokens = scanner.tokens;

    let first_token = tokens[0].clone();
    assert_eq!(first_token.kind, TokenKind::LeftBrace);
    assert_eq!(first_token.line, 1);
    assert_eq!(first_token.lexeme, "{");
    assert!(first_token.literal.is_none());

    let second_token = tokens[1].clone();
    assert_eq!(second_token.kind, TokenKind::RightBrace);
    assert_eq!(second_token.line, 1);
    assert_eq!(second_token.lexeme, "}");
    assert!(second_token.literal.is_none());

    let third_token = tokens[2].clone();
    assert_eq!(third_token.kind, TokenKind::Plus);
    assert_eq!(third_token.line, 1/* like break not implemented yet */);
    assert_eq!(third_token.lexeme, "+");
    assert!(third_token.literal.is_none());
  }
}