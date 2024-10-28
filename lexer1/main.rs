/*******************************************************************
 * Aqui temos a implementação de um simples lexer que tokeniza
 * expressoes aritmeticas com as quatro operacoes basicas
 * (adicao, subtracao, multiplicacao e divisao) e numeros inteiros.
 * *****************************************************************/
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { chars: input.chars() }
    }
    
	pub fn tokenize(&mut self) -> Vec<Token> {
			let mut tokens = Vec::new();
			while let Some(token) = self.next_token() {
				tokens.push(token);
			}
			tokens			
	}

	fn next_token(&mut self) -> Option<Token> {
			let next_char = self.chars.next()?;
			match next_char {
				'+' => Some(Token::Plus),
				'-' => Some(Token::Minus),
				'*' => Some(Token::Multiply),
				'/' => Some(Token::Divide),
				'0'..='9' => {
					let mut number = next_char.to_digit(10)? as i32;
					while let Some(next_char) = self.chars.clone().next() {
						if let Some(digit) = next_char.to_digit(10) {
							number = number * 10 + digit as i32;
							self.chars.next();
						} else {
							break;
						}
					}
					Some(Token::Number(number))
				}
				_ => {
					println!("Token não reconhecido:[{}]", next_char);
					panic!();
				}
			}
	}
}

fn main() {
	let codigo = "1+22*333-4444/55555";
	let mut lexer = Lexer::new(codigo);
	println!("codigo: {codigo}");
	let tokens = lexer.tokenize();
	
	println!("=== TOKENS ==");
	for i in tokens {
		println!("{:?}", i);
	}	
}
