# Tratamento de erros em Rust

O tratamento de de erros em programação pode ser basicamente dividido em dois ramos: <tt>manipulação de exceções</tt> e <tt>retorno de valores</tt>. Rust opta por retornar valores. 

## Básico
Você pode pensar no tratamento de erros como sendo o uso de análise de casos para determinar se uma determinada tarefa ou cálculo foi bem-sucedido ou não. Como você verá, a chave para o tratamento ergonômico de erros é reduzir a quantidade de análise  de casos explícita que o programador precisa fazer enquanto mantém o código combinável.

Código combinável refere-se a um código onde os componentes ou módulos de software são organizados de maneira a serem facilmente combinados para criar novas funcionalidades. A ideia de código combinável defende a criação de pequenas unidades independentes que sejam tratadas como blocos de construção para sistemas maiores.

Manter o código combinável é importante porque, sem esse requisito, poderíamos entrar em pânico sempre que nos deparássemos com algo inesperado. (o pânico faz com que a tarefa atual seja interrompida e, na maioria dos casos, todo o programa é abortado.) Aqui está um exemplo:

```
// Guess a number between 1 and 10.
// If it matches the number we had in mind, return `true`. Else, return `false`.
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}

fn main() {
    guess(11);
}
```

### Referências
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#the-basics

https://www.howtogeek.com/devops/what-is-composable-code-and-how-can-you-create-it/#:~:text=Composable%20code%20describes%20classes%20and,more%20powerful%20higher-level%20constructs.
