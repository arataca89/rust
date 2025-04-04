#### arataca89

# Linguagem Rust - enum

[Introdução](#introdução)

[enum ao estilo C](#enum-ao-estilo-c)

[enum pode ter métodos](#enum-pode-ter-métodos)

[enum com dados](#enum-com-dados)

---

## Introdução

```enum``` é um tipo definido pelo usuário que consiste de um container de nomes relacionados que podem conter valores. Por exemplo:

```rust
enum Operacao {
    Adicao,
    Subtracao,
    Multiplicacao,
    Divisao,
}

fn executar(operacao: Operacao, x: f64, y: f64) -> Result<f64, &'static str> {
    match operacao {
        Operacao::Adicao => Ok(x + y),
        Operacao::Subtracao => Ok(x - y),
        Operacao::Multiplicacao => Ok(x * y),
        Operacao::Divisao => {
            if y == 0.0 {
                return Err("Divisao por zero");
            } else {
                Ok(x / y)
            }
        }
    }
}

#[test]
fn somar(){
    assert_eq!(executar(Operacao::Adicao, 3.0, 5.0), Ok(8.0));
}

#[test]
fn subtrair(){
    assert_eq!(executar(Operacao::Subtracao, 3.0, 5.0), Ok(-2.0));
}

#[test]
fn multiplicar(){
    assert_eq!(executar(Operacao::Multiplicacao, 3.0, 5.0), Ok(15.0));
}

#[test]
fn dividir(){
    assert_eq!(executar(Operacao::Divisao, 3.0, 5.0), Ok(0.6));
    assert_eq!(executar(Operacao::Divisao, 3.0, 0.0), Err("Divisao por zero"));
}

fn main() {
    println!("Usando enum");
}
```

Neste exemplo temos uma ```enum``` chamada **Operacao** que possui 4 nomes: **Adicao**, **Subtracao**, **Multiplicacao** e **Divisao**. Estes nomes são chamados **variantes** da ```enum```. 

```rust
enum Operacao {
    Adicao,
    Subtracao,
    Multiplicacao,
    Divisao,
}
```

Note que as variantes estão relacionadas, elas são as quatro operações aritméticas básicas. A intenção é criar uma função que receba a operação e os operandos e retorne o resultado da operação aritmética, ou seja, uma simples calculadora.

Neste caso a função citada é ```executar()``` que deve receber a operação aritmética a ser executada, e os dois operandos. Ela deve retonar o resultado da operação ou uma string de erro.

```rust
fn executar(operacao: Operacao, x: f64, y: f64) -> Result<f64, &'static str> {
```

Note que ela retorna um tipo ```Result``` que também é uma enumeração.

```Result``` é uma enumeração da biblioteca padrão Rust que possui duas variantes.

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Neste caso, as variantes armazenam valores:

* **Ok** - que armazena um valor genérico do tipo **T**;
* **Err** - que armazena um valor genérico do tipo **E**.

**T** e **E** são parâmetros [genéricos](generics.md#arataca89). Isto significa que podemos usar ```Result``` com vários tipos diferentes, como aqui neste exemplo, onde **T** é do tipo **f64** e **E** é do tipo **&'static str**. A sintaxe **&'static** indica um tempo de vida ([lifetime](lifetimes.md#arataca89)) estático; o que significa que o valor, neste caso uma ```str```, deve ter o mesmo tempo de vida do programa, ou seja, deve durar por todo o programa. 

Em Rust, quando uma função pode retornar um valor de sucesso ou um erro, podemos retornar ```Result```. E aqui temos este tipo de caso, pois por ocasião da divisão, se o divisor for zero a operação não terá sucesso pois não podemos dividir um número por zero.

Dentro da função ```executar()``` temos um **match** que é uma instrução que faz o casamento de padrões. 

```rust
    match operacao {
        Operacao::Adicao => Ok(x + y),
        Operacao::Subtracao => Ok(x - y),
```

Aqui ela faz o casamento do parâmetro de função **operacao** que refere-se a operação aritmética desejada. Aqui podemos ver uma das aplicações de uma enumeração. Os nomes significativos ajudam a termos um código bem mais fácil de entender. Conforme a operação desejada, o **match** retorna um objeto ```Result``` que é uma variante **OK** com o valor da operação dentro dele. Esta é uma das aplicações do valor dentro de uma variante de uma enumeração.

Normalmente as enumerações são usadas junto com o casamento de padrões, como feito aqui com **match**.

No caso da operação de divisão observe que temos um código mais extenso.

```rust
Operacao::Divisao => {
            if y == 0.0 {
                return Err("Divisao por zero");
            } else {
                Ok(x / y)
            }
        }
```

Aqui, verificamos se o divisor é zero; se for, retornamos um ```Result``` que é uma variante **Err** com uma string literal indicando o erro; senão, retornamos um ```Result``` que é um **Ok** com o resultado da operação de divisão. Note o valor do uso da enumeração neste caso; conseguimos retornar o sucesso ou o erro, deixando tudo muito claro.

---

## enum ao estilo C

A enumeração ao estilo da linguagem C possui apenas os nomes, sem valores embutidos, como mostrado no exemplo anterior. Abaixo temos outro exemplo.

```rust
enum HttpStatus {
    Ok,
    NotModified,
    NotFound,
}

fn main(){

    // o status veio como resposta do servidor
    let status = HttpStatus::NotFound;

    match status {
        HttpStatus::Ok          => println!("Beleza"),
        HttpStatus::NotModified => println!("Não modificado"),
        HttpStatus::NotFound    => println!("Não encontrado"),
    }
}
```

Assim como na linguagem C, este tipo de enumeração pode ter valores inteiros associados; e a conversão para inteiro é permitida; conversão de inteiro para a variante da ```enum``` não é permitida.

```rust
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn main(){

    // o status veio como resposta do servidor
    let status = HttpStatus::NotFound;

    match status {
        HttpStatus::Ok          => println!("Beleza"),
        HttpStatus::NotModified => println!("Não modificado"),
        HttpStatus::NotFound    => println!("Não encontrado"),
    }

    println!("OK            : {}", HttpStatus::Ok as i32); // 200
    //println!("HttpStatus::Ok: {}", 200 as HttpStatus::Ok); // ERRO
}
```

Se inteiros não forem atribuídos, assim como na linguagem C, serão atribuidos automaticamente, iniciando com zero.

```rust
enum TokenType {
    Eof,
    Plus,
    Minus,
    Unknown,
}

fn main(){

    let t1 = TokenType::Eof;
    let t2 = TokenType::Plus;
    let t3 = TokenType::Minus;
    let t4 = TokenType::Unknown;

    println!("Eof   : {}", t1 as i32);// 0
    println!("Plus  : {}", t2 as i32);// 1
    println!("Minus : {}", t3 as i32);// 2
    println!("Unknow: {}", t4 as i32);// 3
}
```

Variantes que não recebem valor, após uma que recebe, seguirão a ordem crescente dos valores.

```rust
enum TokenType {
    Eof,
    Plus = 43,
    Minus = 45,
    Unknown,
}

fn main(){

    let t1 = TokenType::Eof;
    let t2 = TokenType::Plus;
    let t3 = TokenType::Minus;
    let t4 = TokenType::Unknown;

    println!("Eof   : {}", t1 as i32);// 0
    println!("Plus  : {}", t2 as i32);// 43
    println!("Minus : {}", t3 as i32);// 45
    println!("Unknow: {}", t4 as i32);// 46
}
```

---

## enum pode ter métodos

```rust
enum Token {
    Num,
    Plus,
    Minus,
    Unknown,
}

impl Token {
    pub fn print(&self){
        match self {
            Token::Num     => println!("Token{{Num}}"),
            Token::Plus    => println!("Token{{Plus}}"),
            Token::Minus   => println!("Token{{Minus}}"),
            Token::Unknown => println!("Token{{Unknown}}"),
        }
    }
}

fn main(){
    let tokens = vec![
        Token::Num,
        Token::Plus,
        Token::Minus,
        Token::Unknown,
        ];

    for t in tokens {
        t.print();
    }
}
```

Saída:

```
Token{Num}
Token{Plus}
Token{Minus}
Token{Unknown}
```

---

## enum com dados

```rust
// Importa o módulo 'fmt'
use std::fmt;

enum TokenType {
    Int,
    Float,
    Plus,
}

// Implementar 'Display' permite imprimir o tipo usando '{}'
impl fmt::Display for TokenType {
    // Esta trait requer o método 'fmt' com esta assinatura.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Int => write!(f, "Int"),
            TokenType::Float => write!(f, "Float"),
            TokenType::Plus => write!(f, "Plus"),
        }
    }
}

enum Token{
    Int(TokenType, i32),
    Float(TokenType, f64),
    Plus(TokenType, char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Int(t,v) => write!(f, "Token{{{}, {}}}", t, v),
            Token::Float(t,v) => write!(f, "Token{{{}, {}}}", t, v),
            Token::Plus(t,v) => write!(f, "Token{{{}, {}}}", t, v),
        }
    }
}

fn main(){
    let t1 = Token::Int(TokenType::Int, 13);
    let t2 = Token::Float(TokenType::Float, 3.1415);
    let t3 = Token::Plus(TokenType::Plus, '+');

    println!("t1: {}", t1);
    println!("t2: {}", t2);
    println!("t3: {}", t3);
}
```

Saída:

```
t1: Token{Int, 13}
t2: Token{Float, 3.1415}
t3: Token{Plus, +}
```

Rust tem três tipos de enumerações (```enum```) que correspondem aos três tipos de estruturas (```struct```):

* ```enum``` com variantes sem dados corresponde a ```struct``` do tipo unidade (```()```); são enumerações similares as da linguagem C;
* ```enum``` com variantes de tupla se parecem e funcionam como ```struct``` de tupla;
* ```enum``` com variantes do tipo ```struct``` têm chaves e campos nomeados.


Uma enumeração pode ter variantes de vários tipos diferentes.

```rust
#[derive(Debug)]
enum VariosTipos {
    SemDados,
    ComDados(i32),
    ComTupla(char, u8),
    ComStruct{
        R: i32,
        G: i32,
        B: i32,
    }
}

fn main(){
    let e1 = VariosTipos::SemDados;
    let e2 = VariosTipos::ComDados(13);
    let e3 = VariosTipos::ComTupla('+', 43);
    let e4 = VariosTipos::ComStruct{R:0, G:0, B: 255};

    println!("e1: {:?}", e1);
    println!("e2: {:?}", e2);
    println!("e3: {:?}", e3);
    println!("e4: {:?}", e4);
}
```

Saída:

```
e1: SemDados
e2: ComDados(13)
e3: ComTupla('+', 43)
e4: ComStruct { R: 0, G: 0, B: 255 }
```


## Referências

Livros:

Programação em Rust (Jim Blandy, Jason Orendorff e Leonora F.S. Tindall), 2a Edição, Editora Novatec, 2023, Capítulo 10.

Links:

[std::Result](https://doc.rust-lang.org/std/result/enum.Result.html)

---

arataca89@gmail.com

Última atualização: 20250106
