# Tratamento de erros em Rust

Como a maioria das linguagens de programação, Rust incentiva o programador a lidar com erros de uma maneira específica. Em termos gerais, o tratamento de erros é dividido em duas categorias amplas: exceções e valores de retorno. Rust opta por valores de retorno.

Nesta seção, pretendemos fornecer um tratamento abrangente de como lidar com erros em Rust. Mais do que isso, tentaremos introduzir o tratamento de erros uma parte de cada vez para que você saia com um conhecimento prático sólido de como tudo se encaixa.

Quando feito de forma ingênua, o tratamento de erros em Rust pode ser prolixo e irritante. Esta seção explorará esses obstáculos e demonstrará como usar a biblioteca padrão para tornar o tratamento de erros conciso e ergonômico.

* [Básico](#Básico)
	- [Explicando unwrap](#Explicando-unwrap)
	- [O tipo Option](#O-tipo-Option)
* ASD
	- qwe


## Básico

Você pode pensar no tratamento de erros como usar análise de casos para determinar se uma computação foi bem-sucedida ou não. Como você verá, a chave para o tratamento de erros ergonômico é reduzir a quantidade de análise de casos explícita que o programador tem que fazer, mantendo o código componível. Um código componível refere-se a característica de componibilidade do software. Componibilidade se refere à capacidade de diferentes componentes ou elementos serem combinados ou conectados de várias maneiras para criar sistemas ou estruturas maiores e mais complexas. 

Manter o código componível é importante, pois sem esse requisito, poderíamos ter que acionar ```panic``` sempre que nos deparássemos com algo inesperado. (```panic``` faz com que a tarefa atual seja desfeita e, na maioria dos casos, o programa inteiro seja abortado.) Aqui está um exemplo: 

```
// Adivinha um número entre 1 e 10.
// Se o número for adivinhado retorna 'true'; senão retorna 'false.
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Número inválido: {}", n);
    }
    n == 5
}

fn main() {
    guess(11);
}
```

Se você tentar executar este código, o programa irá travar com uma mensagem como esta: 

```
thread 'main' panicked at src/main.rs:5:9:
```

Aqui está outro exemplo que é um pouco menos artificial. Um programa que aceita um inteiro como argumento, dobra o valor do inteiro e o imprime. 

```
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // erro 1
    let n: i32 = arg.parse().unwrap(); // erro 2
    println!("{}", 2 * n);
}
```

Se você não fornecer argumentos (erro 1) ou se o primeiro argumento não for um inteiro (erro 2), o programa entrará em pânico, como no primeiro exemplo. 

Você pode pensar nesse estilo de tratamento de erros como um touro correndo em uma loja de porcelana. O touro chegará aonde quer, mas causará estrago no processo. 


## Explicando unwrap

No exemplo anterior, afirmamos que o programa simplesmente entraria em pânico se atingisse uma das duas condições de erro, no entanto, o programa não inclui uma chamada explícita a ```panic!``` como o primeiro exemplo. Isso ocorre porque o pânico está embutido nas chamadas a ```unwrap```. 

Na linguagem Rust, executar "unwrap" em algo significa dizer: "Dê-me o resultado do processamento, e se houver um erro, entre em pânico e pare o programa." Seria melhor se mostrássemos o código de ```unwrap```, pois é muito simples, mas para fazer isso, primeiro precisamos explorar os tipos ```Option``` e ```Result```. Estes dois tipos têm um método chamado ```unwrap()``` definido neles. 

## O tipo Option

asd



## Referências
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#the-basics

https://www.howtogeek.com/devops/what-is-composable-code-and-how-can-you-create-it/#:~:text=Composable%20code%20describes%20classes%20and,more%20powerful%20higher-level%20constructs.

---

arataca89@gmail.com

Última atualização: 20241010
