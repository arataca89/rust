#### arataca89

# Linguagem Rust - estruturas

Uma ```struct```, ou estrutura, é um tipo de dado criado pelo programador que permite encapsular vários valores relacionados de modo que o tipo de dado criado melhore o significado do seu uso e a compreensão das diversas partes do programa. Se você estiver familiarizado com uma linguagem orientada a objetos, uma ```struct``` é como os atributos de dados de um objeto. 

* [Definindo e instanciando estruturas](#definindo-e-instanciando-estruturas)

* [Usando a sintaxe abreviada para inicializar os campos da struct](#usando-a-sintaxe-abreviada-para-inicializar-os-campos-da-struct)

* [Criando instâncias a partir de outras instâncias](#criando-instâncias-a-partir-de-outras-instâncias)

* [Estruturas de tupla](#estruturas-de-tupla)

* [Estruturas sem campos](#estruturas-sem-campos)

* [Propriedade dos dados da struct](#propriedade-dos-dados-da-struct)

* [Exemplo do uso de struct](#exemplo-do-uso-de-struct)

* [Métodos](#métodos)

---

## Definindo e instanciando estruturas

Uma estrutura pode possuir vários campos nomeados que podem ser de tipos diferentes. 


Para definir uma ```struct``` usa-se a seguinte sintaxe:

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

Para definir uma estrutura usamos a palavra-chave ```struct```, seguida pelo nome da estrutura, neste caso ```User```, seguida dos campos da estrutura entre chaves.

Cada campo da estrutura tem um nome, seguido pelo caractere de dois pontos, seguido pelo tipo de dados do campo, seguido pelo caractere vírgula.

Neste exemplo, observe que a estrutura encapsula todos os dados referentes a um usuário de um sistema: se ele está ou não ativo, seu nome de usuário, seu email e quantas vezes ele logou no sistema.

Para usar uma ```struct``` depois de defini-la, criamos uma instância dessa ```struct``` especificando valores concretos para cada um dos campos. Criamos uma instância declarando o nome da ```struct``` e então adicionamos chaves contendo pares chave:valor, onde as chaves são os nomes dos campos e os valores são os dados que queremos armazenar nesses campos. Não precisamos especificar os campos na mesma ordem em que os declaramos na ```struct```. Em outras palavras, a definição da ```struct``` é como um modelo geral para o tipo, e as instâncias preenchem esse modelo com dados específicos para criar valores do tipo. Por exemplo, podemos declarar um usuário específico conforme mostrado abaixo.

```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

Para obter um valor específico de uma ```struct```, usamos a notação de ponto. Por exemplo, para acessar o endereço de e-mail deste usuário, usamos ```user1.email```. Se a instância for mutável, podemos alterar um valor usando a notação de ponto e atribuindo novo valor a um campo específico. Abaixo mostramos como alterar o valor no campo de e-mail de uma instância ```User``` mutável.

```
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Observe que toda a instância deve ser mutável; Rust não permite que marquemos apenas certos campos como mutáveis. 

Como em qualquer expressão, podemos construir uma nova instância da estrutura como a última expressão no corpo da função para retornar implicitamente essa nova instância. O código abaixo mostra uma função ```build_user()``` que retorna uma instância de ```User``` com o email e nome de usuário fornecidos. O campo ```active``` recebe o valor ```true``` e o ```sign_in_count``` recebe o valor 1.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

Faz sentido nomear os parâmetros da função com o mesmo nome dos campos da estrutura, mas ter que repetir os nomes dos campos e variáveis de email e nome de usuário é um pouco tedioso. Se a estrutura tivesse mais campos, repetir cada nome ficaria ainda mais irritante. Felizmente, existe um atalho conveniente! 

## Usando a sintaxe abreviada para inicializar os campos da struct

Como os nomes dos parâmetros e os nomes dos campos da estrutura são exatamente iguais no código acima, podemos usar a sintaxe abreviada de inicialização de campo para reescrever ```build_user()``` para que se comporte exatamente da mesma forma, mas não tenha a repetição de ```username``` e ```email```.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Aqui, estamos criando uma nova instância da estrutura ```User```, que possui um campo chamado ```email```. Queremos definir o valor do campo ```email``` para o valor no parâmetro de mesmo nome, ```email``` da função ```build_user()```. Como o campo ```email``` e o parâmetro ```email``` têm o mesmo nome, não precisamos escrever ```email: email``` , podemos escrever apenas ```email``` e o Rust vai entender que queremos usar o valor passado para a função no parâmetro de mesmo nome.

## Criando instâncias a partir de outras instâncias 

É frequentemente útil criar uma nova instância de uma ```struct``` que use alguns dos valores de outra instância. Você pode fazer isso usando a sintaxe de atualização de ```struct```.

Primeiro, mostramos como criar uma nova instância de ```User``` em ```user2``` sem a sintaxe de atualização. Definimos um novo valor para email e nos outros campos usamos os valores de ```user1```.

```
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Usando a sintaxe de atualização de ```struct```, podemos alcançar o mesmo efeito com menos código. A sintaxe ```..``` (dois pontos seguidos) especifica que os campos restantes não definidos explicitamente devem ter o mesmo valor que os campos da instância fornecida. 

```
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Este código também cria uma instância em ```user2``` que possui um valor diferente para email, mas possui os mesmos valores para os campos username, active e sign_in_count de ```user1```. O ```..user1``` deve vir por último para especificar que quaisquer campos restantes devem receber seus valores dos campos correspondentes em ```user1```, mas podemos escolher especificar valores para quantos campos quisermos em qualquer ordem, independentemente da ordem dos campos na definição da estrutura. 

Observe que a sintaxe de atualização de ```struct``` usa ```=``` (o caractere de igualdade) como operador de atribuição; isso ocorre porque ele move os dados. Neste exemplo, não podemos mais usar ```user1``` como um todo após criar ```user2``` porque a ```String``` no campo ```username``` de ```user1``` foi movida para ```user2```. Se tivéssemos dado a ```user2``` novos valores ```String``` para ```email``` e ```username```, e assim usado apenas os valores ```active``` e ```sign_in_count``` de ```user1```, então ```user1``` ainda seria válido após criar ```user2```. Ambos ```active``` e ```sign_in_count``` são tipos que implementam a trait ```Copy```, então os dados seriam copiados em vez de movidos.

## Estruturas de tupla 

Rust também suporta estruturas que se parecem com tuplas, chamadas de structs de tupla. Structs de tupla têm o significado adicional que o nome da struct fornece, mas não têm nomes associados a seus campos; em vez disso, eles apenas têm os tipos dos campos. Structs de tupla são úteis quando você deseja dar à tupla inteira um nome e fazer a tupla ser um tipo diferente de outras tuplas, e quando nomear cada campo como em uma struct regular seria verboso ou redundante. 

Para definir uma estrutura de tupla, comece com a palavra-chave ```struct``` e o nome da estrutura seguido pelos tipos na tupla. Por exemplo, aqui definimos e usamos duas estruturas de tupla chamadas ```Color``` e ```Point```:

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Note que os valores ```black``` e ```origin``` são tipos diferentes porque são instâncias de diferentes structs de tupla. Cada ```struct``` que você define é seu próprio tipo, mesmo que os campos dentro da struct possam ter os mesmos tipos. Por exemplo, uma função que recebe um parâmetro do tipo ```Color``` não pode receber um ```Point``` como argumento, mesmo que ambos os tipos sejam compostos de três valores ```i32```. 

As instâncias de struct de tupla são semelhantes às tuplas, pois você pode desestruturá-las em suas partes individuais e pode usar um ```.``` (ponto) seguido pelo índice para acessar um valor individual. 

## Estruturas sem campos

Você também pode definir structs que não possuem nenhum campo! Essas são chamadas de structs de tipo unitário porque se comportam de forma semelhante a ```()```, o tipo unitário. Structs de tipo unitário podem ser úteis quando você precisa implementar uma trait em algum tipo, mas não possui nenhum dado que deseja armazenar no próprio tipo. Aqui está um exemplo de declaração e instanciação de uma struct unitária chamada ```AlwaysEqual```:

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

Para definir ```AlwaysEqual```, usamos a palavra-chave ```struct```, o nome que queremos e, em seguida, um ponto e vírgula. Não há necessidade de chaves ou parênteses! Então, podemos obter uma instância de ```AlwaysEqual``` na variável ```subject``` de forma semelhante: usando o nome que definimos, sem chaves ou parênteses. Imagine que mais tarde implementaremos um comportamento para esse tipo de forma que cada instância de ```AlwaysEqual``` seja sempre igual a cada instância de qualquer outro tipo, talvez para ter um resultado conhecido para fins de teste. Não precisaríamos de nenhum dado para implementar esse comportamento! 

## Propriedade dos dados da struct

Na definição da estrutura ```User``` .....

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

..... usamos o tipo ```String``` proprietário em vez do tipo slice de string ```&str```. Esta é uma escolha deliberada porque queremos que cada instância desta estrutura possua todos os seus dados e que esses dados sejam válidos por todo o tempo em que a estrutura inteira for válida.

Também é possível que structs armazenem referências a dados de propriedade de outra coisa, mas para fazer isso é necessário o uso de lifetimes. Lifetimes garantem que os dados referenciados por uma struct sejam válidos por tempo indeterminado. Digamos que você tente armazenar uma referência em uma struct sem especificar lifetimes, como o seguinte; isso não funcionará:

<table><tr>
<td><img src="images/error.png" width="48" alt="ERROR"></td>
<td>
<pre>
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
<br>
fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
</pre>
</td>
</tr></table> 
 
O compilador reclamará que ele precisa de especificadores de lifetime:

```
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` (bin "structs") due to 2 previous errors
```

## Exemplo do uso de struct 

Para entender quando podemos querer usar structs, vamos escrever um programa que calcula a área de um retângulo. Começaremos usando variáveis ​​simples e, em seguida, refatoraremos o programa até usarmos structs.

Vamos criar um novo projeto binário com **Cargo** chamado **rectangles** que receberá a largura e a altura de um retângulo especificadas em pixels e calculará a área do retângulo. O código abaixo mostra um programa curto com uma maneira de fazer exatamente isso no **src/main.rs** do nosso projeto.

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "A área do retângulo é {} pixels quadrados.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Agora, execute este programa usando ```cargo run```:

```
C:\Users\arataca89\Documents\rust\packages\rectangle>cargo run
   Compiling rectangle v0.1.0 (C:\Users\arataca89\Documents\rust\packages\rectangle)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.69s
     Running `target\debug\rectangle.exe`
A área do retângulo é 1500 pixels quadrados.
```

Este código consegue descobrir a área do retângulo chamando a função ```area()``` com cada dimensão, mas podemos fazer mais para tornar este código claro e legível.

O problema com este código é evidente na assinatura de ```area()```:

```
fn area(width: u32, height: u32) -> u32 {
```

A função ```area()``` deve calcular a área de um retângulo, mas a função que escrevemos tem dois parâmetros, e não está claro em nenhum lugar do nosso programa que os parâmetros estão relacionados. Seria mais legível e mais gerenciável agrupar largura(width) e altura(height). Esse agrupamento pode ser feito usando tuplas.

#### Refatorando com tuplas

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "A área do retângulo é {} pixels quadrados.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

De certa forma, este programa é melhor. Tuplas nos permitem adicionar um pouco de estrutura, e agora estamos passando apenas um argumento. Mas de outra forma, esta versão é menos clara: tuplas não nomeiam seus elementos, então temos que indexar nas partes da tupla, tornando nosso cálculo menos óbvio.

Misturar largura e altura não importaria para o cálculo da área, mas se quisermos desenhar o retângulo na tela, importaria! Teríamos que ter em mente que largura é o índice de tupla 0 e altura é o índice de tupla 1. Isso seria ainda mais difícil para outra pessoa descobrir e manter em mente se ela fosse usar nosso código. Como não transmitimos o significado de nossos dados em nosso código, agora é mais fácil introduzir erros.

#### Refatoração com Structs: adicionando mais significado 

Usamos structs para adicionar significado rotulando os dados. Podemos transformar a tupla que estamos usando em uma ```struct``` com um nome para a estrutura toda, bem como nomes para as partes, como mostrado abaixo.

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "A área do retângulo é {} pixels quadrados.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
``` 

Aqui, definimos uma ```struct``` e a nomeamos **Rectangle**. Dentro das chaves, definimos os campos como **width**(largura) e **height**(altura), ambos do tipo ```u32```. Então, em **main()**, criamos uma instância particular de **Rectangle** que tem uma largura de 30 e uma altura de 50.

Nossa função de área agora é definida com um parâmetro, que nomeamos **rectangle**, cujo tipo é um empréstimo imutável de uma instância da estrutura **Rectangle**. Queremos emprestar a estrutura em vez de assumir a propriedade dela. Dessa forma, **main()** mantém sua propriedade e pode continuar usando **rect1**, que é o motivo pelo qual usamos o **&** na assinatura da função e onde chamamos a função.

A função **area()** acessa os campos **width** e **height** da instância de **Rectangle** (note que acessar campos de uma instância struct emprestada não move os valores dos campos, e é por isso que você frequentemente vê empréstimos de structs). Nossa assinatura de função para **area()** agora diz exatamente o que queremos dizer: calcular a área de um **Rectangle**, usando seus campos **width**(largura) e **height**(altura). Isso transmite que **width** e **height** estão relacionados entre si, e dá nomes descritivos aos valores em vez de usar os valores de índice de tupla de 0 e 1. Isso é uma vitória para a clareza.

#### Adicionando funcionalidades com traits derivadas

Seria útil poder imprimir uma instância de **Rectangle** enquanto depuramos nosso programa e ver os valores de todos os seus campos. O código abaixo tenta usar o macro ```println!```. Isso não funcionará, no entanto.

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {}", rect1);
}
```

Quando compilamos este código, obtemos um erro com esta mensagem principal:

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

A macro ```println!``` pode fazer muitos tipos de formatação e, por padrão, as chaves dizem ao ```println!``` para usar a formatação conhecida como ```Display```: saída destinada ao consumo direto do usuário final. Os tipos primitivos que vimos até agora implementam ```Display``` por padrão porque há apenas uma maneira de mostrar o inteiro **1** ou qualquer outro tipo primitivo para um usuário. Mas com **structs**, a maneira como ```println!``` deve formatar a saída é menos clara porque há mais possibilidades de exibição: Você quer vírgulas ou não? Você quer imprimir as chaves? Todos os campos devem ser mostrados? Devido a essa ambiguidade, Rust não tenta adivinhar o que queremos, e <font color="blue">**structs não têm uma implementação fornecida de ```Display``` para usar com ```println!``` e o espaço reservado ```{}```**</font>. 

Se continuarmos lendo os erros, encontraremos esta nota útil:

```
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Vamos tentar! A chamada de macro ```println!``` agora ficará assim:

```
println!("rect1: {rect1:?}");
```

Colocar o especificador **:?** dentro das chaves informa ao ```println!``` que queremos usar um formato de saída chamado ```Debug```. A trait ```Debug``` nos permite imprimir nossa estrutura de uma forma útil para desenvolvedores, para que possamos ver seu valor enquanto depuramos nosso código.

Compile o código com essa mudança. Droga! Ainda recebemos um erro: 

```
error[E0277]: `Rectangle` doesn't implement `Debug`
```

Mas, novamente, o compilador nos dá uma nota útil:

```
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust inclui funcionalidade para imprimir informações de depuração, mas temos que optar explicitamente por tornar essa funcionalidade disponível para nossa estrutura. Para fazer isso, adicionamos o atributo externo:

```
#[derive(Debug)]
```

Logo antes da definição da estrutura, como mostrado abaixo.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {rect1:?}");
}
```

Agora, quando executarmos o programa, não teremos nenhum erro e veremos a seguinte saída: 

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1: Rectangle { width: 30, height: 50 }
```

Ótimo! Não é a saída mais bonita, mas mostra os valores de todos os campos para esta instância, o que definitivamente ajudaria durante a depuração. Quando temos structs maiores, é útil ter uma saída que seja um pouco mais fácil de ler; nesses casos, podemos usar **{:#?}** em vez de **{:?}** na string fornecida a ```println!```. Neste exemplo, usar o estilo **{:#?}** irá gerar a seguinte saída:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1: Rectangle {
    width: 30,
    height: 50,
}
```

Outra maneira de imprimir um valor usando o formato ```Debug``` é usar a macro ```dbg!```, que assume a propriedade de uma expressão (ao contrário de ```println!```, que recebe uma referência), imprime o arquivo e o número da linha de onde essa chamada de macro ```dbg!``` ocorre em seu código junto com o valor resultante dessa expressão e retorna a propriedade do valor. 

**Observação**: Chamar a macro ```dbg!``` imprime para o fluxo de console de erro padrão (**stderr**), ao contrário de ```println!```, que imprime para o fluxo de console de saída padrão (**stdout**).

Aqui está um exemplo onde estamos interessados no valor que é atribuído ao campo **width**, bem como no valor da estrutura inteira em **rect1**:

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

Podemos colocar ```dbg!``` recebendo a expressão **30 * scale** porque ```dbg!``` retorna a propriedade do valor da expressão, o campo **width** receberá o mesmo valor como se não tivéssemos a chamada ```dbg!```. Porém, não queremos que ```dbg!``` assuma a propriedade de **rect1**, então usamos uma referência a **rect1** na próxima chamada. Aqui está como a saída deste exemplo se parece:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

Podemos ver que o primeiro pedaço de saída veio de **src/main.rs** linha 10, onde estamos depurando a expressão **30 * scale**, e seu valor resultante é **60** (a formatação de depuração implementada para inteiros é imprimir apenas seu valor). A chamada ```dbg!``` na linha 14 de **src/main.rs** imprime o valor de **&rect1**, que é a estrutura **Rectangle**. Esta saída usa a formatação de depuração bonita do tipo **Rectangle**. A macro ```dbg!``` pode ser realmente útil quando você está tentando descobrir o que seu código está fazendo!

Além da trait ```Debug```, Rust forneceu uma série de traits para usarmos com o atributo **derive** que podem adicionar comportamento útil aos nossos tipos personalizados. Essas traits e seus comportamentos podem ser vistas [aqui](derivable_traits.md#arataca89).  Existem também muitos atributos além de **derive**; para mais informações, consulte a seção [Atributos](https://doc.rust-lang.org/reference/attributes.html) da Referência Rust.

Nossa função ```area()``` é muito específica: ela só calcula a área de retângulos. Seria útil vincular esse comportamento mais estreitamente à nossa estrutura **Rectangle**  porque ele não funcionará com nenhum outro tipo. Vamos ver como podemos continuar a refatorar esse código transformando a função de área em um método de área definido no nosso tipo **Rectangle**.

## Métodos

Métodos são semelhantes a funções: nós os declaramos com a palavra-chave **fn** e um nome, eles podem ter parâmetros e um valor de retorno, e eles contêm algum código que é executado quando o método é chamado de outro lugar. Diferentemente das funções, os métodos são definidos dentro do contexto de uma ```struct``` (ou um ```enum``` ou um objeto ```trait```), e seu primeiro parâmetro é sempre **self**, que representa a instância da ```struct``` em que o método está sendo chamado.

#### Definindo métodos

Vamos mudar a função ```area()``` que recebe uma instância de ```Rectangle``` como parâmetro e, em vez disso, criar um método ```area()``` definido na estrutura ```Rectangle```.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "A área do retângulo é {} square pixels.",
        rect1.area()
    );
}
```

Para definir a função dentro do contexto de **Rectangle**, iniciamos um bloco **impl** (implementação) para **Rectangle**. Tudo dentro deste bloco **impl** será associado ao tipo **Rectangle**. Então movemos a função ```area()``` para dentro das chaves de **impl** e alteramos o primeiro (e neste caso, único) parâmetro para ```self``` na assinatura e em todos os lugares dentro do corpo. Em ```main()```, onde chamamos a função ```area()``` e passamos **rect1** como argumento, podemos usar a sintaxe do método para chamar o método ```area()``` em nossa instância de **Rectangle**. A sintaxe do método vai depois de uma instância: adicionamos um ponto seguido pelo nome do método, parênteses e quaisquer argumentos.

Na assinatura do método ```area()``` usamos **&self** em vez de **rectangle: &Rectangle**. O **&self** é, na verdade, uma abreviação de **self: &Self**. Dentro de um bloco **impl**, o tipo ```Self``` é um alias para o tipo para o qual o bloco **impl** está sendo implementado. <font color="blue">Os métodos devem ter um parâmetro chamado **self** do tipo **Self** como seu primeiro parâmetro; Rust permite que você abrevie isso com apenas o nome **self** como primeiro parâmetro.</font> Observe que ainda precisamos usar o **&** na frente da abreviação **self** para indicar que este método pega emprestado a instância **Self**, assim como fizemos em **rectangle: &Rectangle**. Os métodos podem assumir a propriedade de **self**, pegar um empréstimo imutável de **self**, como fizemos aqui, ou pegar um empréstimo mutável de **self**, assim como podem fazer com qualquer outro parâmetro. 

Escolhemos **&self** aqui pelo mesmo motivo que usamos **&Rectangle** na versão da função: não queremos assumir a propriedade, e queremos apenas ler os dados na ```struct```, não escrever nela. 

Se quiséssemos alterar a instância na qual chamamos o método como parte do que o método faz, usaríamos **&mut self** como primeiro parâmetro. 

Um método que assume a propriedade da instância usando apenas **self** como primeiro parâmetro é raro; essa técnica é geralmente usada quando o método transforma **self** em outra coisa e você quer evitar que o chamador use a instância original após a transformação.

A principal razão para usar métodos em vez de funções, além de fornecer a sintaxe de método e não ter que repetir o tipo de **self** em cada assinatura de método, é para organização. Colocamos todas as coisas que podemos fazer com uma instância de um tipo em um bloco **impl**, em vez de fazer com que os usuários futuros do nosso código procurem por funcionalidades do tipo **Rectangle** em vários lugares na biblioteca que fornecemos.

Observe que podemos escolher dar a um método o mesmo nome que um dos campos da estrutura. Por exemplo, podemos definir um método em **Rectangle** que também se chama **width**:

```
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("O retângulo tem largura diferente de zero; seu valor é {}", rect1.width);
    }
}
```

Aqui, estamos escolhendo fazer o método **width** retornar ```true``` se o valor no campo **width** da instância for maior que 0 e ```false``` se o valor for 0: podemos usar um campo dentro de um método com o mesmo nome para qualquer finalidade. Em ```main()```, quando colocamos parênteses após **rect1.width**, Rust sabe que estamos nos referindo ao método **width()**. Quando não usamos parênteses, Rust sabe que queremos dizer o campo **width**.

Frequentemente, mas não sempre, quando damos a um método o mesmo nome de um campo, queremos que ele apenas retorne o valor no campo e não faça mais nada. Métodos como este são chamados de **getters**, e Rust não os implementa automaticamente para campos de ```struct``` como algumas outras linguagens fazem. Getters são úteis porque você pode tornar o campo privado, mas o método público, e assim permitir acesso somente leitura a esse campo como parte da API pública do tipo.

#### Métodos com mais parâmetros

Vamos praticar o uso de métodos implementando um segundo método na estrutura **Rectangle**. Desta vez, queremos que uma instância de **Rectangle** receba outra instância de **Rectangle** e retorne ```true``` se o segundo **Rectangle** pode caber completamente dentro de **self** (o primeiro **Rectangle**); caso contrário, deve retornar ```false```. Ou seja, depois de definirmos o método ```can_hold()``` queremos poder escrever o programa abaixo:

```
 fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 pode conter rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 pode conter rect3? {}", rect1.can_hold(&rect3));
}
```

A saída esperada seria semelhante ao seguinte porque as dimensões de **rect2** são menores que as dimensões de **rect1**, mas **rect3** é maior que **rect1**:

```
rect1 pode conter rect2? true
rect1 pode conter rect3? false
```

Sabemos que queremos definir um método, então ele estará dentro do bloco **impl Rectangle**. O nome do método será **can_hold**, e ele pegará um empréstimo imutável de outro **Rectangle** como parâmetro. Podemos dizer qual será o tipo do parâmetro olhando para o código que chama o método: **rect1.can_hold(&rect2)** passa **&rect2**, que é um empréstimo imutável para **rect2**, uma instância de **Rectangle**. Isso faz sentido porque precisamos apenas ler **rect2** (em vez de escrever, o que significaria que precisaríamos de um empréstimo mutável), e queremos que **main** retenha a propriedade de **rect2** para que possamos usá-lo novamente após chamar o método **can_hold**. O valor de retorno de **can_hold** será um booleano, e a implementação verificará se a largura e a altura de **self** são maiores que a largura e a altura do outro **Rectangle**, respectivamente. 

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Quando executarmos este código conforme a chamada em ```main()```, obteremos a saída desejada.

Os métodos podem receber vários parâmetros que adicionamos à assinatura após o parâmetro **self**, e esses parâmetros funcionam como parâmetros em funções.

#### Funções associadas

Todas as funções definidas dentro de um bloco **impl** são chamadas de funções associadas porque estão associadas ao tipo nomeado após o **impl**. Podemos definir funções associadas que não têm **self** como seu primeiro parâmetro (e, portanto, não são métodos) porque elas não precisam de uma instância do tipo para funcionar. Já usamos uma função como esta: a função ```String::from()``` que é definida no tipo ```String```.

Funções associadas que não são métodos são frequentemente usadas para construtores que retornarão uma nova instância da estrutura. Essas funções são frequentemente chamadas de **new**, mas **new** não é um nome especial e não é integrado à linguagem. Por exemplo, poderíamos optar por fornecer uma função associada chamada **square** que teria um parâmetro de dimensão e usaria isso como largura e altura, tornando mais fácil criar um **Rectangle** quadrado em vez de ter que especificar o mesmo valor duas vezes:

```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

As palavras-chave **Self** no tipo de retorno e no corpo da função são alias para o tipo que aparece após a palavra-chave **impl**, que neste caso é **Rectangle**.

Para chamar essa função associada, usamos a sintaxe ```::``` com o nome da estrutura; por exemplo:

```
let sq = Rectangle::square(3);
```

Essa função é chamada usando o espaço de nome, namespace, da estrutura: a sintaxe ```::``` é usada para funções associadas e namespaces (espaços de nomes) criados por módulos.

#### Vários blocos impl

Cada ```struct``` pode ter múltiplos blocos **impl**. Por exemplo, o código abaixo é válido e faz a mesma coisa que o código anterior com os dois métodos no mesmo bloco **impl**.

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Não há razão para separar esses métodos em vários blocos **impl** aqui, mas essa é uma sintaxe válida.

#### Conclusão

**Structs** permitem que você crie tipos personalizados que sejam significativos para seu domínio. Ao usar **structs**, você pode manter partes associadas de dados conectadas umas às outras e nomear cada parte para deixar seu código claro. Em blocos **impl**, você pode definir funções que são associadas ao seu tipo, e métodos são um tipo de função associada que permite que você especifique o comportamento que as instâncias de suas **structs** têm.

---

## Referências
[Capítulo 5 do Livro](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

arataca89@gmail.com

Última atualização: 20241225
