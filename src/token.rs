#[derive(Debug, Clone)]
pub enum TokenType {
  // Symbols
  LeftParen,
  RightParen,
  LeftSquare,
  RightSquare,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Plus,
  Minus,
  Star,
  Slash,
  Semicolon,

  Bang,
  BangEqual,
  Equal,
  DoubleEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  String,
  Character,
  Number(f64),

  // Words
  Ident,
  And,
  Or,
  Self_,
  Struct,
  True,
  False,
  Fn,
  If,
  Else,
  Nil,
  Print,
  Return,
  Super,
  Let,
  While,
  For,

  // Special
  Unrecognized,
  TooLong,
}

/// Type, index
#[derive(Debug, Clone)]
pub struct Token {
  pub ttype: TokenType,
  pub start: usize,
  pub end: usize,
}

pub fn tokenize(input: &str) -> Vec<Token> {
  let input_str = input;
  let mut input = input.char_indices().peekable();
  let mut tokens = vec![];
  'outer: loop {
    // Find next non-whitespace line
    let (start, c) = 'ws: loop {
      match input.next() {
        // Stop at end of input
        None => break 'outer,
        Some((index, character)) if !character.is_whitespace() => {
          break 'ws (index, character)
        },
        _ => {},
      }
    };
    let mut end = start + 1;
    let mut advance = || {};
    let ttype = match c {
      // Match single character tokens
      '(' => TokenType::LeftParen,
      ')' => TokenType::RightParen,
      '[' => TokenType::LeftSquare,
      ']' => TokenType::RightSquare,
      '{' => TokenType::LeftBrace,
      '}' => TokenType::RightBrace,
      ',' => TokenType::Comma,
      '.' => TokenType::Dot,
      '+' => TokenType::Plus,
      '-' => TokenType::Minus,
      '*' => TokenType::Star,
      '/' => TokenType::Slash,
      ';' => TokenType::Semicolon,
      // Match multicharacter tokens
      '!' => match input.peek() {
        Some((_, '=')) => {
          input.next();
          end += 1;
          TokenType::BangEqual
        },
        _ => TokenType::Bang,
      },
      '=' => match input.peek() {
        Some((_, '=')) => {
          input.next();
          end += 1;
          TokenType::DoubleEqual
        },
        _ => TokenType::Equal,
      },
      '<' => match input.peek() {
        Some((_, '=')) => {
          input.next();
          end += 1;
          TokenType::GreaterEqual
        },
        _ => TokenType::Greater,
      },
      '>' => match input.peek() {
        Some((_, '=')) => {
          input.next();
          end += 1;
          TokenType::LessEqual
        },
        _ => TokenType::Less,
      },
      // Match keywords, identifiers, and literals
      c if c.is_alphanumeric() => 'case: {
        // Scan full word
        while let Some((new_end, next)) = input.peek() {
          if next.is_alphanumeric() || *next == '_' {
            let _ = input.next();
          } else {
            end = *new_end;
            break;
          }
        }
        let word = &input_str[start..end];
        // Attempt to parse hex literal
        if let Some(s) =
          word.strip_prefix("0x").or_else(|| word.strip_prefix("0X"))
        {
          if let Ok(n) = u64::from_str_radix(s, 16) {
            break 'case TokenType::Number(n as f64);
          } else {
            break 'case TokenType::Unrecognized;
          }
        }
        // Attempt to parse binary literal
        if let Some(s) =
          word.strip_prefix("0b").or_else(|| word.strip_prefix("0B"))
        {
          if let Ok(n) = u64::from_str_radix(s, 2) {
            break 'case TokenType::Number(n as f64);
          } else {
            break 'case TokenType::Unrecognized;
          }
        }
        // Attempt to parse decimal literal
        if let Ok(f) = word.parse::<f64>() {
          break 'case TokenType::Number(f);
        }
        // Parse keyword or ident
        match word {
          "and" => TokenType::And,
          "or" => TokenType::Or,
          "self" => TokenType::Self_,
          "struct" => TokenType::Struct,
          "true" => TokenType::True,
          "false" => TokenType::False,
          "fn" => TokenType::Fn,
          "if" => TokenType::If,
          "else" => TokenType::Else,
          "nil" => TokenType::Nil,
          "print" => TokenType::Print,
          "return" => TokenType::Return,
          "super" => TokenType::Super,
          "let" => TokenType::Let,
          "while" => TokenType::While,
          "for" => TokenType::For,
          _ => TokenType::Ident,
        }
      },
      // Parse string
      '"' => {
        while let Some((new_end, next)) = input.next() {
          match next {
            '"' => {
              end = new_end + 1;
              break;
            },
            // Skip escapes and deal with them later
            '\\' => {
              let _ = input.next();
            },
            _ => {},
          }
        }
        TokenType::String
      },
      // Parse character
      _ => TokenType::Unrecognized,
    };
    tokens.push(Token { ttype, start, end });
  }
  tokens
}
