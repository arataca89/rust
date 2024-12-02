# Linguagem Rust - String

```String``` é o tipo de string mais comum. Ele possui a propriedade do conteúdo da string, armazenado em um buffer alocado na memória heap. Ele está intimamente relacionado à sua contraparte emprestada, a ```str``` primitiva. 


* [Exemplos](#exemplos)
* [UFT8](#utf8)
* [Deref](#deref)
* [Representação](#representação)
* [Métodos](#métodos)
	- [new()](#new) - Cria uma nova ```String``` vazia.

---

## Exemplos

Você pode criar uma ```String``` a partir de uma string literal com ```String::from()```: 


```
let hello = String::from("Hello, world!");
```

Você pode anexar um ```char``` a uma ```String``` com o método ```push()``` e anexar uma ```&str``` com o método ```push_str()```:

```
let mut hello = String::from("Hello, ");

hello.push('w');
hello.push_str("orld!");
```

Se você tiver um vetor de bytes UTF-8, você pode criar uma ```String``` a partir dele com o método ```from_utf8()```:

```
// alguns bytes, em um vector
let sparkle_heart = vec![240, 159, 146, 150];

// Sabemos que estes bytes são válidos, então podemos usar `unwrap()`. 
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);
```

## UTF8

Strings são sempre UTF-8 válidas. Se você precisar de uma string não UTF-8, considere usar ```OsString```. É semelhante, mas sem a restrição UTF-8. Como UTF-8 é uma codificação de tamanho variável, strings são normalmente menores do que um array dos mesmos caracteres:

```
use std::mem;

// `s` é ASCII que representa cada `char` como um byte
let s = "hello";
assert_eq!(s.len(), 5);

// Um array `char` com o mesmo conteúdo poderia ser maior porque
// cada `char` tem quatro bytes
let s = ['h', 'e', 'l', 'l', 'o'];
let size: usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();
assert_eq!(size, 20);

// Porém, para strings não ASCII, a diferença será menor
// e algumas vezes terão o mesmo tamanho.
let s = "💖💖💖💖💖";
assert_eq!(s.len(), 20);

let s = ['💖', '💖', '💖', '💖', '💖'];
let size: usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();
assert_eq!(size, 20);
```

Isso levanta questões interessantes sobre como ```s[i]``` deve funcionar. O que ```i``` deve ser aqui? Várias opções incluem índices de bytes e índices de caracteres, mas, devido à codificação UTF-8, apenas índices de bytes forneceriam indexação de tempo constante. Obter o i-ésimo caractere, por exemplo, está disponível usando ```chars```:

```
let s = "hello";
let third_character = s.chars().nth(2);
assert_eq!(third_character, Some('l'));

let s = "💖💖💖💖💖";
let third_character = s.chars().nth(2);
assert_eq!(third_character, Some('💖'));
```

Em seguida, o que ```s[i]``` deve retornar? Como a indexação retorna uma referência aos dados subjacentes, ela pode ser ```&u8```, ```&[u8]``` ou algo semelhante. Como estamos fornecendo apenas um índice, ```&u8``` faz mais sentido, mas isso pode não ser o que o usuário espera e pode ser alcançado explicitamente com ```as_bytes()```:

```
// O primeiro byte é 104 - o valor byte de `'h'`
let s = "hello";
assert_eq!(s.as_bytes()[0], 104);
// ou
assert_eq!(s.as_bytes()[0], b'h');

// O primeiro byte é 240 que não é obviamente útil
let s = "💖💖💖💖💖";
assert_eq!(s.as_bytes()[0], 240);
```

Devido a essas ambiguidades/restrições, a indexação com um ```usize``` é simplesmente proibida:

<table><tr>
<td><img src="images/error.png" width="48" alt="ERROR"></td>
<td>
<pre>
let s = "hello";

// O código abaixo não irá compilar!
println!("A primeira letra de s é {}", s[0]);
</pre>
</td>
</tr></table>
 
No entanto, é mais claro como ```&s[i..j]``` deve funcionar (ou seja, indexação com um intervalo). Ele deve aceitar índices de bytes (para ser de tempo constante) e retornar um ```&str``` que é codificado em UTF-8. Isso também é chamado de "slice de string" ou "fatiamento de string". Observe que isso causará pânico se os índices de bytes fornecidos não forem limites de caracteres - consulte ```is_char_boundary``` para mais detalhes. Consulte as implementações para ```SliceIndex<str>``` para mais detalhes sobre o fatiamento de string. Para uma versão não panicante do fatiamento de string, consulte ```get```.

Os métodos ```bytes()``` e ```chars()``` retornam iteradores sobre os bytes e pontos de código da string, respectivamente. Para iterar sobre pontos de código juntamente com índices de bytes, use ```char_indices()```. 

## Deref

```String``` implementa ```Deref<Target = str>```, e portanto herda todos os métodos de ```str```. Além disso, isso significa que você pode passar uma ```String``` para uma função que recebe um ```&str``` usando um e comercial (```&```):

```
fn takes_str(s: &str) { }

let s = String::from("Hello");

takes_str(&s);
```

Isso criará uma ```&str``` a partir da ```String``` e a passará. Essa conversão é muito barata, então, geralmente, as funções aceitarão ```&str``` como argumento, a menos que precisem de uma ```String``` por algum motivo específico. 

Em certos casos, o Rust não tem informações suficientes para fazer essa conversão, conhecida como coerção Deref. No exemplo a seguir, uma slice de string ```&'a str``` implementa a trait ```TraitExample```, e a função ```example_func()``` recebe qualquer coisa que implemente esta trait. Nesse caso, o Rust precisaria fazer duas conversões implícitas, o que o Rust não tem meios de fazer. Por esse motivo, o exemplo a seguir não compilará.

<table><tr>
<td><img src="images/error.png" width="48" alt="ERROR"></td>
<td>
<pre>
trait TraitExample {}

impl<'a> TraitExample for &'a str {}

fn example_func<A: TraitExample>(example_arg: A) {}

let example_string = String::from("example_string");
example_func(&example_string);
</pre>
</td>
</tr></table>

Existem duas opções que funcionariam em vez disso. A primeira seria alterar a linha ```example_func(&example_string);``` para ```example_func(example_string.as_str());```, usando o método ```as_str()``` para extrair explicitamente a slice de string que contém a string. A segunda maneira altera ```example_func(&example_string);``` para ```example_func(&*example_string);```. Neste caso, estamos desreferenciando uma ```String``` para uma ```str```, então referenciando a ```str``` de volta para ```&str```. A segunda maneira é mais idiomática, no entanto, ambas funcionam para fazer a conversão explicitamente em vez de depender da conversão implícita.

Em programação, idiomático significa código que segue as convenções e melhores práticas de uma linguagem de programação ou framework específico. É considerado natural ou intuitivo por programadores experientes.

## Representação
 
Uma ```String``` possui três componentes: um ponteiro para os bytes, um comprimento e uma capacidade. O ponteiro aponta para o buffer interno que a ```String``` usa para armazenar seus dados. O comprimento é o número de bytes atualmente armazenados no buffer, e a capacidade é o tamanho do buffer em bytes. O comprimento sempre será menor ou igual à capacidade.

Este buffer é sempre armazenado na memória heap.

Você pode ver esses componentes com os métodos ```as_ptr()```, ```len()``` e ```capacity()```:

```
use std::mem;

let story = String::from("Once upon a time...");

// Evita que os dados da String sejam dropados automaticamente
let mut story = mem::ManuallyDrop::new(story);

let ptr = story.as_mut_ptr();
let len = story.len();
let capacity = story.capacity();

// story tem dezenove bytes
assert_eq!(19, len);

// We can re-build a String out of ptr, len, and capacity. This is all
// unsafe because we are responsible for making sure the components are
// valid:

// Podemos reconstruir uma String a partir de ptr, len e capacity. Tudo isso é
// inseguro porque somos responsáveis ​​por garantir que os componentes sejam
// válidos:
let s = unsafe { String::from_raw_parts(ptr, len, capacity) } ;

assert_eq!(String::from("Once upon a time..."), s);
```

Se uma ```String``` tiver capacidade suficiente, adicionar elementos a ela não realocará. Por exemplo, considere este programa:

```
let mut s = String::new();

println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
```

Isso irá gerar o seguinte:

```
0
8
16
16
32
32
```

Inicialmente, não temos nenhuma memória alocada, mas à medida que anexamos à string, ela aumenta sua capacidade de forma apropriada. Se, em vez disso, usarmos o método ```with_capacity()``` para alocar a capacidade correta inicialmente:

```
let mut s = String::with_capacity(25);

println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
```

Acabamos com uma saída diferente:

```
25
25
25
25
25
25
```

Aqui, não há necessidade de alocar mais memória dentro do loop. 

### Métodos

#### new()
```
new() -> String
```


Cria uma nova ```String``` vazia.

Dado que a ```String``` está vazia, isso não alocará nenhum buffer inicial. Embora isso signifique que esta operação inicial é muito barata, pode causar alocação excessiva mais tarde quando você adicionar dados. Se você tiver uma ideia de quanta informação a ```String``` irá conter, considere o método ```with_capacity()``` para evitar re-alocação excessiva.

Exemplo:

```
let s = String::new();
```

### with_capacity()
```
with_capacity(capacity: usize) -> String
```






---

## Referências

[std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)

---

arataca89@gmail.com

Última atualização: 20241202
