#### arataca89

# RBE - operador ?

Encadear resultados usando ```match``` pode ficar bem desorganizado; felizmente, o operador ```?``` pode ser usado para deixar as coisas bonitas novamente. ```?``` é usado no final de uma expressão que retorna um ```Result```, e é equivalente a uma expressão ```match```, onde o ramo ```Err(err)``` se expande para ``` return Err(From::from(err))```, e o ramo ```Ok(ok)``` se expande para uma expressão ```ok```.

```
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Função intermediária
    fn op_(x: f64, y: f64) -> MathResult {
        // se 'div()' falha, 'DivisionByZero' será retornado
        let ratio = div(x, y)?;

        // se 'ln()' falha, 'NonPositiveLogarithm' será retornado
        let ln = ln(ratio)?;
        
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logaritmo de número não positivo",
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

## Referências

[Rust by Example - ?](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)

---

arataca89@gmail.com

Última atualização: 20241224