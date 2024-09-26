# Linguagem Rust - Lifetimes

Lifetimes são um recurso da linguagem Rust que garantem que as referências sejam válidas pelo tempo que precisarmos delas. Toda referência em Rust tem um 'lifetime' ou 'tempo de vida', numa tradução livre. Lifetime refere-se ao escopo onde a referência é válida. Na maioria das vezes, as lifetimes são implícitas e inferidas, assim como na maioria das vezes, os tipos são inferidos. 

Devemos anotar tipos apenas quando múltiplos tipos forem possíveis e o Rust não puder inferir. De forma semelhante, devemos anotar lifetimes  quando as lifetimes das referências puderem ser relacionadas de algumas maneiras diferentes de modo que o Rust não possa inferir qual o lifetime correto.

O Rust exige que anotemos as relações usando parâmetros de lifetime genéricos para garantir que as referências atuais usadas em tempo de execução sejam definitivamente válidas. 

A sintaxe de lifetime não é um conceito que a maioria das outras linguagens de programação possui, então isso vai parecer estranho no início. Este artigo discute as maneiras mais comuns de utilização de lifetimes de modo que você possa se familiarizar com o conceito. 


[1. Evitando dangling references usando lifetimes](#1-Evitando-dangling-references-usando-lifetimes)

[2. O borrow checker](#2-O-borrow-checker)

[3. Lifetimes genéricos em funções](#3-Lifetimes-genéricos-em-funções)

[4. Sintaxe de lifetime](#4-Sintaxe-de-lifetime)

[5. Usando lifetime na assinatura de funções](#5-Usando-lifetime-na-assinatura-de-funções)

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

Abaixo temos a correção deste código para que ele não tenha dangling reference compile sem erros.

```
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+  
```

Agora o lifetime de x, que é 'b, é maior que o lifetime de r, que é 'a; e r pode referenciar x porque Rust sabe que a referência em r sempre será válida pois x, o objeto referenciado, tem um tempo de vida maior que a referência.

## 3. Lifetimes genéricos em funções

Vamos escrever uma função que recebe duas slices de string e retorna a maior entre elas.

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("A maior string é {result}"); // abcd
} 
```

Observe que queremos que a função receba slices de string, que são referências, porque não queremos que a função assuma a propriedade de seus parâmetros. 

Se tentarmos implementar a função conforme abaixo, este código não compilará:

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Ao tentar compilar, receberemos o seguinte erro:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```
O texto de ajuda revela que o tipo de retorno precisa de um parâmetro de tempo de vida genérico porque o Rust não consegue dizer se a referência que está sendo retornada se refere a x ou y. Na verdade, nós também não sabemos, porque o bloco if no corpo desta função retorna uma referência a x e o bloco else retorna uma referência a y! 

Quando definimos a função, não temos conhecimento dos valores concretos que serão passados para ela, então não sabemos se o if ou o else será executado. Também não sabemos as durações concretas das referências que serão passadas, então não podemos olhar para os escopos para determinar se a referência que retornaremos será sempre válida. 

O borrow checker (verificador de empréstimo) também não consegue determinar isso, porque ele não sabe como as durações de vida de x e y se relacionam com a duração de vida do valor de retorno. Para corrigir esse erro, adicionaremos parâmetros de duração de vida (lifetime) genéricos que definem o relacionamento entre as referências para que o verificador de empréstimo possa realizar sua análise. 

## 4. Sintaxe de lifetime

As anotações de tempo de vida não alteram quanto tempo qualquer uma das referências vive. Em vez disso, elas descrevem as relações dos tempos de vida de várias referências entre si, sem afetar os tempos de vida. Assim como as funções podem aceitar qualquer tipo quando a assinatura especifica um parâmetro de tipo genérico, as funções podem aceitar referências com qualquer tempo de vida especificando um parâmetro de tempo de vida genérico. 

As anotações de tempo de vida têm uma sintaxe um pouco incomum: os nomes dos parâmetros de tempo de vida devem começar com um apóstrofo (') e geralmente são todos minúsculos e muito curtos, como tipos genéricos. A maioria das pessoas usa o nome ```'a``` para a primeira anotação de tempo de vida. Colocamos anotações de parâmetros de tempo de vida após o ```&``` de uma referência, usando um espaço para separar a anotação do tipo da referência. 

Aqui estão alguns exemplos: uma referência a um i32 sem um parâmetro de tempo de vida, uma referência a um i32 que tem um parâmetro de tempo de vida chamado 'a, e uma referência mutável a um i32 que também tem o tempo de vida 'a. 

```
&i32        // uma referência sem o lifetime
&'a i32     // uma referência com um lifetime explícito
&'a mut i32 // uma referência mutável com um lifetime explícito
```

Uma anotação de tempo de vida por si só não tem muito significado porque as anotações são destinadas a dizer ao Rust como os parâmetros de tempo de vida genéricos de várias referências se relacionam entre si. Vamos examinar como as anotações de tempo de vida se relacionam entre si no contexto da função ```longest()```.

## 5. Usando lifetime na assinatura de funções 

Para usar anotações de tempo de vida em assinaturas de função, precisamos declarar os parâmetros de tempo de vida genéricos dentro de colchetes angulares entre o nome da função e a lista de parâmetros, da mesma forma que fazemos com os parâmetros de tipo genéricos. 

Desejamos que a assinatura expresse a seguinte restrição: a referência retornada será válida enquanto ambos os parâmetros forem válidos. Esta é a relação entre as durações dos parâmetros e o valor de retorno. Nomearemos a duração 'a e, em seguida, a adicionaremos a cada referência, como mostrado no código abaixo.

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
``` 

Agora a compilação deve ocorrer sem erros.

A assinatura da função agora diz a Rust que para algum tempo de vida 'a, a função recebe dois parâmetros, ambos são slices de string que vivem pelo menos tanto quanto o tempo de vida 'a. A assinatura da função também diz que a slice de string retornada da função viverá pelo menos tanto quanto o tempo de vida 'a. Na prática, isso significa que o tempo de vida da referência retornada pela função é o mesmo que o menor dos tempos de vida dos valores referenciados pelos argumentos da função. Essas relações são o que queremos que Rust use ao analisar esse código.

asd


---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-02-traits.html)

[https://www.naukri.com/code360/library/dangling-reference-in-compiler-design](https://www.naukri.com/code360/library/dangling-reference-in-compiler-design)

---

arataca89@gmail.com

Última atualização: 20240926
