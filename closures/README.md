# Linguagem Rust - closures

Closures em Rust são funções anônimas que você pode salvar numa variável ou passar como argumento para outras funções. Você pode criar uma closure em um lugar e chamá-la em outro para avaliá-la em outro contexto. Diferente das funções tradicionais, closures podem capturar valores do escopo onde são definidas. Os recursos das closures permitem reutilização de código e customização de comportamento.

[1. Capturando o ambiente com closures](#1-Capturando-o-ambiente-com-closures)

[2. Inferência de tipos e anotações na closure](#2-Inferência-de-tipos-e-anotações-na-closure)

[3. Capturando referências ou movendo propriedade](#3-Capturando-referências-ou-movendo-propriedade)

[4. Movendo valores capturados para fora da closure e traits fn](#4-Movendo-valores-capturados-para-fora-da-closure-e-traits-fn)

---

## 1. Capturando o ambiente com closures

Closures podem capturar valores do ambiente onde foram definidas para uso posterior.

Para exemplificar o uso deste recurso foi criado um cenário onde uma certa empresa que comercializa camisetas faz a seguinte promoção: será sorteada uma camiseta exclusiva entre as pessoas que se inscreverem na lista para receber emails promocionais da empresa. No momento da inscrição a pessoa pode também declarar sua cor favorita. Se a pessoa sorteada tiver declarado sua cor favorita, ganhará a camiseta dessa cor. Senão ganhará uma camiseta da cor que a empresa mais tenha no momento.

Há muitas maneiras de implementar isso. Para este exemplo, usaremos uma enumeração chamada ```ShirtColor``` que tem as variantes ```Red``` e ```Blue```. Representamos o estoque da empresa com uma estrutura chamada ```Inventory``` que tem um campo chamado ```shirts``` que contém um ```Vec<ShirtColor>``` representando as cores das camisas atualmente em estoque. O método ```giveaway()``` definido em ```Inventory``` obtém a preferência de cor de camisa do ganhador da camisa grátis e retorna a cor de camisa que a pessoa receberá.

```
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

A variável ```store``` definida na função ```main()``` tem duas camisas azuis)Blue) e uma camisa vermelha(Red) restantes para distribuir nesta promoção de edição limitada. Chamamos o método ```giveway()```para um usuário com preferência por uma camisa vermelha e um usuário sem nenhuma preferência. 

Novamente, esse código pode ser implementado de muitas maneiras, e aqui, para focar em closures, nós nos ativemos aos conceitos que você já aprendeu, exceto pelo corpo do método ```giveaway()``` que usa uma closure.

No método ```giveaway()```, obtemos a preferência do usuário como um parâmetro do tipo ```Option<ShirtColor>```, chamado ```user_preference```, e chamamos o método ```unwrap_or_else()``` a partir deste parâmetro.

O método ```unwrap_or_else()``` em ```Option<T>``` é definido pela biblioteca padrão. Ele recebe um argumento: um closure sem argumentos que retorna um valor ```T``` (o mesmo tipo armazenado na variante ```Some``` de ```Option<T>```, neste caso ```ShirtColor```). Se ```Option<T>``` for a variante ```Some```, ```unwrap_or_else()``` retorna o valor armazenado dentro de ```Some```. Se ```Option<T>``` for a variante ```None```, ```unwrap_or_else()``` chama a closure e retorna o valor retornado pela closure.

Especificamos a closure ```|| self.most_stocked()``` como argumento de ```unwrap_or_else()```. Esta closure não recebe nenhum parâmetro (se a closure tivesse parâmetros, eles apareceriam entre as duas barras verticais). O corpo da closure chama ```self.most_stocked()```. Estamos definindo a closure aqui, e a implementação de ```unwrap_or_else()``` avaliará a closure mais tarde se for necessário. 

Ao compilar este código teremos a seguinte saída:

```
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

Um aspecto interessante aqui é que passamos uma closure que chama ```self.most_stocked()``` na instância de ```Inventory```. A biblioteca padrão não precisou saber nada sobre os tipos ```Inventory``` ou ```ShirtColor``` que definimos, ou a lógica que queríamos usar neste cenário. A closure captura uma referência imutável para a instância ```self Inventory``` e a passa com o código que especificamos para o método ```unwrap_or_else()```. As funções, por outro lado, não conseguem capturar seu ambiente dessa forma.

## 2. Inferência de tipos e anotações na closure

Existem mais diferenças entre funções e closures. Closures geralmente não exigem que você anote os tipos dos parâmetros ou o valor de retorno como as funções o fazem. Anotações de tipo são necessárias em funções porque os tipos são parte de uma interface explícita exposta aos seus usuários. Definir essa interface rigidamente é importante para garantir que todos concordem sobre quais tipos de valores uma função usa e retorna. Closures, por outro lado, não são usados ​​em uma interface exposta como esta: elas são armazenadas em variáveis ​​e usadas ​​sem nomeá-las e expô-las aos usuários da nossa biblioteca.

Closures geralmente são curtas e relevantes apenas dentro de um pequeno  contexto, em vez de em qualquer cenário arbitrário. Dentro desses contextos limitados, o compilador pode inferir os tipos dos parâmetros e o tipo de retorno, semelhante a como ele é capaz de inferir os tipos da maioria das variáveis (existem casos raros em que o compilador também precisa de anotações de tipo na closure). 

Assim como fazemos com as variáveis, podemos adicionar anotações de tipo se quisermos aumentar a legibilidade e clareza ao custo de sermos mais verbosos do que estritamente necessário. Anotar os tipos para uma closure seria parecido com o exemplo abaixo. Neste exemplo, estamos definindo uma closure e armazenando-a em uma variável em vez de definir a closure no local onde a passamos como argumento como fizemos no exemplo anterior. 

```
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

Com anotações de tipo adicionadas, a sintaxe das closures se parece mais com a sintaxe das funções. 

Aqui definimos uma função que adiciona 1 ao seu parâmetro e um closure que tem o mesmo comportamento, para comparação. Adicionamos alguns espaços para alinhar as partes relevantes. Isso ilustra como a sintaxe de closure é semelhante à sintaxe de função, exceto pelo uso de pipes e a quantidade de informação passada que é opcional: 

```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

A primeira linha mostra uma definição de função, e a segunda linha mostra uma definição de closure totalmente anotada. Na terceira linha, removemos as anotações de tipo da definição de closure. Na quarta linha, removemos os colchetes, que são opcionais porque o corpo da closure apenas uma expressão.Todas essas são definições válidas que produzirão o mesmo comportamento quando forem chamadas. As linhas ```add_one_v3``` e ```add_one_v4``` exigem que os fechamentos sejam avaliados para poder compilar porque os tipos serão inferidos de seu uso. Isso é semelhante a ```let v = Vec::new();``` precisando que anotações de tipo ou valores de algum tipo sejam inseridos no ```Vec``` para que o Rust possa inferir o tipo.

Para definições de closure, o compilador inferirá um tipo concreto para cada um de seus parâmetros e para seu valor de retorno. Por exemplo, o código abaixo mostra a definição de uma closure que apenas retorna o valor que recebe como parâmetro. Esta closure não é muito útil, exceto para os propósitos deste exemplo. Observe que não adicionamos nenhuma anotação de tipo à definição. Como não há anotações de tipo, podemos chamar a closure com qualquer tipo, o que fizemos aqui com String pela primeira vez. Se tentarmos depois chamar ```example_closure``` com um inteiro, obteremos um erro.

```
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

Ao tentar compilar este código recebemos o seguinte erro:

```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` (bin "closure-example") due to 1 previous error
```

Na primeira vez que chamamos ```example_closure``` com o valor String, o compilador infere o tipo de ```x``` e o tipo de retorno da closure como String. Esses tipos são então bloqueados na closure em ```example_closure```, e obtemos um erro de tipo quando tentamos usar um tipo diferente com a mesma closure.

 
## 3. Capturando referências ou movendo propriedade

Closures podem capturar valores de seu ambiente de três maneiras, que se correlacionam diretamente com as três maneiras pelas quais uma função pode receber um parâmetro: empréstimo imutável, empréstimo mutável e aquisição de propriedade. A closure decidirá qual dessas usar com base no que o corpo da função faz com os valores capturados.

No código abaixo definimos uma closure que captura uma referência imutável de vetor chamada ```list``` porque ele só precisa de uma referência imutável para imprimir o valor:

```
fn main() {
    let list = vec![1, 2, 3];
    println!("Antes de definir a closure: {list:?}");

    let only_borrows = || println!("Dentro da closure: {list:?}");

    println!("Antes de chamar a closure: {list:?}");
    only_borrows();
    println!("Após chamar a closure: {list:?}");
}
```

Este exemplo também ilustra que uma variável pode estar vinculada a uma definição de closure, e podemos mais tarde chamar a closure usando o nome da variável e parênteses como se o nome da variável fosse um nome de função. 

Como podemos ter múltiplas referências imutáveis para a variável ```list``` ao mesmo tempo, ```list``` é acessível em todas as situações do código acima, seja antes da definição da closure, após a definição da closure, mas antes da closure ser chamada, e após a closure ser chamada. Este código compila e executa beleza:

```
Antes de definir a closure: [1, 2, 3]
Antes de chamar a closure: [1, 2, 3]
Dentro da closure: [1, 2, 3]
Após chamar a closure: [1, 2, 3]
```

Em seguida, alteramos o corpo da closure para que ela adicione um elemento ao vetor ```list```. A closure agora captura uma referência mutável: 

```
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Antes de definir a closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("Após chamar a closure: {list:?}");
}
```

Este código compila a roda beleza:

```
Antes de definir a closure: [1, 2, 3]
Após chamar a closure: [1, 2, 3, 7]
```

Observe que não há mais um ```println!``` entre a definição e a chamada da closure ```borrows_mutably```: quando ```borrows_mutably``` é definida, ela captura uma referência mutável para ```list```. Não usamos a closure novamente depois que ela é chamada, então o empréstimo mutável termina. Entre a definição e a chamada da closure, um empréstimo imutável para imprimir não é permitido porque nenhum outro empréstimo é permitido quando há um empréstimo mutável. Tente adicionar um ```println!``` nesta posição e você receberá uma mensagem de erro ao compilar.

Se você quiser forçar que a closure assuma a propriedade dos valores que ele usa do ambiente, mesmo que o corpo da closure não precise estritamente de propriedade, você pode usar a palavra-chave ```move``` antes da lista de parâmetros.

Essa técnica é mais útil quando você passa uma closure para uma nova thread a fim de mover os dados de modo que sejam de propriedade da nova thread. O exemplo abaixo modifica o código que imprime o vetor ```list``` para imprimir o vetor em uma nova thread em vez da thread principal:

```
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Antes de definir a closure: {list:?}");

    thread::spawn(move || println!("Dentro da closure: {list:?}"))
        .join()
        .unwrap();
}
```

Criamos uma nova thread, dando a ela uma closure para ser executada como argumento. O corpo do closure imprime ```list```. Na implementação anterior, a closure apenas capturou ```list``` usando uma referência imutável porque essa é a menor quantidade de acesso a ```list``` necessária para imprimi-la (princípio do menor privilégio). Neste exemplo, mesmo que o corpo da closure ainda precise apenas de uma referência imutável, precisamos especificar que a variável ```list``` deve ser movida para a closure colocando a palavra-chave ```move``` no início da definição da closure. A nova thread pode terminar antes da thread principal, ou a thread principal pode terminar primeiro. Se a thread principal mantivesse a propriedade de ```list```, mas terminasse antes da nova thread e eliminasse ```list```, a referência imutável na thread seria inválida. Portanto, o compilador requer que ```list``` seja movida para a closure de modo que a closure dentro da nova thread tenha sempre uma referência válida. Tente remover a palavra-chave ```move``` ou usar ```list``` na thread principal após a closure ser definida e você receberá erros do compilador.

## 4. Movendo valores capturados para fora da closure e traits fn

Uma vez que a closure tenha capturado uma referência ou a propriedade de um valor do ambiente onde foi definida, o código no corpo da closure define o que acontece com as referências ou valores. A closure pode: mover um valor capturado para fora, alterar o valor capturado, não mover nem mudar o valor ou não capturar nada do ambiente.

A forma como uma closure captura e manipula valores do ambiente afeta quais traits a closure implementa, e traits são como funções e structs podem especificar quais tipos de closures elas podem usar. Closures implementarão automaticamente uma, duas ou todas as três traits Fn, de forma aditiva, dependendo de como o corpo da closure manipula os valores:

* ```FnOnce``` se aplica a closures que podem ser chamadas uma vez. Todas as closures implementam pelo menos essa trait, porque todas as closures podem ser chamados. Uma closure que move valores capturados para fora de seu corpo implementará apenas ```FnOnce``` e nenhuma das outras traits Fn, porque ela só pode ser chamado uma vez.
* ```FnMut``` se aplica a closures que não movem valores capturados para fora de seu corpo, mas que podem mudar os valores capturados. Essas closures podem ser chamadas mais de uma vez.
* ```Fn``` se aplica a closures que não movem valores capturados para fora de seu corpo e que não mudam valores capturados, bem como closures que não capturam nada de seu ambiente. Essas closures podem ser chamadas mais de uma vez sem mudar seu ambiente, o que é importante em casos como chamar uma closure várias vezes concorrentemente (simultaneamente). 

Vamos analisar a definição do método ```unwrap_or_else()``` em ```Option<T>```.

 

asd

---
## Referências

[capítulo 13 do "Livro"](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

arataca89@gmail.com

Última atualização: 20241002
