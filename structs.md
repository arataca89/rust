# Linguagem Rust - estruturas

Uma ```struct```, ou estrutura, é um tipo de dado criado pelo programador que permite encapsular vários valores relacionados de modo que o tipo de dado criado melhore o significado do seu uso e a compreensão das diversas partes do programa. Se você estiver familiarizado com uma linguagem orientada a objetos, uma ```struct``` é como os atributos de dados de um objeto. 

* [Definindo e instanciando estruturas](#definindo-e-instanciando-estruturas)

* [Usando a sintaxe abreviada para inicializar os campos da struct](#usando-a-sintaxe-abreviada-para-inicializar-os-campos-da-struct)

* [Criando instâncias a partir de outras instâncias](#criando-instâncias-a-partir-de-outras-instâncias)

* [Estruturas de tupla](#estruturas-de-tupla)

* [Estruturas sem campos](#estruturas-sem-campos)

* [Propriedade dos dados da struct](#propriedade-dos-dados-da-struct)

* [Exemplo do uso de struct](#exemplo-do-uso-de-struct)

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

asd


---

## Referências
[Capítulo 5 do Livro](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

arataca89@gmail.com

Última atualização: 20241202
