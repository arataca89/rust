# Linguagem Rust - Genéricos

Toda linguagem de programação tem ferramentas para efetivamente tratar a duplicação de conceitos e códigos. Em Rust, uma destas ferramentas são os genéricos. Como o próprio nome já indica, o tipo genérico em um código significa que este código poderá ser executado com qualquer tipo concreto e funcionará do mesmo jeito.

[1. Evitando a duplicação de código usando uma função](#1-Evitando-a-duplicação-de-código-usando-uma-função)

[2. Usando genéricos para que nossa função aceite qualquer tipo de dados](#2-Usando-genéricos-para-que-nossa-função-aceite-qualquer-tipo-de-dados)

[3. Usando genéricos em structs](#3-Usando-genéricos-em-structs)

[4. Usando genéricos em enums](#4-Usando-genéricos-em-enums)

[5. Usando genéricos em métodos](#5-Usando-genéricos-em-métodos)

[6. Usando parâmetros genéricos diferentes em métodos](#6-Usando-parâmtros-genéricos-diferentes-em-métodos)

[7. Custo de performance usando genéricos](#7-Custo-de-performance-usando-genéricos)

---

## 1. Evitando a duplicação de código usando uma função

Dada a função abaixo, que encontra o maior valor em uma lista de inteiros.

```
fn main() {
    let numeros = vec![34, 50, 25, 100, 65];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 100
}

```
Apesar deste código funcionar, se quisermos procurar o maior valor em outra lista de inteiros no mesmo programa, teremos que fazer algo semelhante a:

```
fn main() {
    let numeros = vec![34, 50, 25, 100, 65];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 100

    let numeros = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 6000
}

```
Note que houve duplicação de código e isto é cansativo e propenso a erros. Outra grande desvantagem da duplicação de código é no momento de realizar alguma correção ou alteração. Você terá que alterar seu código em vários locais e esquecer de um deles é bem comum.

Uma maneira de evitar esta duplicação de código seria criar uma função que recebe uma lista de inteiros e retorna seu maior valor. Esta solução torna nosso código mais claro e nos permite expressar o conceito de encontrar o maior número em uma lista de forma abstrata. 


```
fn maior(slice_int: &[i32]) -> &i32 {

    let mut maior_valor = &slice_int[0];

    for n in slice_int {
        if n > maior_valor {
            maior_valor = n;
        }
    }

    maior_valor
}

fn main() {
    let v1 = vec![34, 50, 25, 100, 65];

    let x = maior(&v1);

    println!("O maior valor é {x}"); // 100

    let v2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let x = maior(&v2);

    println!("O maior valor é {x}"); // 6000
}

```
 
 ## 2. Usando genéricos para que nossa função aceite qualquer tipo de dados
 
E se quisermos uma função que retorne o maior valor em um slice de char, ou em um slice de f64, ou em um slice de qualquer outro tipo de dados ??? Uma solução seria escrever outra função similar a nossa função ```maior()``` usando outro tipo de dados, mas isto também caracterizaria duplicação de código. O recurso dos genéricos pode ser utilizado nestes casos. 
 
Genéricos são usados para criar funções ou structs que podem ser usadas com muitos tipos de dados concretos diferentes. Abaixo temos a implementação da função ```maior()``` usando genéricos.

```
fn maior<T>(slice_t: &[T]) -> &T {
    let mut maior_valor = &slice_t[0];

    for n in slice_t {
        if n > maior_valor {
            maior_valor = n;
        }
    }

    maior_valor
}

fn main() {
    let v1 = vec![34, 50, 25, 100, 65];

    let x = maior(&v1);

    println!("O maior valor é {x}"); // 100

    let v2 = vec!['a','s','d','f','g'];

    let x = maior(&v2);

    println!("O maior valor é {x}"); // s
}
```
Ao ser compilado este código emitirá a seguinte mensagem de erro:
```
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:7:14
  |
7 |         if n > maior_valor {
  |            - ^ ----------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
3 | fn maior<T: std::cmp::PartialOrd>(slice_t: &[T]) -> &T {
  |           ++++++++++++++++++++++
```
Nas mensagens de erro exibidas, ```std::cmp::PartialOrd``` é uma ```trait```. Esta mensagem significa que a função ```maior()``` não funcionará para todos os tipos possíveis que ```T``` poderá assumir. Como queremos comparar valores do tipo ```T```, podemos usar apenas tipos cujos valores podem ser ordenados. Para habilitar comparações, a biblioteca padrão Rust tem a trait ```std::cmp::PartialOrd``` que você pode implementar em seus tipos (mais informações em [https://doc.rust-lang.org/beta/std/cmp/trait.PartialOrd.html](https://doc.rust-lang.org/beta/std/cmp/trait.PartialOrd.html)). 

Note que o compilador emite um ```help``` onde ele considera restringir  os tipos válidos para ```T``` somente aos tipos que implementam ```PartialOrd``` e este exemplo será compilado. Isto acontece porque a biblioteca padrão implementa ```PartialOrd``` em ```i32``` e ```char```, que são os tipos utilizados neste exemplo.

Seguindo a dica do compilador, nossa função ```maior()``` ficará assim:
```
//fn maior<T>(slice_T: &[T]) -> &T {
fn maior<T: std::cmp::PartialOrd>(slice_t: &[T]) -> &T {
    let mut maior_valor = &slice_t[0];

    for n in slice_t {
        if n > maior_valor {
            maior_valor = n;
        }
    }

    maior_valor
}
```
E agora o código compilará beleza.

Observe a sintaxe que indica que o tipo ```T``` implementa a trait ```PartialOrd```.

```
<T: std::cmp::PartialOrd>
```

É importante também notar que o identificador do parâmero do tipo genérico,  ```T``` neste caso, pode ser qualquer identificador válido, mas por convenção usa-se uma letra maiúscula (T, V, U, etc...).


 ## 3. Usando genéricos em structs
 
 ```
 struct Point<T> {
    x: T,
    y: T,
}

fn print<T: std::fmt::Display>(p: Point<T>){
    println!("{{{},{}}}",p.x,p.y);
}

fn main() {
    let ponto_int = Point { x: 5, y: 10 };
    let ponto_float = Point { x: 1.0, y: 4.0 };

    print(ponto_int);
    print(ponto_float);
}
```

Observe que neste caso a função ```print()``` aceitará tipos que implementam a trait ```std::fmt::Display```. Se não fizermos isso, a compilação emitirá um erro solicitando esta implementação.

```
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:8:26
  |
8 |     println!("{{{},{}}}",p.x,p.y);
  |                          ^^^ `T` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
  |
6 | fn print<T: std::fmt::Display>(p: Point<T>){
  |           +++++++++++++++++++
```
 
## 4. Usando genéricos em enums
 
Assim como nas structs, as enums podem ter tipos genéricos em suas variantes. Observe abaixo uma implementação da enum ```Option<T>``` que faz parte da biblioteca padrão da linguagem Rust.

```
enum Option<T> {
    Some(T),
    None,
}
```

Note que esta enum possui um parâmetro genérico chamado ```T``` e duas variantes:

* ```Some``` que tem um valor; e
* ```None``` que não tem nenhum valor.


Ao usar  ```Option<T>```, podemos expressar o conceito abstrato de um valor opcional e, como ```Option<T>``` é genérico, podemos usar essa abstração independentemente do tipo concreto de dados a ser utilizado.

Outro exemplo da utilização de genéricos na biblioteca Rust é a enum ```Result```.

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

A enum ```Result``` possui duas variantes:

* ```Ok``` que tem o parâmetro genérico ```T```; e
* ```Err``` que tem o parâmtero genérico ```E```

Note que neste caso usamos dois tipos genéricos, indicando que em ```Result``` podemos usar dois tipos concretos diferentes.

```Result``` expressa o conceito abstrato de uma operação que pode ser bem sucedida e retornar um valor de algum tipo ```T``` ou mal sucedida e retornar um valor do tipo ```E```, que significa erro.

## 5. Usando genéricos em métodos

```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let p = Point{x: 5, y: 10};

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p2 = Point{x: 2.0, y: 3.0};

    println!("Distância de p2 até a origem: {}", p2.distance_from_origin());
}
```
Observe que você pode implementar métodos para qualquer tipo concreto e pode também implementar métodos para determinados tipos, como foi feito acima no método ```distance_from_origin()``` o 	qual só atua em ```Point``` do tipo ```f32```.

Este código significa que o tipo ```Point<f32>``` terá um método ```distance_from_origin()```; outras instâncias de ```Point<T>``` onde ```T``` não é do tipo ```f32``` não terão esse método definido.


## 6. Usando parâmetros genéricos diferentes em métodos

```
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}
```

Neste exemplo, observe que ```p1``` foi criado com ```x``` sendo um ```i32``` com valor 5; e ```y``` sendo um ```f64``` com valor 10.4.

```p2```, por sua vez, foi criado com ```x``` sendo um ```&str``` (slice de string) com valor "Hello"; e ```y``` sendo um ```char``` com valor 'c'. 

Como o método ```mixup()``` foi declarado com outros parâmetros genéricos podemos misturar os tipos criando um ```Point``` com o ```x``` de ```p1``` e o ```y``` de ```p2```.


## 7. Custo de performance usando genéricos 

Você pode estar se perguntando se há algum custo no tempo de execução ao usar parâmetros genéricos. A boa notícia é que usar tipos genéricos não fará seu programa rodar mais devagar do que faria com tipos concretos.

Rust faz isso realizando a monomorfização do código usando genéricos em tempo de compilação. Monomorfização é o processo de transformar código genérico em código específico substituindo os parâmetros genéricos por tipos concretos que são usados no momento da compilação. 

O Rust reescreve o código substituindo os parâmetros genéricos com os tipos concretos e só depois executa a compilação, então não há custo em tempo de execução. Quando o código é executado, ele funciona como se nós tivéssemos escrito as structs, funções e métodos com os tipos concretos. 

Este processo torna o uso de genéricos em Rust extremamente prático e eficiente em tempo de execução.

---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

arataca89@gmail.com

Última atualização: 20240910
