# Linguagem Rust - Lifetimes

Lifetime(tempo de vida) é um dos recursos da linguagem Rust que garantem que as referências sejam válidas pelo tempo que precisarmos delas. Toda referência em Rust tem um lifetime. Lifetime refere-se ao escopo onde a referência é válida. Na maioria das vezes, os lifetimes são implícitos e inferidos, assim como na maioria das vezes, os tipos são inferidos. 

Devemos anotar tipos apenas quando múltiplos tipos forem possíveis e o Rust não puder inferir. De forma semelhante, devemos anotar lifetimes  quando os lifetimes das referências puderem ser relacionadas de algumas maneiras diferentes de modo que o Rust não possa inferir qual o lifetime correto.

O Rust exige que anotemos as relações usando parâmetros de lifetime genéricos para garantir que as referências atuais usadas em tempo de execução sejam definitivamente válidas. 

A sintaxe de lifetime não é um conceito que a maioria das outras linguagens de programação possui, então isso vai parecer estranho no início. Este artigo discute as maneiras mais comuns de utilização de lifetimes de modo que você possa se familiarizar com o conceito. 


[1. Evitando dangling references usando lifetimes](#1-Evitando-dangling-references-usando-lifetimes)

[2. O borrow checker](#2-O-borrow-checker)

[3. Lifetimes genéricos em funções](#3-Lifetimes-genéricos-em-funções)

[4. Sintaxe de lifetime](#4-Sintaxe-de-lifetime)

[5. Usando lifetime na assinatura de funções](#5-Usando-lifetime-na-assinatura-de-funções)

[6. Pensando em termos de lifetime](#6-Pensando-em-termos-de-lifetime)

[7. Lifetime em structs](#7-Lifetime-em-structs)

[8. Regras para omitir o lifetime](#8-Regras-para-omitir-o-lifetime)

[9. Lifetimes em métodos](#9-Lifetimes-em-métodos)

[10. O lifetime static](#10-O-lifetime-static)

[11. Genéricos, traits e lifetimes juntos](#11-Genéricos-traits-e-lifetimes-juntos)

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

Abaixo temos a correção deste código para que ele não tenha dangling reference e compile sem erros.

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

Agora o lifetime de x, que é 'b, é maior que o lifetime de r, que é 'a; e r pode referenciar x porque Rust sabe que a referência em r sempre será válida pois x, o objeto referenciado, tem um lifetime maior que a referência.

## 3. Lifetimes genéricos em funções

Vamos escrever uma função que recebe duas slices de string e retorna a maior entre elas.

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2); // as_str() converte String em &str
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
O texto de ajuda revela que o tipo de retorno precisa de um parâmetro de lifetime genérico porque o Rust não consegue dizer se a referência que está sendo retornada se refere a x ou y. Na verdade, nós também não sabemos, porque o bloco if no corpo desta função retorna uma referência a x e o bloco else retorna uma referência a y! 

Quando definimos a função, não temos conhecimento dos valores concretos que serão passados para ela, então não sabemos se o if ou o else será executado. Também não sabemos as durações concretas das referências que serão passadas, então não podemos olhar para os escopos para determinar se a referência que retornaremos será sempre válida. 

O borrow checker (verificador de empréstimo) também não consegue determinar isso, porque ele não sabe como as durações de vida de x e y se relacionam com a duração de vida do valor de retorno. Para corrigir esse erro, adicionaremos parâmetros de duração de vida (lifetime) genéricos que definem o relacionamento entre as referências para que o verificador de empréstimo possa realizar sua análise. 

## 4. Sintaxe de lifetime

As anotações de lifetime não alteram quanto tempo qualquer uma das referências vive. Em vez disso, elas descrevem as relações dos tempos de vida de várias referências entre si, sem afetar os tempos de vida. Assim como as funções podem aceitar qualquer tipo quando a assinatura especifica um parâmetro de tipo genérico, as funções podem aceitar referências com qualquer lifetime especificando um parâmetro de lifetime genérico. 

As anotações de lifetime têm uma sintaxe um pouco incomum: os nomes dos parâmetros de lifetime devem começar com um apóstrofo ```( ' )``` e geralmente são todos minúsculos e muito curtos, como tipos genéricos. A maioria das pessoas usa o nome ```'a``` para a primeira anotação de lifetime. Colocamos anotações de parâmetros de lifetime após o ```&``` de uma referência, usando um espaço para separar a anotação do tipo da referência. 

Aqui estão alguns exemplos: uma referência a um i32 sem um parâmetro de lifetime, uma referência a um i32 que tem um parâmetro de lifetime chamado 'a, e uma referência mutável a um i32 que também tem o lifetime 'a. 

```
&i32        // uma referência sem o lifetime
&'a i32     // uma referência com um lifetime explícito
&'a mut i32 // uma referência mutável com um lifetime explícito
```

Uma anotação de lifetime por si só não tem muito significado porque as anotações são destinadas a dizer ao Rust como os parâmetros de lifetime genéricos de várias referências se relacionam entre si. Vamos examinar como as anotações de lifetime se relacionam entre si no contexto da função ```longest()```.

## 5. Usando lifetime na assinatura de funções 

Para usar anotações de lifetime em assinaturas de função, precisamos declarar os parâmetros de lifetime genéricos dentro de colchetes angulares entre o nome da função e a lista de parâmetros, da mesma forma que fazemos com os parâmetros de tipo genéricos. 

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

A assinatura da função agora diz a Rust que para algum lifetime 'a, a função recebe dois parâmetros, ambos são slices de string que vivem pelo menos tanto quanto o lifetime 'a. A assinatura da função também diz que a slice de string retornada da função viverá pelo menos tanto quanto o lifetime 'a. Na prática, isso significa que o lifetime da referência retornada pela função é o mesmo que o menor dos tempos de vida dos valores referenciados pelos argumentos da função. Essas relações são o que queremos que Rust use ao analisar esse código.

'a receberá o menor lifetime concreto entre os lifetimes de x e y. A referência retornada será válida enquanto o menor lifetime entre os de x e y for vigente. 

Por exemplo, considere o seguinte uso da função ```longest()```.

```
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
```

Neste exemplo, string1 é válida até o final do escopo externo, string2 é válida até o final do escopo interno e result referencia algo que é válido até o final do escopo interno. Execute este código e você verá que o borrow checker aprova; ele irá compilar e imprimir ```The longest string is long string is long```. 

Em seguida, vamos tentar um exemplo que mostra que a vida útil da referência em result deve ser a menor vida útil dos dois argumentos. Moveremos a declaração da variável result para fora do escopo interno, mas deixaremos a atribuição do valor à variável result dentro do escopo com string2. Então, moveremos o println! que usa result para fora do escopo interno, depois que o escopo interno tiver terminado. O código não compilará. 

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
```

Abaixo temos a mensagem de erro emitida ao tentarmos compilar este código.

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
5 |         let string2 = String::from("xyz");
  |             ------- binding `string2` declared here
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {result}");
  |                                     -------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

O erro mostra que para result ser válida para a instrução println!, string2 precisaria ser válida até o final do escopo externo. Rust sabe disso porque anotamos as durações dos parâmetros da função e dos valores de retorno usando o mesmo parâmetro de lifetime 'a'. 

## 6. Pensando em termos de lifetime

A maneira como você precisa especificar parâmetros de lifetime depende do que sua função está fazendo. Por exemplo, se alterássemos a implementação da função ```longest()``` para sempre retornar o primeiro parâmetro em vez da fatia de string mais longa, não precisaríamos especificar um lifetime no parâmetro y. O código a seguir será compilado: 

```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Especificamos um parâmetro de lifetime 'a para o parâmetro x e o tipo de retorno, mas não para o parâmetro y, porque o lifetime de y não tem nenhuma relação com o lifetime de x ou o valor de retorno. 

Ao retornar uma referência de uma função, o parâmetro de lifetime para o tipo de retorno precisa corresponder ao parâmetro de lifetime para um dos parâmetros. Se a referência retornada não se referir a um dos parâmetros, ela deve se referir a um valor criado dentro dessa função. No entanto, isso seria uma referência pendente (dangling reference) porque o valor sairá do escopo no final da função. Considere esta tentativa de implementação da função ```longest()``` que não compilará:

```
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Aqui, mesmo que tenhamos especificado um parâmetro de lifetime 'a para o tipo de retorno, esta implementação falhará na compilação porque o lifetime do valor de retorno não está relacionado ao lifetime dos parâmetros de forma alguma. Aqui está a mensagem de erro que recebemos: 

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

O problema é que result sai do escopo e é destruído no final da função. Também estamos tentando retornar uma referência ao resultado da função. Não há como especificar parâmetros de lifetime que alterariam a referência pendente, e o Rust não nos permite criar uma referência pendente. Nesse caso, a melhor solução seria retornar um tipo de dados próprio em vez de uma referência, para que a função chamadora seja então responsável por limpar o valor. 

Em última análise, a sintaxe de lifetime é sobre conectar os tempos de vida de vários parâmetros e valores de retorno de funções. Uma vez conectados, o Rust tem informações suficientes para permitir operações seguras de memória e proibir operações que criariam ponteiros pendurados ou violassem a segurança de memória de outra forma. 

## 7. Lifetime em structs

Até agora, as structs que definimos armazenaram tipos com propriedade. Podemos definir structs para armazenar referências, mas nesse caso, precisaríamos adicionar uma anotação de lifetime a cada referência na definição da struct. Abaixo temos uma struct chamada ImportantExcerpt que armazena uma fatia de string. 

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

Esta estrutura possui o único campo (part) que contém uma fatia de string, que é uma referência. Como acontece com tipos de dados genéricos, declaramos o nome do parâmetro de lifetime genérico dentro de colchetes angulares após o nome da estrutura para que possamos usar o parâmetro de lifetime no corpo da estrutura. Essa anotação significa que uma instância de ImportantExcerpt não pode sobreviver mais que a referência que ela contém em seu campo part. 

A função ```main()``` aqui cria uma instância da estrutura ImportantExcerpt que contém uma referência à primeira frase da String de propriedade da variável novel. Os dados em novel existem antes da instância ImportantExcerpt ser criada. Além disso, novel não sai do escopo até depois de ImportantExcerpt sair do escopo, então a referência na instância ImportantExcerpt é válida. 

## 8. Regras para omitir o lifetime

Vimos acima que cada referência tem um lifetime e que você precisa especificar parâmetros de lifetime para funções ou structs que usam referências. No entanto, já vimos uma função que compilou sem anotações de lifetime. Observe no código abaixo.

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

O motivo pelo qual essa função compila sem anotações de lifetime é histórico: nas primeiras versões (pré-1.0) do Rust, esse código não teria compilado porque cada referência precisava de um lifetime explícito. Naquela época, a assinatura da função teria sido escrita assim: 

```
fn first_word<'a>(s: &'a str) -> &'a str {
```

Depois de escrever muito código, a equipe do Rust descobriu que os programadores estavam inserindo as mesmas anotações de lifetime repetidamente em situações específicas. Essas situações eram previsíveis e seguiam alguns padrões determinísticos. Os desenvolvedores programaram esses padrões no código do compilador para que o verificador de empréstimo pudesse inferir os tempos de vida nessas situações e não precisasse de anotações explícitas. 

Esta parte da história do Rust é relevante porque é possível que mais padrões determinísticos surjam e sejam adicionados ao compilador. No futuro, ainda menos anotações de lifetime podem ser necessárias. 

Estes padrões programados na análise de referências do Rust são chamados de  ```regras de elipse de lifetime``` (lifetime elision rules). Essas regras não são para os programadores seguirem; são um conjunto de casos específicos que o compilador considerará, e se seu código se encaixar nesses casos, você não precisará escrever os tempos de vida explicitamente. 

As regras de elipse não fornecem inferência completa. Se ainda houver ambiguidade quanto às durações de vida das referências depois que o Rust aplicar as regras, o compilador não adivinhará qual deve ser a duração de vida das referências restantes. Em vez de adivinhar, o compilador fornecerá um erro que você pode resolver adicionando as anotações de lifetime. 

Os lifetimes em parâmetros de função ou método são chamados de ```input lifetimes``` (tempos de vida de entrada), e os lifetimes em valores de retorno são chamadas de ```output lifetimes``` (tempos de vida de saída). 

O compilador usa três regras para descobrir os lifetimes das referências quando não há anotações explícitas. A primeira regra se aplica a lifetimes de entrada, e a segunda e a terceira regras se aplicam a lifetimes de saída. Se o compilador chegar ao final das três regras e ainda houver referências para as quais ele não conseguir descobrir as durações de vida, o compilador interromperá com um erro. Essas regras se aplicam a definições de ```fn```, bem como a blocos ```impl```. 

A primeira regra é que o compilador atribui um parâmetro de lifetime a cada parâmetro que é uma referência. Em outras palavras, uma função com um parâmetro recebe um parâmetro de lifetime: 

```
fn foo<'a>(x: &'a i32) {
```

Uma função com dois parâmetros recebe dois parâmetros de lifetime separados:

```
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
```

e assim por diante. 

A segunda regra é que, se houver exatamente um parâmetro de lifetime de entrada, esse lifetime é atribuído a todos os parâmetros de lifetime de saída.

```
fn foo<'a>(x: &'a i32) -> &'a i32 {
```

A terceira regra é que, se houver múltiplos parâmetros de lifetime de entrada, mas um deles for ```&self``` ou ```&mut self``` porque este é um método, o lifetime de ```self``` é atribuído a todos os parâmetros de lifetime de saída. Esta terceira regra torna os métodos muito mais agradáveis de ler e escrever porque são necessários menos símbolos. 

Vamos fingir que somos o compilador. Vamos aplicar essas regras para descobrir as durações das referências na assinatura da função ```first_word()```. A assinatura começa sem nenhuma duração associada às referências: 

```
fn first_word(s: &str) -> &str {
```

Então o compilador aplica a primeira regra, que especifica que cada parâmetro recebe sua própria duração. Vamos chamá-la de 'a como de costume, então agora a assinatura será: 

```
fn first_word<'a>(s: &'a str) -> &str {
```

A segunda regra se aplica porque existe exatamente um lifetime de entrada. A segunda regra especifica que o lifetime do único parâmetro de entrada é atribuído ao lifetime de saída, então a assinatura agora será:

```
fn first_word<'a>(s: &'a str) -> &'a str {
```

Agora, todas as referências nesta assinatura de função têm tempos de vida, e o compilador pode continuar sua análise sem precisar que o programador anote os tempos de vida nesta assinatura de função. 

Vamos ver outro exemplo, desta vez usando a função ```longest()``` que não tinha parâmetros de lifetime quando começamos a trabalhar com ela.

```
fn longest(x: &str, y: &str) -> &str {
```

Vamos aplicar a primeira regra: cada parâmetro tem seu próprio lifetime. Desta vez, temos dois parâmetros em vez de um, então temos dois tempos de vida.

```
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Você pode ver que a segunda regra não se aplica porque há mais de um lifetime de entrada. A terceira regra também não se aplica, porque ```longest()``` é uma função em vez de um método, então nenhum dos parâmetros é ```self```. Depois de trabalhar com todas as três regras, ainda não descobrimos qual é o lifetime do tipo de retorno. É por isso que tivemos um erro ao tentar compilar esse código; o compilador trabalhou com as três regras mas ainda não conseguiu descobrir todos os tempos de vida das referências na assinatura.

Como a terceira regra realmente só se aplica em assinaturas de método, veremos as durações nesse contexto a seguir para entender por que a terceira regra significa que não precisamos anotar durações em assinaturas de método com muita frequência.

## 9. Lifetimes em métodos

Quando implementamos métodos em uma struct com lifetimes, usamos a mesma sintaxe que a dos parâmetros de tipo genérico. Onde declaramos e usamos os parâmetros de lifetime depende se eles estão relacionados aos campos da struct ou aos parâmetros e valores de retorno do método. 

Nomes de lifetime para campos de struct sempre precisam ser declarados após a palavra-chave ```impl``` e, em seguida, usados após o nome da struct porque esses tempos de vida fazem parte do tipo da struct. 

Nas assinaturas de métodos dentro do bloco ```impl```, as referências podem estar vinculadas à lifetime nos campos da estrutura, ou podem ser independentes. Além disso, as regras de elipse de lifetime geralmente fazem com que as anotações de lifetime não sejam necessárias nas assinaturas de métodos. Vamos ver alguns exemplos usando a estrutura ```ImportantExcerpt```. 

```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

A declaração do parâmetro de lifetime após ```impl``` e seu uso após o nome do tipo são obrigatórios, mas não somos obrigados a anotar o lifetime da referência a ```self``` devido à primeira regra de elipse. 

Aqui está um exemplo onde a terceira regra de elipse de lifetime se aplica: 

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

Existem dois lifetimes de entrada, então Rust aplica a primeira regra de elipse e dá a ```&self``` e ```announcement``` seus próprios lifetimes. Então, porque um dos parâmetros é ```&self```, o tipo de retorno recebe a duração de vida de ```&self```, e todos os lifetimes foram contabilizados. 

## 10. O lifetime static

Um lifetime especial que precisamos discutir é ```'static```, que denota que a referência afetada pode viver por toda a duração do programa. Todos os literais de string têm lifetime 'static, que podemos anotar da seguinte forma:

```
let s: &'static str = "I have a static lifetime.";
```

O texto desta string é armazenado diretamente no binário do programa, que está sempre disponível. Portanto, a duração de todas as literais de string é 'static. 
 
Você pode ver sugestões para usar 'static em mensagens de erro. Mas antes de especificar 'static como o lifetime para uma referência, pense se a referência que você tem realmente vive o lifetime inteiro do seu programa ou não, e se você quer que isso aconteça. Na maioria das vezes, uma mensagem de erro sugerindo o lifetime 'static resulta da tentativa de criar uma referência pendente ou de uma incompatibilidade dos lifetimes disponíveis. Em tais casos, a solução é consertar esses problemas, não especificar um lifetime 'static.

## 11. Genéricos, traits e lifetimes juntos

```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Aqui temos a função ```longest()``` modificada. Nesta implementação ela  retorna a mais longa de duas slices de string e tem um parâmetro extra chamado ```ann``` do tipo genérico ```T```, que pode ser preenchido por qualquer tipo que implemente a trait ```Display``` conforme especificado pela cláusula ```where```. Este parâmetro extra será impresso usando ```{}```, e é por isso que implementar a trait Display é necessário. Como os lifetimes são um tipo de genérico, as declarações do parâmetro de lifetime 'a e do parâmetro de tipo genérico T vão na mesma lista dentro dos colchetes angulares após o nome da função.

---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

[https://www.naukri.com/code360/library/dangling-reference-in-compiler-design](https://www.naukri.com/code360/library/dangling-reference-in-compiler-design)

---

arataca89@gmail.com

Última atualização: 20240928
