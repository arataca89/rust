#### arataca89

# Linguagem Rust - Smart Pointers

[Introdução](#introdução)

[Box&lt;T&gt;](#boxt) 


---

## Introdução

Um ponteiro é um conceito geral para uma variável que contém um endereço na memória. Este endereço se refere, ou "aponta para", algum outro dado. O tipo mais comum de ponteiro em Rust é uma referência. As referências são indicadas pelo símbolo ```&``` e emprestam o valor para o qual apontam. Elas não têm nenhuma capacidade especial além de se referir a dados e não têm nenhuma sobrecarga.

Os smart pointers (ponteiros inteligentes), por outro lado, são estruturas de dados que agem como um ponteiro, mas também têm metadados e recursos adicionais. O conceito de ponteiros inteligentes não é exclusivo do Rust: ponteiros inteligentes se originaram em C++ e existem em outras linguagens também. O Rust tem uma variedade de ponteiros inteligentes definidos na biblioteca padrão que fornecem funcionalidade além daquela fornecida pelas referências. Para explorar o conceito geral, veremos alguns exemplos diferentes de ponteiros inteligentes, incluindo um tipo de ponteiro inteligente de contagem de referência. Este ponteiro permite que os dados tenham vários proprietários, mantendo o controle do número de proprietários e, quando nenhum proprietário permanecer, limpando os dados.

Rust, com seu conceito de propriedade e empréstimo, tem uma diferença adicional entre referências e ponteiros inteligentes: enquanto referências apenas emprestam dados, em muitos casos, ponteiros inteligentes têm a propriedade dos dados para os quais apontam.

Os tipos ```String``` e ```Vec<T>``` são smart pointers. Ambos contam como ponteiros inteligentes porque possuem alguma memória e permitem que você a manipule. Eles também têm metadados e capacidades ou garantias extras. ```String```, por exemplo, armazena sua capacidade como metadados e tem a capacidade extra de garantir que seus dados serão sempre UTF-8 válido.

Ponteiros inteligentes geralmente são implementados usando structs. Diferentemente de uma ```struct``` comum, ponteiros inteligentes implementam as traits ```Deref``` e ```Drop```. A trait ```Deref``` permite que uma instância da ```struct``` de ponteiro inteligente se comporte como uma referência para que você possa escrever seu código para trabalhar com referências ou ponteiros inteligentes. A trait ```Drop``` permite que você personalize o código que é executado quando uma instância do ponteiro inteligente sai do escopo. Discutiremos aqui ambas as traits e demonstraremos por que elas são importantes para ponteiros inteligentes. 

Visto que o padrão de ponteiro inteligente é um padrão de design geral usado frequentemente em Rust, não serão abordados todos os ponteiros inteligentes existentes. Muitas bibliotecas têm seus próprios ponteiros inteligentes, e você pode até escrever o seu próprio. serão abordados os ponteiros inteligentes mais comuns na biblioteca padrão:

* ```Box<T>``` para alocar valores no heap;
* ```Rc<T>```, um tipo de contagem de referência que permite múltiplos proprietários;
* ```Ref<T>``` e ```RefMut<T>```, acessados através de ```RefCell<T>```, um tipo que impõe as regras de empréstimo em tempo de execução em vez de tempo de compilação. 

Além disso, será abordado o padrão de mutabilidade interna, onde um tipo imutável expõe uma API para mudar um valor interno. Também serão discutidos ciclos de referência: como eles podem vazar memória e como evitá-los.

---

## Box&lt;T&gt;

O ponteiro inteligente mais direto é uma **Box** (caixa), cujo tipo é escrito ```Box<T>```. Caixas permitem que você armazene dados na memória heap em vez de armazenar na memória pilha. O que permanece na pilha é o ponteiro para os dados da heap.

Caixas não têm sobrecarga de desempenho (overhead de performance), além de armazenar seus dados na heap em vez da pilha. Mas elas também não têm muitas capacidades extras. Você as usará com mais frequência nessas situações:

* Quando você tem um tipo cujo tamanho não pode ser conhecido em tempo de compilação e você deseja usar um valor desse tipo em um contexto que requer um tamanho exato;
* Quando você tem uma grande quantidade de dados e deseja transferir a propriedade, mas garantir que os dados não serão copiados ao fazer isso;
* Quando você deseja a propriedade de um valor e se importa apenas com o fato de que ele é um tipo que implementa uma determinada trait em vez de ser de um tipo específico. 
 
Demonstraremos a primeira situação na seção [Habilitando Tipos Recursivos com Boxes](#habilitando-tipos-recursivos-com-boxes). No segundo caso, transferir a propriedade de uma grande quantidade de dados pode levar muito tempo porque os dados são copiados na pilha. Para melhorar o desempenho nessa situação, podemos armazenar a grande quantidade de dados na heap em uma caixa. Então, apenas a pequena quantidade de dados do ponteiro é copiada na pilha, enquanto os dados aos quais ele faz referência permanecem em um lugar na heap. O terceiro caso é conhecido como **objeto de trait**, e será abordado no capítulo referente a OOP em Rust.

### Usando Box&lt;T&gt; para apontar para dados na memória heap

Antes de discutirmos o caso de uso de armazenamento na heap para ```Box<T>```, abordaremos a sintaxe e como interagir com os valores armazenados dentro de uma ``´Box<T>```.

O código abaixo mostra como usar uma caixa para armazenar um valor ```i32``` na heap:

```
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

Definimos a variável **b** para ter o valor de uma **Box** que aponta para o valor **5**, que é alocado na heap. Este programa imprimirá ```b = 5```; neste caso, podemos acessar os dados na **Box** de forma semelhante a como faríamos se esses dados estivessem na pilha. Assim como qualquer valor que a função tem a propriedade, quando uma **Box** sai do escopo, como **b** faz no final da função ```main()```, ela será destruída e terá sua memória desalocada. A desalocação acontece tanto para a **Box** (armazenada na pilha) quanto para os dados para os quais ela aponta (armazenados na heap).

Colocar um único valor na heap não é muito útil, então você não usará caixas sozinhas dessa forma com muita frequência. Ter valores como um único ```i32``` na pilha, onde eles são armazenados por padrão, é mais apropriado na maioria das situações. Vamos ver um caso em que as caixas nos permitem definir tipos que não seriam permitidos se não tivéssemos caixas.

### Habilitando tipos recursivos com Box 
 
Um valor de tipo recursivo pode ter outro valor do mesmo tipo como parte de si mesmo. Tipos recursivos representam um problema porque, em tempo de compilação, o Rust precisa saber quanto espaço um tipo ocupa. No entanto, a aninhamento de valores de tipos recursivos poderia teoricamente continuar infinitamente, então o Rust não pode saber quanto espaço o valor precisa. Como as caixas têm um tamanho conhecido, podemos habilitar tipos recursivos inserindo uma caixa na definição do tipo recursivo.

Como exemplo de um tipo recursivo, vamos explorar a ```cons list```. Este é um tipo de dados comumente encontrado em linguagens de programação funcional. O tipo ```cons list``` que definiremos é direto, exceto pela recursão; portanto, os conceitos no exemplo com o qual trabalharemos serão úteis sempre que você entrar em situações mais complexas envolvendo tipos recursivos.

#### Mais informações sobre cons list

Uma ```cons list``` é uma estrutura de dados que vem da linguagem de programação Lisp e seus dialetos e é composta por pares aninhados, sendo a versão Lisp de uma lista encadeada. Seu nome vem da função ```cons()``` (abreviação de "construct function", ou "função de construção") em Lisp que constrói um novo par a partir de seus dois argumentos. Ao chamar ```cons()``` em um par que consiste de um valor e outro par, podemos construir ```cons list``` compostas por pares recursivos.

Por exemplo, aqui está uma representação de pseudocódigo de uma ```cons list``` contendo a lista **1, 2, 3** com cada par entre parênteses:

```
(1, (2, (3, Nil)))
```

Cada item em uma ```cons list``` contém dois elementos: o valor do item atual e o próximo item. O último item na lista contém apenas um valor chamado **Nil** sem um próximo item. Uma ```cons list``` é produzida chamando recursivamente a função ```cons()```. O nome canônico para denotar o caso base da recursão é **Nil**. Observe que isso não é o mesmo que o conceito de "null" ou "nil", que é um valor inválido ou ausente.

A ```cons list``` não é uma estrutura de dados comumente usada em Rust. Na maioria das vezes, quando você tem uma lista de itens em Rust, ```Vec<T>``` é uma escolha melhor para usar. Outros tipos de dados recursivos mais complexos são úteis em várias situações, mas começando com a ```cons list``` neste capítulo, podemos explorar como as **Box** nos permitem definir um tipo de dados recursivo sem muita distração.

O código abaixo contém uma definição de ```enum``` para uma ```cons list```. Note que este código ainda não irá compilar porque o tipo **List** não tem um tamanho conhecido, o que demonstraremos.

```
enum List {
    Cons(i32, List),
    Nil,
}
```

Note que estamos implementando uma ```cons list``` que contém apenas valores ```i32``` para fins deste exemplo. Poderíamos tê-la implementado usando genéricos, para definir um tipo de ```cons list``` que poderia armazenar valores de qualquer tipo.

Usar o tipo **List** para armazenar a lista 1, 2, 3 ficaria assim:

```
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

O primeiro valor ```Cons``` contém 1 e outro valor **List**. Este valor **List** é outro valor ```Cons``` que contém 2 e outro valor **List**. Este valor **List** é mais um valor ```Cons``` que contém 3 e um valor **List**, que é finalmente **Nil**, a variante não recursiva que sinaliza o fim da lista.

Se tentarmos compilar este código, obtemos o erro mostrado abaixo:

```
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

error[E0391]: cycle detected when computing when `List` needs drop
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which immediately requires computing when `List` needs drop again
  = note: cycle used when computing whether `List` needs drop
  = note: see https://rustc-dev-guide.rust-lang.org/overview.html#queries and https://rustc-dev-guide.rust-lang.org/query.html for more information

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `cons-list` (bin "cons-list") due to 2 previous errors
```

O erro mostra a mensagem "tem tamanho infinito". A razão é que definimos **List** com uma variante que é recursiva: ela contém outro valor de si mesma diretamente. Como resultado, o Rust não consegue descobrir quanto espaço precisa para armazenar um valor **List**. Vamos analisar por que recebemos esse erro. Primeiro, veremos como o Rust decide quanto espaço precisa para armazenar um valor de um tipo não recursivo.

#### Calculando o tamanho de um tipo não recursivo

Dada a enumeração:

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

---

<img src="images/em_construcao.png" width="250" alt="EM CONSTRUCAO">

---

## Referências

[The Book - Chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

arataca89@gmail.com

Última atualização: 20250102