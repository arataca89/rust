# Tratamento de erros em Rust

O tratamento de de erros em programação pode ser basicamente dividido em dois ramos: <tt>manipulação de exceções</tt> e <tt>retorno de valores</tt>. Rust opta por retornar valores. 

## Básico
Você pode pensar no tratamento de erros como sendo o uso de análise de casos para determinar se uma determinada tarefa ou cálculo foi bem-sucedido ou não. Como você verá, a chave para o tratamento ergonômico de erros é reduzir a quantidade de análise  de casos explícita que o programador precisa fazer enquanto mantém o código combinável.

Código combinável refere-se a um código onde os componentes ou módulos de software são organizados de maneira a serem facilmente combinados para criar novas funcionalidades. A ideia de código combinável defende a criação de pequenas unidades independentes que sejam tratadas como blocos de construção para sistemas maiores.

Manter o código combinável é importante porque, sem esse requisito, poderíamos entrar em pânico sempre que nos deparássemos com algo inesperado. (o pânico faz com que a tarefa atual seja interrompida e, na maioria dos casos, todo o programa é abortado.) Aqui está um exemplo:

```
// Adivinhe um número entre 1 e 10.
// Se seu número corresponde ao que temos em mente, retorne 'True'.
// Senão, retorne 'False'.
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
thread 'main' panicked at src/main.rs:7:9:
Número inválido: 11
```
Aqui está outro exemplo. Um programa que aceita um número inteiro como argumento, duplica-o e imprime-o:
```
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // erro 1
    let n: i32 = arg.parse().unwrap(); // erro 2
    println!("{}", 2 * n);
}

```
Se você fornecer zero argumentos a este programa (erro 1) ou se o primeiro argumento não for um número inteiro (erro 2), o programa entrará em pânico como no primeiro exemplo.

## Unwrapping
No exemplo anterior, afirmamos que o programa simplesmente entraria em pânico se atingisse uma das duas condições de erro, mas o programa não inclui uma chamada explícita a <tt>panic!</tt> como no primeiro exemplo. Isso ocorre porque panic! está embutida nas chamadas a <tt>unwrap</tt>.

“unwrap” algo em Rust é dizer: “Dê-me o resultado do cálculo e, se houver um erro, entre em pânico e pare o programa”. Seria melhor se mostrássemos o código de unwrap porque é muito simples, mas para fazer isso, primeiro precisaremos explorar os tipos <tt>Option</tt> e <tt>Result</tt>. Ambos possuem um método chamado unwrap definido neles.

## Option
O tipo <tt>Option</tt> é definido na [biblioteca padrão](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/option/enum.Option.html):
```
enum Option<T> {
    None,
    Some(T),
}
```
O tipo <tt>Option</tt> é uma forma de usar o sistema de tipos do Rust para expressar a possibilidade de ausência. Codificar a possibilidade de ausência no sistema de tipos é um conceito importante porque fará com que o compilador force o programador a lidar com essa ausência. Vamos dar uma olhada em um exemplo que tenta encontrar um caractere em uma string:
```
// Procura em 'haystack' pelo caractere Unicode 'needle' . Se for encontrado,
// retorna o byte offset do caractere. Senão, 'None' é retornado.
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
```
Observe que quando esta função encontra um caractere correspondente, ela não retorna apenas o <tt>offset</tt>. Em vez disso, retorna <tt>Some(offset)</tt>. <tt>Some</tt> é uma variante ou construtor de valor para o tipo <tt>Option</tt>. Você pode pensar nisso como uma função com o tipo ```fn<T>(value: T) -> Option<T>```. Da mesma forma, <tt>None</tt> também é um construtor de valor, exceto que não possui argumentos. Você pode pensar em <tt>None</tt> como uma função com o tipo <tt>fn<T>() -> Option<T></tt>.

### Referências
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#the-basics

https://www.howtogeek.com/devops/what-is-composable-code-and-how-can-you-create-it/#:~:text=Composable%20code%20describes%20classes%20and,more%20powerful%20higher-level%20constructs.
