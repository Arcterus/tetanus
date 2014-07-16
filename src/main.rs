mod lexer;

fn main() {
   let lexer = lexer::Lexer::new();
   match lexer.tokenize("fn main() { 3 + 4 }") {
      Ok(m) => println!("{}", m),
      Err(f) => fail!(f)
   }
}
