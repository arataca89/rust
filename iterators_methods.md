#### arataca89

# Linguagem Rust - Métodos de iteradores

[collect()](#collect) - Transforma um iterador em uma coleção. 

[enumerate()](#enumerate) - Cria um iterador que retorna a contagem da iteração atual e o  valor.

[into_iter()](#into_iter) - Cria um iterador a partir de um valor.

[next()](#next) - 
Avança o iterador e retorna o próximo valor.

---

## collect()

```
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized,
```

Transforma um iterador em uma coleção. 

```collect()``` pode receber qualquer iterável e transformá-lo em uma coleção. Este é um dos métodos mais poderosos da biblioteca padrão, usado em uma variedade de contextos.

O padrão mais básico em que ```collect()``` é usado é transformar uma coleção em outra. Você pega uma coleção, chama ```iter()``` nela, faz um monte de transformações e chama ```collect()``` no final.

Existem três métodos comuns que podem criar iteradores a partir de uma coleção:

* ```iter()``` , que itera sobre ```&T```.
* ```iter_mut()```  , que itera sobre ```&mut T```.
* ```into_iter()``` , que itera sobre ```T```.
 
Várias coisas na biblioteca padrão podem implementar um ou mais dos três, conforme apropriado.

```collect()``` também pode criar instâncias de tipos que não são coleções típicas. Por exemplo, uma ```String``` pode ser construída a partir de um vetor de ```char```, e um iterador de itens ```Result<T, E>``` pode ser coletado em ```Result<Collection<T>, E>```. Veja os exemplos abaixo para mais informações.

Como ```collect()``` é tão geral, pode causar problemas com inferência de tipo. Como tal, ```collect()``` é uma das poucas vezes em que você verá a sintaxe carinhosamente conhecida como **turbofish**: ```::<>```. Isso ajuda o algoritmo de inferência a entender especificamente qual coleção você está tentando coletar. 

### Exemplos

#### Uso básico

```
let a = [1, 2, 3];

let doubled: Vec<i32> = a.iter()
                         .map(|&x| x * 2)
                         .collect();

assert_eq!(vec![2, 4, 6], doubled);
```

Observe que precisamos especificar ```: Vec<i32>``` no lado esquerdo. Isso ocorre porque poderíamos coletar como ```VecDeque<T>```:

```
use std::collections::VecDeque;

let a = [1, 2, 3];

let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();

assert_eq!(2, doubled[0]);
assert_eq!(4, doubled[1]);
assert_eq!(6, doubled[2]);
```

Usando o 'turbofish' em vez de anotar o tipo na variável **doubled**:

```
let a = [1, 2, 3];

let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>();

assert_eq!(vec![2, 4, 6], doubled);
```

Como ```collect()``` só se importa com o que você está coletando, você ainda pode usar uma dica de tipo parcial, ```_``` (sublinhado), com o turbofish:

```
let a = [1, 2, 3];

let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();

assert_eq!(vec![2, 4, 6], doubled);
```

Usando ```collect()``` para criar uma ```String```.

```
let chars = ['g', 'd', 'k', 'k', 'n'];

let hello: String = chars.iter()
    .map(|&x| x as u8)
    .map(|x| (x + 1) as char)
    .collect();

assert_eq!("hello", hello);
```

Se você tem uma lista de ```Result<T, E>```, você pode usar ```collect()``` para ver se algum deles falhou:

```
let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];

let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

// retorna o primeiro erro
assert_eq!(Err("nope"), result);

let results = [Ok(1), Ok(3)];

let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

// retorna a coleção
assert_eq!(Ok(vec![1, 3]), result);
```

---

## enumerate()

```
fn enumerate(self) -> Enumerate<Self> 
where
    Self: Sized,
```

Cria um iterador que fornece a contagem da iteração atual, bem como o  valor.

O iterador retornado gera pares **(i, val)**, onde **i** é o índice atual da iteração e **val** é o valor retornado pelo iterador.

```enumerate()``` mantém sua contagem como um ```usize```. Se você quiser contar por um inteiro de tamanho diferente, a função [zip](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip) fornece funcionalidade semelhante. 

#### Overflow

O método não protege contra estouros, portanto, enumerar mais do que ```usize::MAX``` elementos produz um resultado incorreto ou causa pânico. Se as asserções de depuração estiverem habilitadas, um pânico é garantido.

#### Pânico

O iterador retornado pode entrar em pânico se o índice a ser retornado exceder o tamanho de um ```usize```.

#### Exemplo

```
let a = ['a', 'b', 'c'];

let mut iter = a.iter().enumerate();

assert_eq!(iter.next(), Some((0, &'a')));
assert_eq!(iter.next(), Some((1, &'b')));
assert_eq!(iter.next(), Some((2, &'c')));
assert_eq!(iter.next(), None);
```

---

## into_iter()

```
fn into_iter(self) -> Self::IntoIter
```
Cria um iterador a partir de um valor.

```
let v = [1, 2, 3];
let mut iter = v.into_iter();

assert_eq!(Some(1), iter.next());
assert_eq!(Some(2), iter.next());
assert_eq!(Some(3), iter.next());
assert_eq!(None, iter.next());
```

---

## next()

```
fn next(&mut self) -> Option<Self::Item>
```

Avança o iterador e retorna o próximo valor.

Retorna ```None``` quando a iteração termina. 

```
let a = [1, 2, 3];

let mut iter = a.iter();

// Chamar next() retorna o próximo valor...
assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
assert_eq!(Some(&3), iter.next());

// ... e 'None' é retornado ao final da iteração.
assert_eq!(None, iter.next());
```

---

## Referências

[std::IntoIterator::into_iter](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#tymethod.into_iter)

[std::Iterator::next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)

[std::Iterator::enumarate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)

[The three forms of iteration](https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration)

---

arataca89@gmail.com

Última atualização: 20241229
