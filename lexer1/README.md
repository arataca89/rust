# Lexer para expressões aritméticas, versão 1

Este exercício consiste em escrever um analisador léxico para expressões aritméticas simples compostas pelas quatro operações básicas (+, -, *, e /) executadas com números inteiros.

### Token
O enum ```Token``` representa os diferentes tipos de tokens que podem ser reconhecidos pelo lexer.

```
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}
```

### Lexer

A estrutura ```Lexer``` representa o próprio lexer. Ela possui um campo de nome ```chars```, do tipo [std::str::Chars](https://doc.rust-lang.org/std/str/struct.Chars.html). , que é um iterador sobre os caracteres da string de entrada.

O tipo ```Chars``` é um iterador sobre os caracteres de uma fatia de string ([str](https://doc.rust-lang.org/std/primitive.str.html)). Um tipo ```Chars``` é criado pelo método [chars()](https://doc.rust-lang.org/std/primitive.str.html#method.chars). 
 
```
pub struct Lexer<'a> {
    chars: Chars<'a>,
}
```

A lifetime ```'a``` indica que o o iterador ```chars``` deverá ter o mesmo tempo de vida do objeto ```Lexer```

O tipo ```Lexer``` terá três métodos:

* ```new()``` que cria uma nova instância de ```Lexer```;
* ```tokenize()``` que fará a varredura propriamente dita; e
* ```next_token()``` que retornará o próximo token.

### new()

```
    pub fn new(input: &'a str) -> Self {
        Lexer { chars: input.chars() }
    }
```

Observe que ```new()``` recebe uma slice de string com o mesmo lifetime do objeto ```Lexer``` (```'a```). O método ```chars()``` é usado para retornar o iterador do tipo ```Chars```.

### tokenize()

```
	pub fn tokenize(&mut self) -> Vec<Token> {
			let mut tokens = Vec::new();
			while let Some(token) = self.next_token() {
				tokens.push(token);
			}
			tokens			
	}
```

### next_token()

```
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
```

Inicialmente, note que ```next_token()``` não faz parte da interface pública do nosso lexer, ele não está setado para ```pub``` como os outros dois métodos. O usuário do nosso lexer precisará somente criar o lexer com ```new()``` e realizar a varredura com ```tokenize()```. o método ```next_token()``` é uma ferramenta interna do nosso lexer.

Inicialmente o método pega o próximo caractere e o armazena em ```next_char```. Para isso foi utilizado o método [std::Iterator::next()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next), este método avança o iterador e retorna o próximo valor. O método ```next()``` retorna uma ```Option``` e esta virá com o valor ```None``` ao fim da iteração. Enquanto houver valores ela retornará ```Some```.

Depois é feito o processamento deste caractere com ```match```. O reconhecimento dos tokens dos operadores é direto.

```
'+' => Some(Token::Plus),
'-' => Some(Token::Minus),
'*' => Some(Token::Multiply),
'/' => Some(Token::Divide),
```

O reconhecimento dos números inteiros cabe explicação:

```
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
```

Inicialmente usa-se  a sintaxe ```'0'..='9'``` que significa o casamento de dígitos de zero a nove. Em Rust, a palavra-chave ```match``` pode ser usada para combinar um intervalo inclusivo de valores, como ```13..=19```, por exemplo. Aqui estão alguns exemplos de como usar ```match```:

* Combinar um único valor: ```match number { 1 => println!("Um!") }```
* Combinar vários valores: ```2 | 3 | 5 | 7 | 11 => println!```
* Combinar um intervalo inclusivo: ```13..=19 => println!("Um adolescente")```
* Lidar com o resto dos casos: ```_ => println!("qualquer outro valor")``` 
 
Depois, cria-se a variável mutável ```number```, a qual receberá o retorno do método [to_digit()](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit). Este método recebe um ```char``` e o converte para um dígito na base numérica passada como argumento. Neste caso usamos a base 10.

```
let mut number = next_char.to_digit(10)? as i32;
```

O método ```to_digit()``` retorna uma ```Option<u32>```, então o operador ```?``` (interrogação) foi usado. O operador ```?``` é usado para desempacotar uma ```Option```. Se ```x``` é uma ```Option```, avaliar ```x?``` retorna o valor subjacente se ```x``` for ```Some```. Se ```x``` for ```None```, a função que está sendo executada é encerrada e ```None``` é retornado. 

Após o operador ```?````, observe que o dígito retornado é convertido para ```i32```.

Depois, temos um loop while que irá iterar enquanto houver caracteres no objeto Lexer.

```
while let Some(next_char) = self.chars.clone().next() {
```

Note a necessidade de clonar o iterador para que a propriedade do retorno de ```next()``` seja transferido para ```Some(next_char)```.

Dentro do loop while o número vai sendo montado e o iterador movido para a próxima posição. Se o caractere não é um dígito, ```to_digit()``` retornará ```None```, então o break fará o controle sair do loop ...

```
if let Some(digit) = next_char.to_digit(10) {
	number = number * 10 + digit as i32;
	self.chars.next();
} else {
	break;
}
```

... e o número será retornado.

```
Some(Token::Number(number))
```
					
Qualquer outro caractere não será reconhecido pelo Lexer.

```
_ => {
	println!("Token não reconhecido:[{}]", next_char);
	panic!();
```
					
---

## main

Na função ```main()``` podemos testar nosso Lexer.

```
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
```

Esta função ```main()``` produzirá a saída:

```
codigo: 1+22*333-4444/55555
=== TOKENS ==
Number(1)
Plus
Number(22)
Multiply
Number(333)
Minus
Number(4444)
Divide
Number(55555)
```

Vamos inserir um caractere não reconhecido pelo Lexer:

```
fn main() {
	let codigo = "1+22*333-44#44/55555";
	let mut lexer = Lexer::new(codigo);
	println!("codigo: {codigo}");
	let tokens = lexer.tokenize();
	
	println!("=== TOKENS ==");
	for i in tokens {
		println!("{:?}", i);
	}	
}
```

Agora ao compilar e rodar teremos:

```
codigo: 1+22*333-44#44/55555
Token não reconhecido:[#]
thread 'main' panicked at src/main.rs:55:21:
explicit panic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\lexer1.exe` (exit code: 101)
```

A princípio o Lexer está funcionando corretamente.						

---

## Referências

[Simple Lexer in Rust](https://dev.to/kopium/simple-lexer-in-rust-b3k)

[std::str::Chars](https://doc.rust-lang.org/std/str/struct.Chars.html)

[str](https://doc.rust-lang.org/std/primitive.str.html)

[std::Iterator::next()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)

[to_digit()](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit)

---

arataca89@gmail.com

Última atualização: 20241028
