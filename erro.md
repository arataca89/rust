# Tratamento de erros em Rust

O tratamento de de erros em programação pode ser basicamente dividido em dois ramos: <tt>manipulação de exceções</tt> e <tt>retorno de valores</tt>. Rust opta por retornar valores. 

## Básico
Você pode pensar no tratamento de erros como sendo o uso de análise de casos para determinar se uma determinada tarefa ou cálculo foi bem-sucedido ou não. Como você verá, a chave para o tratamento ergonômico de erros é reduzir a quantidade de análise  de casos explícita que o programador precisa fazer enquanto mantém o código combinável.

### Referências
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#the-basics
