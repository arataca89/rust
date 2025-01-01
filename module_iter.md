#### arataca89

# Linguagem Rust - Módulo iter

Um iterador é uma ferramenta que te ajuda a iterar sobre uma coleção e processar seus dados. 

Iteradores são amplamente utilizados em código Rust idiomático, por isso vale a pena se familiarizar com eles.

Antes de explicar mais, vamos falar sobre como o módulo **iter** é estruturado:

## Organização do módulo iter

O módulo **iter** é organizado por tipos:

* [Traits](https://doc.rust-lang.org/std/iter/#traits) são a parte principal: essas traits definem que tipo de iteradores existem e o que você pode fazer com eles. Vale a pena investir algum tempo extra de estudo nos métodos dessas traits.
* [Funções](https://doc.rust-lang.org/std/iter/#functions) fornecem algumas maneiras úteis de criar alguns iteradores básicos.
* [Structs](https://doc.rust-lang.org/std/iter/#structs) são frequentemente os tipos de retorno dos métodos das traits. Normalmente, você vai querer olhar para o método que cria a struct, em vez da struct em si. Para mais detalhes sobre o porquê, veja [Implementando Iterator](#implementando-iterator).

## Iterator

O coração e a alma deste módulo é a trait **Iterator**. O núcleo de **Iterator** parece isto:

```
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Um iterador possui um método, ```next()``` que, quando chamado, retorna um ```Option<Item>```. Chamar ```next()``` retornará ```Some(Item)``` enquanto houver elementos ou ```None``` quando não houver mais elementos.

Iteradores individuais podem optar por retomar a iteração. Assim, chamar ```next()``` novamente pode, ou não, eventualmente começar a retornar ```Some(Item)``` novamente em algum momento (por exemplo, veja [TryIter](https://doc.rust-lang.org/std/sync/mpsc/struct.TryIter.html)).

A definição completa de um iterador inclui vários outros métodos, mas eles são métodos padrão, construídos em cima de ```next()```, e você os obtém gratuitamente.

Iteradores também são compostos, e é comum concatená-los para realizar formas mais complexas de processamento. Consulte a seção [Adaptadores](#adaptadores) abaixo para obter mais detalhes.

## As três formas de iteração
 
Existem três métodos comuns que podem criar iteradores a partir de uma coleção:

* **iter()**, que itera sobre **&T**.
* **iter_mut()**, que itera sobre **&mut T**.
* **into_iter()**, que itera sobre **T**.

Vários elementos na biblioteca padrão podem implementar um ou mais dos três, conforme apropriado.

## Implementando Iterator

Criar um iterador envolve duas etapas: criar uma estrutura para armazenar o estado do iterador e, em seguida, implementar ```Iterator``` para essa estrutura. É por isso que existem tantas estruturas neste módulo: existe uma para cada iterador e adaptador de iterador.
 
Vamos criar um iterador chamado **Counter** que conta de 1 a 5:

```
// Primeiro, deve-se criar a struct:

/// Um iterador que conta de um a cinco
struct Counter {
    count: usize,
}

// queremos que nossa contagem comece em um, então vamos adicionar um método new() para ajudar.
// Isso não é estritamente necessário, mas é conveniente. Note que começamos
// `count` em zero, veremos o porquê na implementação de `next()` abaixo.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


// Depois, implementamos a trait 'Iterator' para o tipo 'Counter':
impl Iterator for Counter {
    // usaremos o tipo 'usize' para a contagem
    type Item = usize;

    // next() é o único método requerido
    fn next(&mut self) -> Option<Self::Item> {
        // Incrementa nossa contagem. É por isso que iniciamos com zero.
        self.count += 1;

        // Verifica se a contagem chegou ao limite.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// E agora podemos usar nosso iterador!

#[test]
fn Count_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main(){
    println!("Criando um iterador");
}
```

Chamar ```next()``` dessa forma fica repetitivo. Rust tem uma construção que pode chamar ```next()``` no seu iterador, até que ele chegue a ```None```. Vamos dar uma olhada nisso a seguir.

## Loop for e IntoIterator

A sintaxe do loop **for** em Rust é otimizada ara iteradores. Aqui está um exemplo básico de **for**:

```
let values = vec![1, 2, 3, 4, 5];

for x in values {
    println!("{x}");
}
```

Isso imprimirá os números de um a cinco, cada um em sua própria linha. Mas você notará algo aqui: nunca chamamos nada em nosso vetor para produzir um iterador. O que aconteceu?

Existe uma trait na biblioteca padrão para converter algo em um iterador: ```IntoIterator```. Essa trait tem um método, ```into_iter()```, que converte a coisa que implementa ```IntoIterator``` em um iterador. Vamos dar uma olhada nesse loop **for** novamente e no que o compilador o converte:

```
let values = vec![1, 2, 3, 4, 5];

for x in values {
    println!("{x}");
}
```

Rust transforma esse código em:

```
let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{x}"); };
        },
    };
    result
}
```

Primeiro, chamamos ```into_iter()``` no valor. Então, combinamos com o iterador que retorna, chamando ```next()``` repetidamente até vermos um ```None```. Nesse ponto, saímos do loop e terminamos de iterar.

Há mais um detalhe sutil aqui: a biblioteca padrão contém uma implementação interessante de ```IntoIterator```:

```
impl<I: Iterator> IntoIterator for I
```

Em outras palavras, todo ```Iterator``` implementa ```IntoIterator```, simplesmente retornando a si mesmo. Isso significa duas coisas:

1. Se você está escrevendo um ```Iterator```, você pode usá-lo com um loop **for**.
2. Se você está criando uma coleção, implementar ```IntoIterator``` para ela permitirá que sua coleção seja usada com o loop **for**.

## Iterando usando referências

Como ```into_iter()``` recebe ```self``` por valor, usar um loop **for** para iterar sobre uma coleção consome essa coleção. Muitas vezes, você pode querer iterar sobre uma coleção sem consumi-la. Muitas coleções oferecem métodos que fornecem iteradores sobre referências, convencionalmente chamados ```iter()``` e ```iter_mut()``` respectivamente:

```
let mut values = vec![41];
for x in values.iter_mut() {
    *x += 1;
}
for x in values.iter() {
    assert_eq!(*x, 42);
}
assert_eq!(values.len(), 1); // `values` ainda é propriedade desta função.
```

Se um tipo de coleção **C** fornece ```iter()```, ele geralmente também implementa ```IntoIterator``` para **&C**, com uma implementação que apenas chama ```iter()```. Da mesma forma, uma coleção **C** que fornece ```iter_mut()``` geralmente implementa ```IntoIterator``` para ```&mut C``` delegando para ```iter_mut()```. Isso permite um atalho conveniente:

```
let mut values = vec![41];
for x in &mut values { // o mesmo que `values.iter_mut()`
    *x += 1;
}
for x in &values { // o mesmo que `values.iter()`
    assert_eq!(*x, 42);
}
assert_eq!(values.len(), 1);
```

Embora muitas coleções ofereçam ```iter()```, nem todas oferecem ```iter_mut()```. Por exemplo, alterar as chaves de um ```HashSet<T>``` pode colocar a coleção em um estado inconsistente se as chaves hash mudarem, então esta coleção só oferece ```iter()```.

## Adaptadores

Funções que recebem um ```Iterator``` e retornam outro são frequentemente chamadas de 'adaptadores de iterador', pois são uma forma do 'padrão de adaptador'.

Adaptadores de iterador comuns incluem [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map), [take](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take) e [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter). Para mais informações, consulte sua documentação.

Se um adaptador de iterador entrar em pânico, o iterador estará em um estado não especificado (mas seguro para a memória). Esse estado também não é garantido para permanecer o mesmo entre as versões do Rust, portanto, você deve evitar depender dos valores exatos retornados por um iterador que entrou em pânico.

## Iterador é preguiçoso

Iteradores (e adaptadores de iteradores) são preguiçosos. Isso significa que apenas criar um iterador não faz muito. Nada realmente acontece até que você chame ```next()```. Isso às vezes é uma fonte de confusão ao criar um iterador apenas para seus efeitos colaterais. Por exemplo, o método ```map()``` chama uma closure em cada elemento sobre o qual ele itera:

```
let v = vec![1, 2, 3, 4, 5];
v.iter().map(|x| println!("{x}"));
```

Isso não imprimirá nenhum valor, pois apenas criamos um iterador, em vez de usá-lo. O compilador nos avisará sobre esse tipo de comportamento:

```
warning: unused result that must be used: iterators are lazy and
do nothing unless consumed
```

A maneira idiomática de escrever um mapeamento deste tipo é usar um loop **for** ou chamar o método ```for_each()```:

```
let v = vec![1, 2, 3, 4, 5];

v.iter().for_each(|x| println!("{x}"));
// ou
for x in &v {
    println!("{x}");
}
```

Outra maneira comum de processar um iterador é usar o método ```collect()``` para produzir uma nova coleção.

## Iteradores podem ser infinitos

Iteradores não precisam ser finitos. Como exemplo, um intervalo aberto é um iterador infinito:

```
let numbers = 0..;
```

É comum usar o adaptador de iterador ```take()``` para transformar um iterador infinito em um finito:

```
let numbers = 0..;
let five_numbers = numbers.take(5);

for number in five_numbers {
    println!("{number}");
}
```

Isso imprimirá os números de 0 a 4, cada um em sua própria linha.

Lembre-se de que os métodos em iteradores infinitos, mesmo aqueles para os quais um resultado pode ser determinado matematicamente em tempo finito, podem não terminar. Especificamente, métodos como ```min()```, que no caso geral exigem percorrer cada elemento no iterador, provavelmente não retornarão com sucesso para nenhum iterador infinito.

```
let ones = std::iter::repeat(1);
let least = ones.min().unwrap(); // Loop infinito!
// `ones.min()` causa um loop infinito, então não chegaremos a esse ponto!
println!("O menor número um é {least}.");
```

## [Structs](https://doc.rust-lang.org/std/iter/#structs)

## [Traits](https://doc.rust-lang.org/std/iter/#traits)

## [Functions](https://doc.rust-lang.org/std/iter/#functions)

## Referências

[std::iter](https://doc.rust-lang.org/std/iter/)

---

arataca89@gmail.com

Última atualização: 20250101