# Linguagem Rust - Lifetimes

Lifetimes são um recurso da linguagem Rust que garantem que as referências sejam válidas pelo tempo que precisarmos delas. Toda referência em Rust tem um 'lifetime' ou 'tempo de vida', numa tradução livre. Lifetime refere-se ao escopo onde a referência é válida. Na maioria das vezes, as lifetimes são implícitas e inferidas, assim como na maioria das vezes, os tipos são inferidos. 

Devemos anotar tipos apenas quando múltiplos tipos forem possíveis e o Rust não puder inferir. De forma semelhante, devemos anotar lifetimes  quando as lifetimes das referências puderem ser relacionadas de algumas maneiras diferentes de modo que o Rust não possa inferir qual o lifetime correto.

O Rust exige que anotemos as relações usando parâmetros de lifetime genéricos para garantir que as referências atuais usadas em tempo de execução sejam definitivamente válidas. 

A sintaxe de lifetime não é um conceito que a maioria das outras linguagens de programação possui, então isso vai parecer estranho no início. Este artigo discute as maneiras mais comuns de utilização de lifetimes de modo que você possa se familiarizar com o conceito. 


[1. Evitando dangling references usando lifetimes](#1-Evitando-dangling-references-usando-lifetimes)

[2. O borrow checker](#2-O-borrow-checker)

---

## 1. Evitando dangling references usando lifetimes

'dangling reference' é um termo que refere-se a uma situação onde um ponteiro ou referência aponta para um local de memória que foi desalocado ou liberado. Isso pode acontecer quando um objeto é excluído ou sai do escopo, mas a referência a ele ainda existe e é acessada posteriormente. 

O objetivo principal do recurso de lifetime do Rust é evitar 'dangling references', que fazem com que um programa faça referência a dados diferentes dos dados originais aos quais ele se referia antes.

Considere o programa abaixo.

```
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}
```

O escopo externo declara uma variável chamada 'r' sem valor inicial, e o escopo interno declara uma variável chamada 'x' com o valor inicial de 5.

Dentro do escopo interno, tentamos definir o valor de 'r' como uma referência a 'x'. Então o escopo interno termina, e tentamos imprimir o valor em 'r'.

Este código não irá compilar porque o valor ao qual 'r' está referenciando saiu do escopo antes de tentarmos usá-lo, ao sair do escopo foi destruído. Aqui está a mensagem de erro recebida ao tentarmos compilar este código: 

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
  |                  --- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

A mensagem de erro diz que a variável 'x' "não vive o suficiente". O motivo é que 'x' estará fora do escopo quando o escopo interno terminar na linha 7. Mas 'r' ainda é válido para o escopo externo; porque seu escopo é maior, dizemos que ele "vive mais tempo".

Se Rust permitisse que esse código funcionasse, 'r' estaria referenciando memória que foi desalocada quando 'x' saiu do escopo, e qualquer coisa que tentássemos fazer com 'r' não funcionaria corretamente. Então, como o Rust determina que esse código é inválido? Ele usa o famoso 'borrow checker'.


## 2. O borrow checker

O compilador Rust possui um recurso chamado 'borrow checker' (verificador de empréstimo) que compara os escopos para determinar se todos os empréstimos são válidos.

Abaixo vemos o código anterior com anotações mostrando os tempos de vida (lifetimes) das variáveis.

```
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

O tempo e vida (lifetime) de ``` r ``` é anotado como ``` 'a ``` e o de ``` x ``` é anotado como ``` 'b ```. Observe que o escopo de 'b é bem menor que o escopo de 'a. Rust compara estes escopos em tempo de compilação e vê que r tem o escopo 'a mas refere-se a uma posição de memória que tem escopo 'b e o programa não é aceito porque 'b é menor que 'a. O objeto referenciado, no caso x, não vive tanto quanto a própria referência, no caso r. 

asd


---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-02-traits.html)

[https://www.naukri.com/code360/library/dangling-reference-in-compiler-design](https://www.naukri.com/code360/library/dangling-reference-in-compiler-design)

---

arataca89@gmail.com

Última atualização: 20240918
