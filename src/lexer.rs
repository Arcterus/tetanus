/*
example:

fn fib(x) {
   3 + 4
}
-------------
Fn Ident(fib) LParen Ident(x) RParen LBrack Int(3) Plus Int(4) RBrack

let x = 5
---------
Let Ident(x) Equal Int(5)

*/

use std::fmt::{Formatter, FormatError, Show};

#[deriving(Show)]
pub enum Token {
   If,
   Loop,
   Break,
   Continue,
   Let,
   LParen,
   RParen,
   LBrace,
   RBrace,
   Fn,
   Macro,
   Plus,
   Minus,
   Times,
   Divide,
   Equal,
   Less,
   Great,
   Period,
   Comma,
   Newline,
   Str(String),
   Int(i64),
   Float(f64),
   Bool(bool),
   Ident(String),
}

pub enum LexerErrorKind {
   EndOfData,
   UnmatchedToken,
   IntegerOverflow
}

pub struct Lexer;

pub struct LexerError {
   kind: LexerErrorKind,
   desc: Option<String>
}

pub type LexerResult<T> = Result<T, LexerError>;

impl Lexer {
   #[inline]
   pub fn new() -> Lexer {
      Lexer
   }

   pub fn tokenize(&self, code: &str) -> LexerResult<Vec<Token>> {
      let mut result = vec!();

      let len = code.len();
      let mut idx = 0;
      let mut line = 1;
      loop {
         let (token, index, new_line) = match self.find_token(code, idx, len, line) {
            Ok(m) => m,
            Err(LexerError { kind: EndOfData, .. }) => break,
            Err(f) => return Err(f)
         };
         idx = index;
         line = new_line;
         result.push(token);
      }

      Ok(result)
   }

   #[inline]
   fn find_token(&self, code: &str, mut idx: uint, len: uint, mut line: uint) -> LexerResult<(Token, uint, uint)> {
      idx = self.skip_whitespace(code, idx);
      if idx < len {
         let val = match self.find_symbol_token(code, idx, line) {
            Some(result) => match result {
               Ok((token, index, new_line)) => {
                  idx = index;
                  line = new_line;
                  token
               }
               Err(f) => return Err(f)
            },
            None => match code.char_at(idx) {
               ch @ '0'..'9' => {
                  // number
                  // TODO: handle floats
                  let mut buffer = String::new();
                  buffer.push_char(ch);
                  for ch in code.slice_from(idx + 1).chars() {
                     if ch.is_digit() {
                        buffer.push_char(ch);
                     } else {
                        break;
                     }
                  }
                  idx += buffer.len();
                  Int(match from_str(buffer.as_slice()) {
                     Some(m) => m,
                     None => return Err(LexerError::new(IntegerOverflow, Some(format!("'{}' at line {} is too big", buffer, line))))
                  })
               }
               ch => {
                  // ident
                  let mut buffer = String::new();
                  buffer.push_char(ch);
                  idx += 1;
                  for ch in code.slice_from(idx).chars() {
                     if ch.is_whitespace() || ch == '"' || self.find_symbol_token(code, idx, line).is_some() {
                        break;
                     } else {
                        buffer.push_char(ch);
                        idx += 1;
                     }
                  }
                  match buffer.as_slice() {
                     "if" => If,
                     "loop" => Loop,
                     "break" => Break,
                     "continue" => Continue,
                     "let" => Let,
                     "fn" => Fn,
                     "macro" => Macro,
                     "true" => Bool(true),
                     "false" => Bool(false),
                     _ => Ident(buffer)
                  }
               }
            }
         };
         Ok((val, idx, line))
      } else {
         Err(LexerError::new(EndOfData, None))
      }
   }

   #[inline]
   fn find_symbol_token(&self, code: &str, mut idx: uint, mut line: uint) -> Option<LexerResult<(Token, uint, uint)>> {
      let val = match code.char_at(idx) {
         '"' => {
            idx += 1;
            let start = idx;
            loop {
               match code.slice_from(idx).find('"') {
                  Some(index) => {
                     let mut count = 0u;
                     for ch in code.slice(idx, index).chars().rev() {
                        if ch == '\\' {
                           count += 1;
                        } else {
                           break;
                        }
                     }
                     if count % 2 == 0 {
                        idx = index;
                        break;
                     } else {
                        idx = index + 1;
                     }
                  }
                  None => return Some(Err(LexerError::new(UnmatchedToken, Some(format!("mismatched '\"' starting at line {}", line)))))
               }
            }
            Str(code.slice(start, idx + 1).to_string())
         }
         '(' => LParen,
         ')' => RParen,
         '{' => LBrace,
         '}' => RBrace,
         '+' => Plus,
         '-' => Minus,
         '*' => Times,
         '/' => Divide,
         '=' => Equal,
         '<' => Less,
         '>' => Great,
         '.' => Period,
         ',' => Comma,
         '\n' => {
            line += 1;
            Newline
         }
         _ => return None
      };
      Some(Ok((val, idx + 1, line)))
   }

   #[inline]
   fn skip_whitespace(&self, code: &str, mut idx: uint) -> uint {
      for ch in code.slice_from(idx).chars() {
         if self.is_whitespace(ch) {
            idx += 1;
         } else {
            break;
         }
      }
      idx
   }

   #[inline]
   fn is_whitespace(&self, ch: char) -> bool {
      ch == '\t' || ch == ' '
   }
}

impl LexerError {
   pub fn new(kind: LexerErrorKind, desc: Option<String>) -> LexerError {
      LexerError {
         kind: kind,
         desc: desc
      }
   }
}

impl Show for LexerError {
   fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
      match self.desc {
         Some(ref desc) => fmt.write(desc.as_bytes()),
         None => Ok(())
      }
   }
}
