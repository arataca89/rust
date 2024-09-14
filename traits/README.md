# Linguagem Rust - Traits

Uma ```trait``` define uma funcionalidade que um tipo de dados deverá implementar.

```trait bounds``` são usadas para especificar que um tipo genérico pode ser de qualquer tipo concreto que tenha determinado comportamento, ou seja, que implemente determinada ```trait```.

Traits são semelhantes ao recurso chamado de interfaces em outras linguagens.

[1. Definindo uma trait](#1-Definindo-uma-trait)

[2. Implementando uma trait](#2-Implementando-uma-trait)

[3. Limitação na implementação de uma trait](#3-Limitação-na-implementação-de-uma-trait)

[4. Implementação padrão em métodos de uma trait](#4-Implementação-padrão-em-métodos-de-uma-trait)


[5. Trait como parâmetro](#5-Trait-como-parâmetro)

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

## 5. Trait como parâmetro

asdfg

---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-02-traits.html)

[Rust By Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)

[https://www.programiz.com/rust/trait](https://www.programiz.com/rust/trait)

[https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html)

---

arataca89@gmail.com

Última atualização: 20240914
