# Rust através de exemplos

[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) (RBE ou Rust através de exemplos, numa tradução livre) é uma coleção de exemplos executáveis que ilustram vários conceitos e bibliotecas padrão do Rust. Este artigo aborda alguns tópicos do RBE .

* [Option](#Option)
* [Result](#Result)
* [Operador ?](#Operador-)
* [panic!](#panic-)
---

## Option

Às vezes, é desejável capturar a falha de algumas partes de um programa em vez de chamar ```panic!```; isso pode ser feito usando o enum ```Option```.

O enum ```Option<T>``` possui duas variantes:

* ```None```, para indicar falha ou falta de valor; e
* ```Some(valor)```, uma estrutura de tupla que envolve um ```valor``` com tipo ```T```.

```
// rbe_option

// Executa uma divisão e não chama 'panic!' em caso de erro
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // A falha é representada como 'None'
        None
    } else {
        // O resultado é encapsulado em 'Some'
        Some(dividend / divisor)
    }
}

// Esta função trata uma divisão que pode não ser bem sucedida
fn try_division(dividend: i32, divisor: i32) {
    // 'Option', como qualquer enum, pode ser processada via match
    match checked_division(dividend, divisor) {
        None => println!("{} / {} ERRO!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Vincular 'None' a uma variável necessita anotar o tipo da variável
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Usar 'unwrap()' em um 'Some' extrairá o valor dentro do 'Some'
    println!("{:?} é um 'Some' e tem dentro o valor {:?}", optional_float, optional_float.unwrap());

    // Usar 'unwrap' em um 'None' causará 'panic!'
    println!("{:?} é um 'None' e tem dentro o valor {:?}", none, none.unwrap());
}
```

Execução:

```
4 / 2 = 2
1 / 0 ERRO!
Some(0.0) é um 'Some' e tem dentro o valor 0.0
thread 'main' panicked at src/main.rs:39:71:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_option.exe` (exit code: 101)
```

## Result

Vimos que o enum ```Option``` pode ser usado como um valor de retorno de funções que podem falhar, onde ```None``` pode ser retornado para indicar falha. No entanto, às vezes é importante expressar por que uma operação falhou. Para fazer isso, temos o enum ```Result```.

O enum ```Result<T, E>``` tem duas variantes: 

* ```Ok(value)```, que indica que a operação foi bem-sucedida e encapsula o valor retornado pela operação. (```value``` tem o tipo ```T```)
* ```Err(why)```, que indica que a operação falhou e encapsula ```why```, que (esperançosamente) explica a causa da falha. (```why``` tem o tipo ```E```)

```
// rbe_result

mod checked {
    // Erros matemáticos que queremos capturar
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // Esta operação falharia, em vez disso, vamos retornar o motivo da
            // falha encapsulada em 'Err'
            Err(MathError::DivisionByZero)
        } else {
            // Esta operação é válida, retorna o resultado dentro de um 'Ok'
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// 'op(x, y)' === 'sqrt(ln(x / y))'
fn op(x: f64, y: f64) -> f64 {
    // Uma pirâmide de 'match' com três níveis
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Isso irá falhar?
   println!("{}", op(1.0, 10.0));
}
```

Execução:

```
thread 'main' panicked at src/main.rs:50:29:
NegativeSquareRoot
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_result.exe` (exit code: 101)
```

## Operador ?

Encadear resultados usando ```match``` pode ficar confuso. O operador ```?``` pode ser usado para deixar o código mais fácil de entender. O operador ```?``` é usado no final de uma expressão que retorna um ```Result```, e é equivalente a uma expressão ```match```, onde o ramo ```Err(err)``` se expande para ```return Err(From::from(err))```, e o ramo ```Ok(ok)``` se expande para uma expressão ```ok```.

```
// rbe_opearador_interrogacao

mod checked {
    // Erros matemáticos que queremos capturar
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // Esta operação falharia, em vez disso, vamos retornar o motivo da
            // falha encapsulada em 'Err'
            Err(MathError::DivisionByZero)
        } else {
            // Esta operação é válida, retorna o resultado dentro de um 'Ok'
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Função intermediária
    fn op_(x: f64, y: f64) -> MathResult {
        // se 'div()' falhar, 'DivisionByZero' será retornado.
        let ratio = div(x, y)?;

        // se 'ln()' falhar, 'NonPositiveLogarithm' será retornado.
        let ln = ln(ratio)?;

        sqrt(ln)
    }


    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logarítmo de número negativo",
                MathError::DivisionByZero
                    => "divisão por zero",
                MathError::NegativeSquareRoot
                    => "raiz quadrada de número negativo",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

Execução:

```
thread 'main' panicked at src/main.rs:55:25:
raiz quadrada de número negativo
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_result.exe` (exit code: 101)
```

## panic!
 
asd



## Referências

[Rust by Example (RBE)](https://doc.rust-lang.org/rust-by-example/index.html)

[RBE - Option](https://doc.rust-lang.org/rust-by-example/std/option.html)

[RBE - Result](https://doc.rust-lang.org/rust-by-example/std/result.html)

[RBE - Operador ?](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)

[RBE - panic!](https://doc.rust-lang.org/rust-by-example/std/panic.html)


---

arataca89@gmail.com

Última atualização: 20241021
