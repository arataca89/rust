#### arataca89

# Linguagem Rust - Métodos de iteradores

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

---

arataca89@gmail.com

Última atualização: 20241229