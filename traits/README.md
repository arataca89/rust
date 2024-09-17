# Linguagem Rust - Traits

Uma ```trait``` define uma funcionalidade que um tipo de dados deverá implementar.

```trait bounds``` são usadas para especificar que um tipo genérico pode ser de qualquer tipo concreto que tenha determinado comportamento, ou seja, que implemente determinada ```trait```.

Traits são semelhantes ao recurso chamado de interfaces em outras linguagens.

[1. Definindo uma trait](#1-Definindo-uma-trait)

[2. Implementando uma trait](#2-Implementando-uma-trait)

[3. Limitação na implementação de uma trait](#3-Limitação-na-implementação-de-uma-trait)

[4. Implementação padrão em métodos de uma trait](#4-Implementação-padrão-em-métodos-de-uma-trait)

[5. Trait como parâmetro](#5-Trait-como-parâmetro-de-função)

[6. Retornando tipos que implementam uma trait](#6-Retornando-tipos-que-implementam-uma-trait)

[7. Usando trait bounds para implementar métodos condicionamente](#7-Usando-trait-bounds-para-implementar-métodos-condicionamente)

---

## 1. Definindo uma trait

O comportamento de um tipo consiste nos métodos que podemos chamar a partir de um objeto desse tipo. Diferentes tipos compartilham o mesmo comportamento se pudermos chamar os mesmos métodos em todos esses tipos. As definições de ```trait``` são uma maneira de agrupar assinaturas de métodos para definir um conjunto de comportamentos necessários a determinados tipos. 

Por exemplo, digamos que queremos implementar um crate que manipule dados de figuras planas (triângulo, quadrado, retângulo, trapézio, losango, círculo). Nosso crate terá que manipular os dados das figuras e calcular perímetro e área. Podemos usar o recurso de trait para definir os métodos a serem implementados por todas as figuras planas da nossa biblioteca.

```
// lib.rs

// Define a trait 'FiguraPlana'
trait FiguraPlana{
    fn print(&self);
    fn perimetro(&self) -> f64;
    fn area(&self) -> f64;
}
```


Uma trait define o comportamento de um tipo de dados. Este "comportamento" significa os métodos que podemos chamar a partir de um objeto daquele tipo de dados.

Vários tipos de dados podem compartilhar o mesmo comportamento, isto significa que podemos chamar o mesmo método a partir de objetos de todos os tipos que compartilham o comportamento.


Note que a trait possui as assinaturas dos métodos a serem implementados pelos tipos que implementarão esta trait.


## 2. Implementando uma trait

Continuando o exemplo, ainda no crate 'lib.rs', temos a implementação da trait 'FiguraPlana' para o tipo de dados 'Retangulo'.

```
pub struct Retangulo{
    base: f64,
    altura: f64,
}

// Métodos específicos do tipo 'Retangulo'
impl Retangulo {
    pub fn new(base: f64, altura: f64) -> Retangulo{
        Retangulo{base, altura}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Retangulo'
impl FiguraPlana for Retangulo{
    fn print(&self){
        println!("Retangulo: {{Base:{}, Altura:{}}}",
        self.base, self.altura);
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.base + self.altura)
    }

    fn area(&self) -> f64 {
        self.base * self.altura
    }
}
```

Observe que podemos implementar métodos específicos para o tipo e métodos que implementam a trait para o tipo.

Abaixo temos a implementação da mesma trait para o tipo ```Triangulo```.

```
pub struct Triangulo{
    lado_a: f64,
    lado_b: f64,
    lado_c: f64,
}

// Métodos específicos do tipo 'Triangulo'
impl Triangulo {
    pub fn new(lado_a: f64, lado_b: f64, lado_c: f64) -> Triangulo{
        Triangulo{lado_a, lado_b, lado_c}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Triangulo'
impl FiguraPlana for Triangulo{
    fn print(&self){
        println!("Triangulo: {{Lado A:{}, Lado B:{}, Lado C:{}}}",
        self.lado_a,self.lado_b, self.lado_c);
    }

    fn perimetro(&self) -> f64 {
        self.lado_a + self.lado_b + self.lado_c
    }

    // Calcula a area de um triangulo segundo a formula de Heron
    // https://mundoeducacao.uol.com.br/matematica/formula-heron.htm
    fn area(&self) -> f64 {
        let p = (self.lado_a + self.lado_b + self.lado_c) / 2.0;
        let q =
        p * (p-self.lado_a) * (p-self.lado_b) * (p-self.lado_c);
        q.sqrt()
    }
}
```

E, agora, a implementação da trait para o tipo 'Circulo'.

```
pub struct Circulo{
    raio: f64,
}

// Métodos específicos do tipo 'Circulo'
impl Circulo {
    pub fn new(raio: f64) -> Circulo{
        Circulo{raio}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Circulo'
impl FiguraPlana for Circulo{
    fn print(&self){
        println!("Circulo: {{Raio:{}}}", self.raio);
    }

    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.raio
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }
}
```

Assim, os tipos de dados 'Retangulo', 'Triangulo' e 'Circulo' terão os métodos 'print()', 'perimetro()' e 'area()'; e note que cada tipo tem seu método com uma implemetação diferente, conforme as necessidades do tipo.


## 3. Limitação na implementação de uma trait

Só podemos implementar uma trait se a trait ou o tipo, ou ambos, forem locais ao crate. Por exemplo, podemos implementar traits da biblioteca padrão, como' 'Display', em um tipo criado por nós em nosso crate. Também podemos implementar uma trait criada por nós em um tipo do Rust, como 'Vec\<T\>', por exemplo. Mas não podemos implementar traits externas em tipos externos. Por exemplo, não podemos implementar a trait 'Display' em 'Vec\<T\>' dentro de nosso crate porque 'Display' e 'Vec\<T\>' são ambos definidos na biblioteca padrão do Rust e não são locais ao nosso crate. Esta regra garante que o código de outras pessoas não possa quebrar o seu código e vice-versa. Sem esta regra, dois crates poderiam implementar a mesma trait para o mesmo tipo, e o Rust não saberia qual implementação usar. 
 

## 4. Implementação padrão em métodos de uma trait

É permitido ter um comportamento padrão para um método em uma trait. A implementação padão de um método da trait permite manter ou substituir o comportamento padrão do método. Para substituir basta implementar o método novamente quando o tipo implementar a trait. A implementação escrita quando o tipo implementar a trait irá sobrescrever a implementação padrão definida na declaração da trait. Abaixo temos o exemplo da implementação de um método padrão em nossa trait 'FiguraPlana' e a substituição deste método no tipo 'Circulo'.

```
// lib.rs

// Define a trait 'FiguraPlana'
pub trait FiguraPlana{
    fn print(&self);
    fn perimetro(&self) -> f64;
    fn area(&self) -> f64;

    // Implementação padrão de um método da trait
    fn relatorio(&self){
        self.print();
        println!("Perimetro: {}", self.perimetro());
        println!("Area     : {}", self.area());
    }
}

// . . . . . 
// código omitido
// . . . . . 

// Implementa a trait 'FiguraPlana' para o tipo 'Circulo'
impl FiguraPlana for Circulo{
    fn print(&self){
        println!("Circulo: {{Raio:{}}}", self.raio);
    }

    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.raio
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }

    // Substitui a implementação padrão definida na criação da trait
    fn relatorio(&self){
        println!("Circulo com raio {}", self.raio);
        println!("Perimetro: {}", self.perimetro());
        println!("Area     : {}", self.area());
    }
}
```

Note que, se o tipo não for implementar nenhum método da trait e quiser usar a implementação padrão dos métodos declarados na criação da trait, deverá ter um bloco 'impl' vazio, como por exemplo;

```
impl FiguraPlana for Trapezio{

}
```

Assim, o tipo 'Trapezio' irá usar todos os métodos da trait 'FiguraPlana' com a implementação padrão definida na criação da trait.

Observe que neste caso a trait deve ter implementação padrão para todos os métodos, pois para o tipo implementar a trait ele deve implementar todos os métodos declarados na trait; então, se não houver implementação padrão de algum método na declaração da trait, o tipo deverá implementar este método. 

## 5. Trait como parâmetro de função

Usando traits podemos definir funções que recebem tipos diferentes de dados. 

```
// Função que recebe um objeto que implementa a trait
// 'FiguraPlana'
pub fn relatorio2(figura: &impl FiguraPlana){
    println!("Perimetro: {}; Area: {}", figura.perimetro(),
    figura.area());
}
```


Outra sintaxe para uma função que recebe um objeto que implementa uma trait pode ser:

```
pub fn relatorio3<T: FiguraPlana>(figura: &T){
    println!("Area: {}; Perimetro: {}", figura.area(),
    figura.perimetro());
}
```

Esta sintaxe é conhecida como 'trait bound', algo como 'trait vinculada' numa tradução livre. Note que ela usa genéricos. Observe também a sintaxe para indicar que o tipo 'T' deve implementar a trait 'FiguraPlana'.

```
<T: FiguraPlana>
```

A sintaxe 'impl Trait', usada na função 'relatorio2()', é conveniente e torna o código mais conciso em casos simples, enquanto a sintaxe do tipo 'trait bound', usada na função 'relatorio3()', pode expressar mais complexidade em outros casos. 

Por exemplo, podemos ter uma função que recebe dois parâmetros que implementam uma mesma trait:

``` 
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

```

Usar a sintaxe 'impl Trait', como na função 'notify()' acima, é apropriado se queremos que 'item1' e 'item2' tenham tipos diferentes que implementam a trait 'Summary'. Porém, se queremos que os dois parâmetros tenham obrigatoriamente o mesmo tipo, devemos usar a sintaxe 'trait bound', como mostrado abaixo:

```
pub fn notify<T: Summary>(item1: &T, item2: &T) {

```

Para vincular o parâmetro a mais de uma trait usa-se as sintaxes abaixo:

```
pub fn notify(item: &(impl Summary + Display)) {
```
 

```
pub fn notify<T: Summary + Display>(item: &T) {
```

Nestes dois casos o parâmetro 'item' deve implementar as traits 'Summary' e 'Display'.

Em funções que recebem mais de um parâmetro genérico e onde cada um destes genéricos tem mais de uma 'trait bound'o código pode ficar difícil de ler, como no exemplo abaixo.

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

Nestes casos podemos usar a sintaxe 'where' fornecida pela linguagem Rust que torna o código mais fácil de ler:

```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```


Abaixo temos uma função que recebe um 'Vec' de objetos que implementam a trait 'FiguraPlana'. 


```
// Função que recebe um 'Vec' de objetos que implementam a trait
// 'FiguraPlana' como parâmetro
pub fn totalizar_area(figuras: Vec<&dyn FiguraPlana>) -> f64 {
    let mut area_total = 0.0;
    for f in figuras{
        area_total += f.area();
    }
    area_total
}
```

Note que os objetos trait neste caso devem ser 'dyn'. Se 'dyn' não for usado, o código acima emitirá erro solicitando a inclusão de 'dyn'.

'dyn' é um prefixo a ser usado com objetos do tipo trait (objetos que implementam determinada trait). A palavra-chave 'dyn' é usada para indicar que as chamadas a funções e métodos associados a trait considerada são despachados dinamicamente. Para usar uma trait dessa maneira ela deve ser 'object safe'.


## 6. Retornando tipos que implementam uma trait

Uma função pode retornar um tipo que implementa uma trait:

```
// Função que retorna um tipo que implementa a trait 'FiguraPlana
pub fn retorna_figura() -> impl FiguraPlana {
    Retangulo{base: 2.5, altura: 3.5}
}
```

Este recurso de especificar o tipo de retorno indicando apenas a trait que ele implementa é especialmente útil no contexto das closures e dos iterators. Closures e iterators criam tipos que apenas o compilador conhece ou tipos que tem nome muito longo para especificar. A sintaxe 'impl Trait' permite que você especifique de forma concisa que uma função retorna algum tipo que implementa a trait 'Iterator' sem precisar escrever o nome longo do tipo. 

Porém, este recurso só pode ser usado se a função retorna um tipo único. Se a função retornar tipos diferentes, como no código abaixo, irá emitir erro:

```
// Função que retorna tipos que implementam a trait 'FiguraPlana
// ERRO: a função abaixo não compila
pub fn retorna_figura2(b: bool) -> impl FiguraPlana {
    if b {
        Retangulo{base: 2.5, altura: 3.5}
    } else {
        Circulo{raio: 4.5}
    }
}
```

Ao tentar compilar o código acima receberemos o seguinte erro:

```
error[E0308]: `if` and `else` have incompatible types
   --> src\lib.rs:149:9
    |
146 | /     if b {
147 | |         Retangulo{base: 2.5, altura: 3.5}
    | |         --------------------------------- expected because of this
148 | |     } else {
149 | |         Circulo{raio: 4.5}
    | |         ^^^^^^^^^^^^^^^^^^ expected `Retangulo`, found `Circulo`
150 | |     }
    | |_____- `if` and `else` have incompatible types
    |
help: you could change the return type to be a boxed trait object
    |
145 | pub fn retorna_figura2(b: bool) -> Box<dyn FiguraPlana> {
    |                                    ~~~~~~~            +
help: if you change the return type to expect trait objects, box the returned expressions
    |
147 ~         Box::new(Retangulo{base: 2.5, altura: 3.5})
148 |     } else {
149 ~         Box::new(Circulo{raio: 4.5})
```

Com esta implementação Rust espera que o tipo retornado seja 'Retangulo' e emite erro porque o ramo 'else' retorna um tipo 'Circulo'. Isso acontece porque Rust precisa saber que o tipo retornado será escolhido dinamicamente e para isso podemos que usar o tipo 'Box' (como sugerido na mensagem de erro). 'Box' é uma estrutura que refere-se a um objeto alocado no heap.

```
// Função que retorna tipos que implementam a trait 'FiguraPlana
pub fn retorna_figura2(b: bool) -> Box<dyn FiguraPlana> {
    if b {
        Box::new(Retangulo{base: 2.5, altura: 3.5})
    } else {
        Box::new(Circulo{raio: 4.5})
    }
}
```

## 7. Usando trait bounds para implementar métodos condicionamente

Usando trait bounds em um bloco 'impl' que usa parâmetros de tipo genérico, podemos implementar métodos condicionalmente para tipos que implementam traits específicas. 

Por exemplo, o tipo 'Pair\<T\>' abaixo, implementa, no primeiro bloco 'impl', a função 'new()' que retorna uma nova instância de 'Pair\<T\>'. No segundo bloco 'impl', o método 'cmp_display()' só é implementado se o tipo concreto ao qual 'T' refere-se implementar as traits 'Display' e 'PartialOrd'.

```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

 Note que 'Self' no código acima é um alias para 'Pair\<T\>'.
 
Também podemos implementar uma trait condicionalmente para qualquer tipo que implemente outra trait. Implementações de uma trait em qualquer tipo que satisfaça traits bounds são chamadas de implementações gerais e são usadas extensivamente na biblioteca padrão do Rust. Por exemplo, a biblioteca Rust implementa a trait 'ToString' para qualquer tipo que implemente a trait 'display'.

```
impl<T: Display> ToString for T {
    // código omitido
}
```

Porque a biblioteca usa este tipo de implementação, podemos chamar o método 'to_string()', definido pela trait 'ToString', a partir de qualquer tipo que implementa a trait 'Display'. Por exemplo, podemos transformar inteiros em seus valores String correspondentes desta forma porque inteiros implementam Display: 

```
let s = 3.to_string();
```

Traits e traits bounds permitem que escrevamos código que usa parâmetros genéricos para reduzir a duplicação de código, mas também especificam ao compilador que queremos que o tipo genérico tenha um comportamento específico. O compilador pode então usar as informações da vinculação de traits (trait bounds) para verificar se todos os tipos concretos usados com nosso código têm o comportamento necessário.

Em linguagens dinamicamente tipadas, receberíamos um erro em tempo de execução se chamássemos um método em um tipo que não definisse o método. Mas Rust move esses erros para o tempo de compilação, então somos forçados a corrigir os problemas antes que nosso código possa ser executado. Além disso, não precisamos escrever código que verifica o comportamento em tempo de execução porque já verificamos em tempo de compilação. Fazer isso melhora o desempenho sem ter que abrir mão da flexibilidade dos genéricos. 

---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-02-traits.html)

[Rust By Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)

[https://www.programiz.com/rust/trait](https://www.programiz.com/rust/trait)

[https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html)

[https://doc.rust-lang.org/std/keyword.dyn.html](https://doc.rust-lang.org/std/keyword.dyn.html)

[https://doc.rust-lang.org/reference/items/traits.html#object-safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)

[https://doc.rust-lang.org/std/boxed/struct.Box.html](https://doc.rust-lang.org/std/boxed/struct.Box.html)

---

arataca89@gmail.com

Última atualização: 20240915
