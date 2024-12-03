# Linguagem Rust - String

```String``` é o tipo de string mais comum. Ele possui a propriedade do conteúdo da string, armazenado em um buffer alocado na memória heap. Ele está intimamente relacionado à sua contraparte emprestada, a ```str``` primitiva. 


* [Exemplos](#exemplos)
* [UFT8](#utf8)
* [Deref](#deref)
* [Representação](#representação)
* [Métodos](#métodos)
	- [new()](#new) - Cria uma nova ```String``` vazia.
	- [with_capacity()](#with_capacity) - Cria uma nova ```String``` vazia com a capacidade especificada. 
	- [from_utf8()](#from_utf8) - Converte um vetor de bytes em uma ```String```.
	- [from_utf8_lossy()](#from_utf8_lossy) - Converte uma slice de bytes em uma string, incluindo caracteres inválidos.
	- [from_utf16()](#from_utf16) - Decodifica um vetor v codificado em UTF-16 para uma ```String```, retornando ```Err``` se v contiver dados inválidos. 
	- [from_utf16_lossy()](#from_utf16_lossy) - Decodifica uma slice v codificada em UTF-16 em uma ```String```, substituindo dados inválidos pelo caractere de substituição (U+FFFD).
	- [from_raw_parts()](#from_raw_parts) - Cria uma nova String a partir de um ponteiro, um comprimento e uma capacidade (<font color="red">unsafe</font>).
	- [from_utf8_unchecked()](#from_utf8_unchecked) - Converte um vetor de bytes para uma ```String``` sem verificar se a string contém UTF-8 válido. (<font color="red">unsafe</font>).
	- [into_bytes()](#into_bytes) - Converte uma ```String``` em um vetor de bytes.
	- [as_str()](#as_str) - Extrai uma slice de string contendo toda a ```String```.
	- [as_mut_str()](#as_mut_str) - Converte uma ```String``` em uma slice de string mutável.
	- [push_str()](#push_str) - Anexa uma slice de string ao final da ```String```.
	- [reserve()](#reserve) - Aumenta a capacidade da ```String```.
	

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

Cria uma nova ```String``` vazia com pelo menos a capacidade especificada. 

As strings possuem um buffer interno para armazenar seus dados. A capacidade é o comprimento desse buffer e pode ser consultada com o método ```capacity()```. Este método cria uma string vazia, mas com um buffer inicial que pode conter pelo menos ```capacity``` bytes. Isso é útil quando você pode estar anexando muitos dados à string, reduzindo o número de realocações que ela precisa fazer.

Se a capacidade fornecida for 0, nenhuma alocação ocorrerá e este método será idêntico ao método ```new()```.

```
let mut s = String::with_capacity(10);

// A String não contém caracteres, embora tenha capacidade para mais
assert_eq!(s.len(), 0);

// Tudo isso é feito sem realocação...
let cap = s.capacity();
for _ in 0..10 {
    s.push('a');
}

assert_eq!(s.capacity(), cap);

// ...mas isso pode fazer com que a string seja realocada
s.push('a'); 
```

## from_utf8()

```
from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>
```

Converte um vetor de bytes em uma ```String```.

Uma string (```String```) é feita de bytes (```u8```), e um vetor de bytes (```Vec<u8>```) é feito de bytes, então esta função converte entre os dois. No entanto, nem todas as slices de bytes são ```Strings``` válidas: ```String``` requer que seja UTF-8 válido. ```from_utf8()``` verifica para garantir que os bytes sejam UTF-8 válidos e, em seguida, faz a conversão.

Se você tem certeza de que a slice de bytes é UTF-8 válido e não quer incorrer na sobrecarga da verificação de validade, há uma versão não segura desta função, ```from_utf8_unchecked()```, que tem o mesmo comportamento, mas ignora a verificação.

Este método tomará cuidado para não copiar o vetor, por uma questão de eficiência.

Se você precisa de uma ```&str``` em vez de uma ```String```, considere ```str::from_utf8()```.

O inverso deste método é ```into_bytes()```.

### Erros

Retorna ```Err``` se a slice não for UTF-8 com uma descrição do motivo pelo qual os bytes fornecidos não são UTF-8. O vetor que você moveu também está incluído.

### Exemplos:

Uso básico:

```
// alguns bytes, em um vetor
let sparkle_heart = vec![240, 159, 146, 150];

// Sabemos que esses bytes são válidos, então usaremos `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart); 
```

bytes incorretos:

```
// alguns bytes inválidos, em um vetor
let sparkle_heart = vec![0, 159, 146, 150];

assert!(String::from_utf8(sparkle_heart).is_err());
```

Veja a documentação para ```FromUtf8Error``` para mais detalhes sobre o que você pode fazer com esse erro.

## from_utf8_lossy()

```
from_utf8_lossy(v: &[u8]) -> Cow<'_, str>
```

Converte uma slice de bytes em uma string, incluindo caracteres inválidos.

```Strings```  são feitas de bytes (```u8```), e uma slice de bytes (```&[u8]```) é feita de bytes, então esta função converte entre os dois. No entanto, nem todas as slices de bytes são strings válidas: as strings precisam ser UTF-8 válidas. Durante esta conversão, ```from_utf8_lossy()``` substituirá quaisquer sequências UTF-8 inválidas por ```U+FFFD REPLACEMENT CHARACTER```, que se parece com isto: �

Se você tem certeza de que a slice de bytes é UTF-8 válida, e não quer incorrer na sobrecarga da conversão, há uma versão não segura desta função, ```from_utf8_unchecked()```, que tem o mesmo comportamento, mas ignora as verificações.

Esta função retorna um ```Cow<'a, str>```. Se nossa slice de bytes for UTF-8 inválida, então precisamos inserir os caracteres de substituição, o que mudará o tamanho da string e, portanto, exigirá uma ```String```. Mas se já for UTF-8 válido, não precisamos de uma nova alocação. Esse tipo de retorno nos permite lidar com ambos os casos.

### Exemplos

Uso básico:

```
// alguns bytes, em um vetor
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

assert_eq!("💖", sparkle_heart);
```

bytes incorretos:

```
// alguns bytes inválidos
let input = b"Hello \xF0\x90\x80World";
let output = String::from_utf8_lossy(input);

assert_eq!("Hello �World", output);
```

## from_utf16()

```
from_utf16(v: &[u16]) -> Result<String, FromUtf16Error>
```

Decodifica um vetor v codificado em UTF-16 para uma ```String```, retornando ```Err``` se v contiver dados inválidos. 

```
// 𝄞music
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0x0073, 0x0069, 0x0063];
assert_eq!(String::from("𝄞music"),
           String::from_utf16(v).unwrap());

// 𝄞mu<invalid>ic
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0xD800, 0x0069, 0x0063];
assert!(String::from_utf16(v).is_err());
```

## from_utf16_lossy()

```
from_utf16_lossy(v: &[u16]) -> String
```

Decodifica uma slice v codificada em UTF-16 em uma ```String```, substituindo dados inválidos pelo caractere de substituição (U+FFFD).

```
// 𝄞mus<invalid>ic<invalid>
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0x0073, 0xDD1E, 0x0069, 0x0063,
          0xD834];

assert_eq!(String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
           String::from_utf16_lossy(v));
```

## from_raw_parts() 

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
from_raw_parts(
    buf: *mut u8,
    length: usize,
    capacity: usize,
) -> String
```

Cria uma nova ```String``` a partir de um ponteiro, um comprimento e uma capacidade. 

### Segurança

Isso é altamente inseguro, devido ao número de invariantes que não são verificados:

* A memória em ```buf``` precisa ter sido alocada anteriormente pelo mesmo alocador que a biblioteca padrão usa, com um alinhamento necessário de exatamente 1.
* ```length``` precisa ser menor ou igual a ```capacity```.
* ```capacity``` precisa ser o valor correto.
* Os primeiros bytes de ```length``` em ```buf``` precisam ser UTF-8 válidos.

Violar isso pode causar problemas como corromper as estruturas de dados internas do alocador. Por exemplo, normalmente não é seguro construir uma ```String``` de um ponteiro para um array de caracteres C contendo UTF-8, a menos que você tenha certeza de que o array foi alocado originalmente pelo alocador da biblioteca padrão Rust.

A propriedade de ```buf``` é efetivamente transferida para a ```String```, que pode então desalocar, realocar ou alterar o conteúdo da memória apontada pelo ponteiro à vontade. Certifique-se de que nada mais use o ponteiro após chamar esta função.

```
use std::mem;

unsafe {
    let s = String::from("hello");

    // Evita que a String seja dropada automaticamente
    let mut s = mem::ManuallyDrop::new(s);

    let ptr = s.as_mut_ptr();
    let len = s.len();
    let capacity = s.capacity();

    let s = String::from_raw_parts(ptr, len, capacity);

    assert_eq!(String::from("hello"), s);
}
```

## from_utf8_unchecked()

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
from_utf8_unchecked(bytes: Vec<u8>) -> String
```

Converte um vetor de bytes para uma ```String``` sem verificar se a string contém UTF-8 válido.

Veja a versão segura, ```from_utf8()```, para mais detalhes.

### Segurança

Esta função é insegura porque não verifica se os bytes passados para ela são UTF-8 válidos. Se esta restrição for violada, pode causar problemas de segurança de memória para usuários futuros da ```String```, pois o restante da biblioteca padrão assume que as Strings são UTF-8 válidas. 

```
// alguns bytes, em um vetor
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = unsafe {
    String::from_utf8_unchecked(sparkle_heart)
};

assert_eq!("💖", sparkle_heart);
```

## into_bytes()

```
into_bytes(self) -> Vec<u8>
```

Converte uma ```String``` em um vetor de bytes.

Isso consome a ```String```, então não precisamos copiar seu conteúdo. 

```
let s = String::from("hello");
let bytes = s.into_bytes();

assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
```

## as_str()

```
as_str(&self) -> &str
```

Extrai uma slice de string contendo toda a ```String```.

```
let s = String::from("foo");

assert_eq!("foo", s.as_str());
```

## as_mut_str()

```
as_mut_str(&mut self) -> &mut str
```

Converte uma ```String``` em uma slice de string mutável.

```
let mut s = String::from("foobar");
let s_mut_str = s.as_mut_str();

s_mut_str.make_ascii_uppercase();

assert_eq!("FOOBAR", s_mut_str);
```

## push_str()

```
push_str(&mut self, string: &str)
```

Anexa uma slice de string ao final da ```String```.

``` 
let mut s = String::from("foo");

s.push_str("bar");

assert_eq!("foobar", s);
```

## capacity()

```
capacity(&self) -> usize
```

Retorna a capacidade da ```String```, em bytes.

```
let s = String::with_capacity(10);

assert!(s.capacity() >= 10);
```

## reserve()

```
reserve(&mut self, additional: usize)
```

Reserva capacidade para pelo menos mais ```additional``` bytes do que o comprimento atual. O alocador pode reservar mais espaço para evitar alocações frequentes especulativamente. Após chamar ```reserve()```, a capacidade será maior ou igual a ```self.len() + additional```. Não faz nada se a capacidade já for suficiente. 

### Pânico

Entra em pânico se a nova capacidade exceder ```usize```.

### Exemplos

Uso básico:

```
let mut s = String::new();

s.reserve(10);

assert!(s.capacity() >= 10);
```

Isso pode não aumentar realmente a capacidade: 

```
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s agora tem um comprimento de 2 e uma capacidade de pelo menos 10
let capacity = s.capacity();
assert_eq!(2, s.len());
assert!(capacity >= 10);

// Como já temos pelo menos 8 de capacidade extra, chamar isso...
s.reserve(8);

// ... na verdade não aumenta.
assert_eq!(capacity, s.capacity());
```

## reserve_exact()

```
reserve_exact(&mut self, additional: usize)
```






---

## Referências

[std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)

---

arataca89@gmail.com

Última atualização: 20241203
