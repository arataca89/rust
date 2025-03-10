#### arataca89

# Linguagem Rust - ownership

Ownership (propriedade) é o recurso do Rust que tem mais implicações profundas no resto da linguagem. A propriedade permite que o Rust possa garantir segurança de memória sem precisar de um coletor de lixo (garbage collection), então é importante entender como ownership funciona.

[1. Entendendo o que é ownership](#1-entendendo-o-que-é-ownership)

[2. Movendo dados na memória](#2-movendo-dados-na-memória)

[3. Clonando dados na memória](#3-clonando-dados-na-memória)

[4. A trait Copy](#4-a-trait-copy)

[5. Ownership e funções](#5-ownership-e-funções)

[6. Valores de retorno e escopo](#6-valores-de-retorno-e-escopo)

[7. Referências e empréstimo](#7-referências-e-empréstimo)

[8. Dangling References](#8-dangling-references)

[9. O tipo de dados slice](#9-o-tipo-de-dados-slice)

[10. Slice de string](#10-slice-de-string)

[11. Literais strings e outros tipos de slices](#11-literais-strings-e-outros-tipos-de-slices)

---

## 1. Entendendo o que é ownership

Ownership (propriedade) consiste em um conjunto de regras que governam como um programa Rust gerencia a memória. Todos os programas precisam gerenciar a maneira como usam a memória de um computador durante a execução. Algumas linguagens têm garbage collection (coleta de lixo) que procura regularmente por memória que não é mais usada enquanto o programa é executado; em outras linguagens, o programador deve alocar e liberar explicitamente a memória. Rust usa uma terceira abordagem: a memória é gerenciada por meio de um sistema de propriedade com um conjunto de regras que o compilador verifica. Se alguma das regras for violada, o programa não será compilado. Nenhum dos recursos de propriedade deixará seu programa lento enquanto ele estiver em execução.

Regras da propriedade(ownership) em Rust:

* Cada valor tem um proprietário;
* Só pode haver um proprietário de cada vez;
* Quando o proprietário sai do escopo o valor é destruído.

Os tipos de dados mais simples (inteiros, decimais, booleanos, caracteres) têm seu tamanho conhecido e podem ser armazenados na região de memória da pilha. Estes dados podem ser facilmente  inseridos e removidos da pilha e podem ser copiados rapidamente quando necessário.

Tipos de dados mais complexos (strings, vetores, objetos criados dinamicamente, etc...) não têm seu tamanho conhecido em tempo de compilação e são armazenados na memória heap, o tipo String é um exemplo. O armazenamento de dados na heap exige que a região de memória alocada para o programa seja devolvida ao sistema quando não estiver mais sendo usada. Além disso, estes tipos de dados têm um custo maior para serem copiados e muitas vezes é preferível que sejam movidos para outras posições de memória e não copiados.

Literais string, valores envoltos em aspas duplas, como por exemplo "Calango", são usados quando normalmente já conhecemos a string a ser processada. Este tipo de dado é conveniente, mas não é adequado para todas as situações onde queremos usar texto. Um dos motivos é porque eles são imutáveis. Outro motivo é que nem todo valor de string pode ser conhecido quando escrevemos nosso código: por exemplo, e se quisermos receber entrada do usuário e armazená-la? Para essas situações, Rust tem o tipo String. O tipo de dados String é armazenado na memória heap e, assim, é capaz de armazenar uma quantidade de texto que é desconhecida para nós em tempo de compilação. 

Um objeto String pode ser criado a partir de um literal string pelo uso da função ```from()```:

```
let s = String::from("hello");
```

O tipo String pode ser mutável:

```
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() adiciona um literal a String

    println!("{s}"); // hello, world!
```


Quando usamos literais string, conhecemos o valor do texto em tempo de compilação, então o texto é codificado diretamente no executável final. É por isso que as literais string são rápidas e eficientes. Mas essa rapidez e eficiência vêm da imutabilidade da literal string. Mas ao criar programas reais precisamos de texto imutável e texto mutável, ou melhor, precisamos de dados imutáveis e mutáveis, sejam eles de qualquer tipo.

Quando usamos o tipo String, ou qualquer outro tipo de dados mutável,  precisamos colocar dados na memória heap pois não sabemos a quantidade de memória que será necessária em tempo de compilação; e para usar a memória heap precisamos:

* Solicitar esta memória em tempo de execução;
* Uma maneira de retornar essa memória ao sistema quando não precisarmos mais dela.

A solicitação de memória ao sistema normalmente é feita através de funções fornecidas pela linguagem de programação. No caso de usarmos String em Rust podemos usar ```String::from()```. Essa abordagem é praticamente  universal nas linguagens de programação. 

No entanto, o retorno da memória não mais necessária não é tão simples assim. Em linguagens com um coletor de lixo, o famoso garbage collector (GC), o GC rastreia e limpa a memória que não está mais sendo usada, e não precisamos pensar sobre isso. Nas linguagens que não têm GC, é responsabilidade do programador identificar quando a memória não está mais sendo usada e chamar o código para liberá-la explicitamente, assim como fizemos para solicitá-la. Fazer isso corretamente tem sido historicamente um problema de programação difícil. Se esquecermos, desperdiçaremos memória. Se fizermos isso muito cedo, teremos uma variável inválida. Se fizermos isso duas vezes, isso também é um bug. Precisamos parear exatamente uma alocação de memória com sua liberação. Ou seja, para cada malloc() um free(); para cada new() um delete().

Rust adota uma abordagem diferente: a memória é devolvida automaticamente quando a variável que a possui sai do escopo. 

Quand a variável sai do escopo Rust chama um método especial da biblioteca chamado [drop()](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop). Este método executa o destrutor do objeto implicitamente.

Esta forma de gerenciar a memória heap tem um impacto profundo na forma como o código Rust é escrito. Pode parecer simples agora, mas o comportamento do código pode ser inesperado em situações mais complicadas quando queremos que várias variáveis usem os dados alocados no heap. 

## 2. Movendo dados na memória

Em Rust, múltiplas variáveis podem interagir com os mesmos dados de maneiras diferentes. Vamos ver um exemplo usando um inteiro. 

```
    let x = 5;
    let y = x;
```

Provavelmente podemos imaginar o que este trecho de código faz: "vincule o valor 5 a x; então faça uma cópia do valor em x e vincule-o a y." Agora temos duas variáveis, x e y, e ambas iguais a 5. Isso é realmente o que está acontecendo, porque inteiros são valores simples com um tamanho fixo conhecido, e esses dois valores 5 são armazenados na região de memória da pilha.

Vamos dar uma olhada no mesmto tipo de código, mas usando um tipo complexo, por exemplo, String.

```
    let s1 = String::from("hello");
    let s2 = s1;
```

Isso parece muito semelhante, então poderíamos assumir que a forma como funciona seria a mesma: ou seja, a segunda linha faria uma cópia do valor em s1 e o vincularia a s2. Mas isso não é bem o que acontece. 

Observe na figura abaixo como um objeto String é armazenado na memória. Na parte esquerda temos a representação do objeto s1 do tipo String. Um objeto String é composto por três partes: um ponteiro(ptr) para a memória que contém o conteúdo da string, um comprimento(len) e uma capacidade(capacity). Este grupo de dados é armazenado na pilha. À direita está a memória no heap que contém o conteúdo. 

<img alt="Campos de um objeto String" src="images/ownership1.svg" class="center" style="width: 50%;">
 
O comprimento é a quantidade de memória, em bytes, que o conteúdo da String está usando atualmente. A capacidade é a quantidade total de memória, em bytes, que a String recebeu do alocador. A diferença entre comprimento e capacidade é importante, mas não neste contexto, então, por enquanto, está tudo bem ignorar a capacidade. 

Quando atribuímos s1 a s2, os dados da String são copiados, o que significa que copiamos o ponteiro, o comprimento e a capacidade que estão na pilha. Não copiamos os dados na memória heap para os quais o ponteiro se refere. Em outras palavras, a representação de dados na memória se parece com a figura abaixo. 

<img alt="Atribuindo um objeto String a outra variavel" src="images/ownership2.svg" class="center" style="width: 50%;">

A representação <font color="red">NÃO</font> se parece com a figura abaixo, que é como a memória seria se o Rust copiasse os dados do heap também. Se o Rust fizesse isso, a operação s2 = s1 poderia ser muito cara em termos de desempenho de tempo de execução se os dados no heap fossem grandes. 

<img alt="Se String fosse copiado" src="images/ownership3.svg" class="center" style="width: 50%;">

Anteriormente, dissemos que quando uma variável sai do escopo, o Rust chama automaticamente a função drop() e limpa a memória heap para essa variável. Mas observe, na figura mais acima referente a atribuição de s1 a s2, que ambos os ponteiros de dados apontam para a mesma região de memória heap. Isso é um problema: quando s2 e s1 saem do escopo, ambos tentarão liberar a mesma memória. Isso é conhecido como erro de liberação dupla (double free error) e é um dos bugs de segurança de memória que mencionamos anteriormente. Liberar memória duas vezes pode levar à corrupção de memória, o que pode levar a vulnerabilidades de segurança.

Para garantir a segurança da memória, após a linha ```let s2 = s1;```, o Rust considera s1 como inválido. Portanto, o Rust não precisa liberar nada quando s1 sai do escopo. Verifique o que acontece quando você tenta usar s1 depois que s2 é criado; não funcionará: 

```
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
```

Ao compilar este código você receberá o seguinte erro porque Rust impede que você use a referência inválida: 

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |               ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

Se você já ouviu os termos shallow copy(cópia superficial) e deep copy(cópia profunda) ao trabalhar com outras linguagens, o conceito de copiar o ponteiro, o comprimento e a capacidade sem copiar os dados provavelmente soa como fazer uma cópia superficial. Mas, como o Rust também invalida a primeira variável, em vez de ser chamada de cópia superficial, este procedimento é conhecido como movimentação. Neste exemplo, diríamos que s1 foi movido para s2. Então, o que realmente acontece é mostrado na figura abaixo.

<img alt="Movimentação de um objeto String" src="images/ownership4.svg" class="center" style="width: 50%;">

Isso resolve nosso problema! Com apenas a variável s2 válida, quando ela sair do escopo, ela sozinha liberará a memória, e pronto.

Além disso, há uma escolha de design que é implícita por isso: Rust nunca criará automaticamente cópias "profundas" de seus dados. Portanto, qualquer cópia automática pode ser considerada barata em termos de desempenho de tempo de execução. 

## 3. Clonando dados na memória

Se quisermos fazer deep copy(cópia profunda) de um objeto, ou seja, copiar os dados armazenados na memória heap além dos dados armazenados na memória stack (pilha), podemos usar o método ```clone()```.

```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```

Após este código teremos dois objetos String, cada um com suas partes na memória stack e na memória heap. Lembre-se que este método executa a cópia como imaginamos mas tem um custo.

Lembre-se que tipos simples (inteiros, float, char, boolean) que têm um tamanho conhecido em tempo de compilação, são armazenados inteiramente na pilha (memória stack), então cópias dos valores reais são rápidas de fazer. Isso significa que neste caso não há necessidade de usar o método clone(). Em outras palavras, não há diferença entre shallow copy(cópia superficial) e deep copy(cópia profunda) ao lidar com tipos simples, então chamar clone() não faria nada diferente da cópia superficial comum.

## 4. A trait Copy

Rust possui uma anotação especial chamada de ```trait Copy``` que podemos colocar em tipos que são armazenados na pilha, como inteiros. Se um tipo implementa o trait Copy, variáveis que o usam não são movidas, mas sim copiadas, tornando-as ainda válidas após a atribuição a outra variável. 

O Rust não permite que anotemos um tipo com Copy se o tipo, ou qualquer uma de suas partes, implementou a trait ```Drop```. Se o tipo precisar de algum procedimento especial quando o valor sair do escopo e adicionarmos a anotação Copy a esse tipo, obteremos um erro de compilação. Para saber como adicionar a anotação Copy ao seu tipo para implementar a trait, consulte [Traits deriváveis](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).

Então, quais tipos implementam a trait Copy? Você pode verificar a documentação do tipo fornecido para ter certeza, mas, como regra geral, qualquer grupo de valores escalares simples pode implementar Copy, e nada que exija alocação na memória heap ou aquisição de algum recurso pode implementar Copy. Aqui estão alguns dos tipos que implementam Copy:

* Todos os tipos inteiros, como u32.
* O tipo booleano, bool, com valores true e false.
* Todos os tipos de ponto flutuante, como f64.
* O tipo de caractere, char.
* Tuplas, se elas contiverem apenas tipos que também implementam Copy. Por exemplo, (i32, i32) implementa Copy, mas (i32, String) não.

## 5. Ownership e funções

A mecânica de passar um valor para uma função é semelhante àquela quando se atribui um valor a uma variável. Passar uma variável para uma função moverá ou copiará, assim como a atribuição. 

```
fn main() {
    let s = String::from("hello");  // s entra no escopo

    takes_ownership(s);             // o valor de s é movido para dentro da função...
                                    // ... e não é mais válido aqui

    let x = 5;                      // x entra no escopo

    makes_copy(x);                  // x poderia ser movido para dentro da função,
                                    // mas i32 é Copy, então beleza
                                    // usar x depois
} // Aqui x sai do escopo; s também, mas como s foi movido está tudo certo.

fn takes_ownership(some_string: String) { // some_string entra no escopo
    println!("{some_string}");
} // Aqui, some_string sai do escopo e `drop()` é chamado e
  // a memória alocada é liberada.

fn makes_copy(some_integer: i32) { // some_integer entra no escopo
    println!("{some_integer}");
} // Aqui, some_integer sai do escopo. Nada de especial acontece.
```
Se tentássemos usar s após a chamada para takes_ownership(), Rust lançaria um erro de compilação. Essas verificações estáticas nos protegem de erros. 

## 6. Valores de retorno e escopo

Retornar valores também pode transferir a propriedade. 

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership move seu valor de retorno para s1

    let s2 = String::from("hello");     // s2 entra no escopo

    let s3 = takes_and_gives_back(s2);  // s2 é movido para dentro de
                                        // takes_and_gives_back(), que por sua vez
                                        // move seu valor de retorno para s3
} // Aqui, s3 sai do escopo e drop() é chamado. s2 foi movido, então nada
  // acontece. s1 sai do escopo e drop() é chamado.

fn gives_ownership() -> String {             // gives_ownership() moverá seu
                                             // valor de retorno para a função chamadora.

    let some_string = String::from("yours"); // some_string entra no escopo

    some_string                              // some_string é retornada e
                                             // movida para fora para a função chamadora.
}

// Esta função pega um objeto String e retorna outro.
fn takes_and_gives_back(a_string: String) -> String { // a_string entra no escopo

    a_string  // a_string é retornada e movida para a função chamadora.
}
```

A propriedade de uma variável segue o mesmo padrão sempre: atribuir um valor a outra variável move o valor. Quando uma variável que inclui dados no heap sai do escopo, o valor será limpo por drop(), a menos que a propriedade dos dados tenha sido movida para outra variável. 

Embora isso funcione, assumir a propriedade e depois retornar a propriedade com cada função é um pouco tedioso. E se quisermos permitir que uma função use um valor, mas não assuma a propriedade? É bastante irritante que tudo o que passarmos também precise ser passado de volta se quisermos usá-lo novamente, além de quaisquer dados resultantes do corpo da função que possamos querer retornar também. 

Rust nos permite retornar múltiplos valores usando uma tupla, como mostrado abaixo.

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("O comprimento de '{s2}' é {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() retorna o tamanho da String

    (s, length)
}
``` 

Mas nem sempre é isso que queremos. Felizmente para nós, Rust tem um recurso para usar um valor sem transferir a propriedade, chamado ```referência```. 

## 7. Referências e empréstimo

Observe que no código da função  calculate_length() temos que retornar a String para que a função chamadora ainda possa usar a String após a chamada. Isto tem que ser feito porque a String foi movida para dentro de calculate_length(). Por isso tivemos que retornar a String e o tamanho da String, os dois valores encapsulados numa tupla.

Em vez disso, podemos fornecer uma referência ao valor da String. Uma referência é como um ponteiro, pois é um endereço de memória que podemos seguir para acessar os dados armazenados naquele endereço; esses dados são de propriedade de alguma outra variável. Ao contrário de um ponteiro, uma referência tem a garantia de apontar para um valor válido de um tipo específico durante a vida útil dessa referência.

Abaixo temos a função calculate_length() refatorada para usar uma referência a um objeto como parâmetro em vez de assumir a propriedade do valor: 

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("O comprimento de '{s1}' é {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

O caractere de e-comercial (``` & ```) representa uma referência  e permite que você se refira a algum valor sem assumir a propriedade dele.

A sintaxe ``` &s1 ``` permite-nos criar uma referência que se refere ao valor de ``` s1 ``` sem ter sua propriedade. Como não tem a propriedade de s1 o valor para o qual a referência aponta não será descartado quando a referência deixar de ser usada. 

```
fn calculate_length(s: &String) -> usize { // s é uma referência para um objeto do tipo String
    s.len()
} // Aqui, s sai do escopo. Mas como  não tem a propriedade do valor referenciado, drop() não é chamado.
```

Chamamos a ação de criar uma referência de empréstimo (borrowing). Como na vida real, se uma pessoa possui algo, você pode pegá-lo emprestado dela. Quando terminar, você tem que devolvê-lo. Você não é o dono. 

Assim como as variáveis, as referências são imutáveis por padrão. O código abaixo não irá compilar.

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Para alterar um valor referenciado você deve utilizar referências mutáveis. O código abaixo compila sem erros. Observe que usamos ```mut``` tanto na declaração de s quanto na referência usada na chamada da função.

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Referências mutáveis têm uma grande restrição: se você tiver uma referência mutável para um valor, você NÃO pode ter outras referências para esse valor. 

```
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

O código acima emitirá o seguinte erro de compilação.

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

O benefício de ter essa restrição é que Rust pode prevenir corridas de dados (data races) em tempo de compilação. Uma corrida de dados é semelhante a uma condição de corrida (race condition) e acontece quando esses três comportamentos ocorrem:

* Dois ou mais ponteiros acessam os mesmos dados ao mesmo tempo.
* Pelo menos um dos ponteiros está sendo usado para escrever nos dados.
* Não há nenhum mecanismo sendo usado para sincronizar o acesso aos dados.

Corridas de dados causam comportamento indefinido e podem ser difíceis de diagnosticar e corrigir quando você está tentando rastreá-las em tempo de execução; Rust previne esse problema recusando compilar código com corridas de dados! 

Note que podemos usar chaves para criar um novo escopo, permitindo múltiplas referências mutáveis, apenas não simultâneas. 

```
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 sai do escopo aqui, então podemos criar nova referencia sem problemas 
    
    let r2 = &mut s;
```

Rust impõe uma regra semelhante para combinar referências mutáveis e imutáveis. Este código resulta em um erro:

```
    let mut s = String::from("hello");

    let r1 = &s; // sem problemas
    let r2 = &s; // sem problemas
    let r3 = &mut s; // GRANDE PROBLEMA

    println!("{}, {}, and {}", r1, r2, r3);
```

Aqui temos o erro:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

Observe que também não podemos ter uma referência mutável enquanto temos uma imutável para o mesmo valor. 

Usuários de uma referência imutável não esperam que o valor mude repentinamente! No entanto, múltiplas referências imutáveis são permitidas porque ninguém que está apenas lendo os dados tem a capacidade de afetar a leitura de outra pessoa. 

Observe que o escopo de uma referência começa a partir de onde ela é introduzida e continua até a última vez que a referência é usada. Por exemplo, o código abaixo irá compilar porque a última utilização das referências imutáveis, o ```println!```, ocorre antes da referência mutável ser introduzida: 

```
    let mut s = String::from("hello");

    let r1 = &s; // sem problemas
    let r2 = &s; // sem problemas
    println!("{r1} and {r2}");
    // as variáveis r1 e r2 will não são usadas após este ponto

    let r3 = &mut s; // sem problemas
    println!("{r3}");
```

Os escopos das referências imutáveis r1 e r2 terminam após o println! onde são usadas pela última vez, o que é antes da referência mutável r3 ser criada. Esses escopos não se sobrepõem, então este código é permitido: o compilador pode dizer que a referência não está mais sendo usada em um ponto antes do final do escopo. 

Embora erros de empréstimo possam ser frustrantes às vezes, lembre-se que é o compilador Rust apontando um possível bug antecipadamente (em tempo de compilação em vez de em tempo de execução) e mostrando exatamente onde o problema está. Então você não precisa rastrear por que seus dados não são o que você pensava que eram. 

## 8. Dangling References

Em linguagens com ponteiros, é fácil criar erroneamente um dangling pointer (ponteiro pendurado) - um ponteiro que referencia um local na memória que pode ter sido dado a outro ponteiro - liberando alguma memória enquanto preserva um ponteiro para essa memória. Em Rust, por outro lado, o compilador garante que as referências nunca serão referências penduradas: se você tiver uma referência a alguns dados, o compilador garantirá que os dados não sairão do escopo antes que a referência aos dados o faça. 

Vamos tentar criar uma referência pendente para ver como o Rust as previne com um erro de compilação: 

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Este código emitirá o seguinte erro:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
5 | fn dangle() -> &'static String {
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
  |

error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:8:5
  |
8 |     &s
  |     ^^ returns a reference to data owned by the current function

Some errors have detailed explanations: E0106, E0515.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `ownership` (bin "ownership") due to 2 previous errors
```

Observe que como s é criado dentro de dangle(), quando o código de dangle() terminar, s será desalocado. Mas tentamos retornar uma referência a ele. Isso significa que essa referência estaria apontando para uma String inválida. Isso não é bom! O Rust não nos permite fazer isso.

A solução aqui seria em vez de retornar uma referência retornar um objeto String, quando teríamos a transferência da propriedade, ou usar lifetimes.

```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

O código acima funcionaria beleza pois a propriedade é movida para a função chamadora e nada é desalocado.

## 9. O tipo de dados slice

Em Rust, uma slice (fatia) é um tipo de dados que referencia uma sequência contígua de elementos em uma coleção. Uma fatia é um tipo de referência, portanto não possui propriedade.

Aqui está um pequeno problema de programação: escreva uma função que receba uma string de palavras separadas por espaços (uma frase, digamos assim) e retorne a primeira palavra que encontrar nessa string. Se a função não encontrar um espaço na string, a string inteira deve ser uma única palavra, então a string inteira deve ser retornada.

Vamos trabalhar em como escreveríamos a assinatura dessa função sem usar slice, para entender o problema que as slices resolverão: 

```
fn first_word(s: &String) -> ?
```

A função ```first_word()``` tem uma ``` &String ``` como parâmetro. Não queremos a propriedade, então isso está correto. Mas o que devemos retornar? Não temos realmente uma maneira de falar sobre parte de uma string. No entanto, poderíamos retornar o índice do final da palavra, indicado por um espaço. Vamos tentar isso. 

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Como precisamos percorrer o elemento String item por item e verificar se o valor do item é um espaço, convertemos nossa String em um array de bytes usando o método ``` as_bytes() ```.

```
    let bytes = s.as_bytes();
```

Em seguida, criamos um iterador sobre o array de bytes usando o método ``` iter() ```:

```
    for (i, &item) in bytes.iter().enumerate() {
```

O métoto ``` iter() ``` retorna cada elemento em uma coleção.

O método ``` enumerate() ``` encapsula o resultado de ``` iter() ``` em uma tupla contendo dois elementos, o primeiro elemento é o índice e o segundo elemento é uma referência ao elemento. 

Como ``` enumerate() ``` retorna uma tupla, podemos usar padrões para desestruturar essa tupla. Especificamos um padrão que tem ``` i ``` para o índice na tupla e ``` &item ``` para o único byte na tupla.  Como obtemos uma referência ao elemento de ```  iter().enumerate() ```, usamos ``` & ``` no padrão. 

```
(i, &item)
```

Dentro do loop for, procuramos pelo byte que representa o espaço usando a sintaxe literal de byte. Se encontrarmos um espaço, retornamos a posição. Caso contrário, retornamos o comprimento da string usando o método ``` len() ```.

```
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

Agora temos uma maneira de descobrir o índice do fim da primeira palavra na string, mas há um problema. Estamos retornando um ```usize``` por si só, mas é apenas um número significativo no contexto do ```&String```. Em outras palavras, porque é um valor separado do objeto String, não há garantia de que ele ainda será válido no futuro. Considere o programa abaixo que usa a função ```first_word()```.

```
fn main() {
	let mut s = String::from("hello world");

	let word = first_word(&s); // word receberá o valor 5

	s.clear(); // aqui limpamos a string, fazendo-a igual a ""

	// word ainda terá o valor 5 aqui, mas não existe mais a string
	// que poderíamos usar com este valor 5. word agora é totalmente
	// inválido.
}
```

Este programa compila sem erros e também o faria se usássemos ```word``` após chamar ```s.clear()```. Porque ```word``` não está conectada ao estado de ```s``` de forma alguma, ```word``` ainda contém o valor 5. Poderíamos usar esse valor 5 com a variável ```s``` para tentar extrair a primeira palavra, mas isso seria um bug porque o conteúdo de ```s``` mudou desde que salvamos 5 em ```word```. 

A preocupação com o índice em ```word``` ficando fora de sincronia com os dados em ```s``` é tedioso e propenso a erros! Gerenciar esses índices é ainda mais frágil se escrevermos uma função ```second_word()```. Sua assinatura teria que ser assim:

```
fn second_word(s: &String) -> (usize, usize) {
```

Agora estamos rastreando um índice inicial e um índice final, e temos ainda mais valores que foram calculados a partir de dados em um estado particular, mas não estão vinculados a esse estado. Temos três variáveis ​​não relacionadas flutuando por aí que precisam ser mantidas sincronizadas.

Felizmente, o Rust tem uma solução para esse problema: slices.

## 10. Slice de string

Uma slice de string é uma referência a uma parte de uma string:

```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

No código acima, a variável ```hello``` é uma referência a uma parte da String especificada pelo intervalo ```[0..5]```.

Criamos slices usando a sintaxe de intervalo entre colchetes especificando ```[índice_inicial..índice_final]```, onde ```índice_inicial``` é a primeira posição na string e ```índice_final``` é um a mais que a última posição na string. Ou seja, a slice ```[0..5]``` refere-se ao pedaço da string que inicia no caracete que está na posição 0 e termina no caractere que está na posição 4 ("hello"). 

Com a sintaxe de intervalo ```..``` do Rust, se você quiser começar no índice 0, pode descartar o valor antes dos dois pontos. Em outras palavras, as slices abaixo são iguais: 

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Da mesma forma, se a slice incluir o último byte da String, você pode descartar o número final. 

```
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Você também pode descartar ambos os valores para obter uma slice de toda a string. 

```
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

OBSERVAÇÃO: Os índices do intervalo da slice de string devem ocorrer em limites de caracteres UTF-8 válidos. Se você tentar criar uma slice de string no meio de um caractere de vários bytes, seu programa será encerrado com um erro. Para fins de introdução as slices de string, estamos assumindo apenas caracteres ASCII nesta seção.

Com todas essas informações em mente, vamos reescrever a função ```first_word()``` para retornar uma slice. O tipo de dados referente a uma slice de string é escrito como ```&str```:

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Obtemos o índice para o final da palavra da mesma forma que fizemos na implementação anterior, procurando a primeira ocorrência de um espaço. Quando encontramos um espaço, retornamos uma slice de string usando o início da string e o índice do espaço como os índices inicial e final.

Agora, quando chamamos ```first_word()```, recebemos um valor que está ligado aos dados passados como argumento da função.

Retornar uma slice também funcionaria para a função ```second_word()```:

```
fn second_word(s: &String) -> &str {
```

Agora temos uma API que é muito mais difícil de bagunçar porque o compilador garantirá que as referências na String permaneçam válidas. 
No bug da implementação anterior, pegamos o índice da primeira palavra, mas depois limpamos a string, então nosso índice ficou inválido. Esse código estava logicamente incorreto, mas não mostrou nenhum erro de imediato. Os problemas apareceriam mais tarde se continuássemos tentando usar o índice da primeira palavra com uma string vazia.

Slices tornam esse bug impossível e nos avisam que temos um problema com nosso código muito mais cedo. Usar a versão slice de ```first_word()``` lançará um erro de tempo de compilação:

```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // ERRO

    println!("the first word is: {word}");
}
```

Abaixo temos o erro emitido ao tentar compilar este código:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {word}");
   |                                  ------ immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

Lembre-se das regras de empréstimo que se temos uma referência imutável para algo, não podemos também ter uma referência mutável. Como ```clear()``` precisa truncar a String, ele precisa obter uma referência mutável. O ```println!``` após a chamada a ```clear()``` usa a referência em ```word```, então a referência imutável ainda deve estar ativa naquele ponto. Rust não permite que a referência mutável em ```clear()``` e a referência imutável em ```word``` existam ao mesmo tempo, e a compilação falha. Rust não apenas tornou nossa API mais fácil de usar, mas também eliminou uma classe inteira de erros em tempo de compilação!

## 11. Literais strings e outros tipos de slices

As literais de string são armazenadas dentro do binário. Por exemplo:

```
let s = "Hello, world!";
```

O tipo de ```s``` aqui é ```&str```: é uma slice que aponta para aquele ponto específico do binário. Esta é também a razão pela qual literais de string são imutáveis; ```&str``` é uma referência imutável.

Com o que aprendemos até agora podemos refatorar a função ```first_word()```; atualmente sua assinatura é:

```
fn first_word(s: &String) -> &str {
```

Um programador Rust mais experiente alteraria a assinatura desta função conforme abaixo, pois assim ela permitiria que usássemos a mesma função com valores ```&String``` e ```&str```.

```
fn first_word(s: &str) -> &str {
```

Se tivermos um tipo ```&str```, podemos passá-lo diretamente. Se tivermos um tipo ```String```, podemos passar uma slice da String ou uma referência à String pois a linguagem Rust converte um ```&String``` para um ```&str``` implicitamente.

Definir uma função para receber uma slice de string em vez de uma referência a uma String torna nossa API mais geral e útil sem perder nenhuma funcionalidade: 

```
fn main() {
    let my_string = String::from("hello world");

	// `first_word` funciona com slices de String, seja parcial ou completa
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

	// `first_word` funciona com referência a String que equivale
	// a uma slice da String toda.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

	// `first_word` funciona com slices de literais string, seja parcial ou completa
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

	// Como literais string já são slices de string isto funciona também, sem a sintaxe de slice
    let word = first_word(my_string_literal);
}
```

Slices podem ser de outros tipos, considere o array abaixo:

```
let a = [1, 2, 3, 4, 5];
```

Assim como podemos querer nos referir a parte de uma string, podemos querer nos referir a parte de um array. Faríamos isso assim: 

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Esta slice é do tipo ``´&[i32]``` e funciona da mesma forma que as slices de string. Você pode usar slices para todos os tipos em outras coleções. 

---
## Referências

[Capítulo 4 do Livro](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)



---

arataca89@gmail.com

Última atualização: 20241227
