mod token;

fn main() {
  repl();
}

pub fn repl() {
  let mut buffer = String::new();
  let stdin = std::io::stdin();
  loop {
    stdin.read_line(&mut buffer).unwrap();
    let tokens = token::tokenize(&buffer);
    for tok in tokens {
      println!("{} : {:?}", &buffer[tok.start..tok.end], tok.ttype);
    }
    buffer = String::new();
  }
}
