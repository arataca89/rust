# Linguagem Rust - Vec

std::Vec

```
pub struct Vec<T, A = Global>
where
    A: Allocator,
{ /* private fields */ }
```

Um tipo de array dinâmico, escrito como ```Vec<T>```, abreviação para "vetor".


* [Exemplos](#exemplos)
* [Indexação](#indexação)
* [Slicing](#slicing)
* [Capacidade e realocação](#capacidade-e-realocação)

---

## Exemplos

```
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3]);

for x in &vec {
    println!("{x}");
}
assert_eq!(vec, [7, 1, 2, 3]);
```

O macro ```vec!``` é fornecida para facilitar a inicialização:

```
let mut vec1 = vec![1, 2, 3];
vec1.push(4);
let vec2 = Vec::from([1, 2, 3, 4]);
assert_eq!(vec1, vec2);
```

Pode-se também inicializar cada elemento de um ```Vec<T>``` com um valor fornecido. Isso pode ser mais eficiente do que realizar alocação e inicialização em etapas separadas, especialmente ao inicializar um vetor com valores "zeros":

```
let vec = vec![0; 5];
assert_eq!(vec, [0, 0, 0, 0, 0]);

// equivalente, mas potencialmente mais lento:
let mut vec = Vec::with_capacity(5);
vec.resize(5, 0);
assert_eq!(vec, [0, 0, 0, 0, 0]);

let vec1 = vec!["asd"; 5];
assert_eq!(vec1, ["asd", "asd", "asd", "asd", "asd"]);

let vec2 = vec![0.0; 3];
assert_eq!(vec2, [0.0, 0.0, 0.0]);
```

Para mais informações, consulte [Capacidade e Realocação](#capacidade-e-realocação). 


Use um ```Vec<T>``` como uma pilha:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    // imprime na tela: 3, 2, 1,
    print!("{top}, ");
}
``` 

## Indexação

O tipo ```Vec``` permite acesso a valores por índice, pois implementa a trait ```Index```. 

```
let v = vec![13, 14, 15, 16];
assert_eq!(v[0], 13);
assert_eq!(v[1], 14);
assert_eq!(v[2], 15);
assert_eq!(v[3], 16);
```

No entanto, tenha cuidado: se você tentar acessar um índice que não está no ```Vec```, seu software entrará em pânico!

<table><tr>
<td><img src="images/error.png" width="48" alt="ERROR"></td>
<td>
<pre>
let v = vec![13, 14, 15, 16];
assert_eq!(v[4], 17); // panic!
</pre>
</td>
</tr></table>


Use ```get()``` e ```get_mut()``` se você quiser verificar se o índice é válido.


## Slicing 

Um ```Vec``` pode ser mutável. Por outro lado, slices(fatias) são objetos somente de leitura. Para obter uma slice, use ```&```.

```
fn read_slice(slice: &[usize]) {
    for i in slice {
        print!("{i}, ");
    }
    println!("\n");
}

fn main() {
    let v = vec![13,14,15,16];
    read_slice(&v);
    
    // você também pode fazer assim:
    let u: &[usize] = &v;
    read_slice(u); 
    
    // ou assims:
    let u: &[_] = &v;
    read_slice(u);
}
```

Saída:

```
13, 14, 15, 16, 

13, 14, 15, 16, 

13, 14, 15, 16, 
```

Em Rust, é mais comum passar slices como argumentos, em vez de vetores, quando você só quiser fornecer acesso de leitura. O mesmo vale para ```String``` e ```&str```.

## Capacidade e realocação 



---

## Referências

[std::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

---

arataca89@gmail.com

Última atualização: 20241213