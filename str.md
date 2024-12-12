# Linguagem Rust - str

O tipo ```str```, também chamado de "slice de string", é o tipo de string mais primitivo. Ele é geralmente visto em sua forma emprestada, ```&str```. É também o tipo de literais de string, ```&'static str```. 


* [Uso básico](#uso-básico)
* [Representação](#representação)
* [Invariante](#invariante)
* [Métodos](#métodos)
   - [len()](#len) - Retorna o comprimento da ```str```. 
   - [is_empty()](#is_empty) - Retorna ```true``` se ```self``` tem um comprimento de zero bytes.
   - [is_char_boundary()](#is_char_boundary) - Verifica se é o primeiro byte em uma sequência UTF-8 ou o final da string.
   - [as_bytes()](#as_bytes) - Converte uma slice de string em uma slice de bytes.
   - [as_bytes_mut()](#as_bytes_mut) - Converte uma slice de string mutável em uma slice de bytes mutável  (<font color="red">unsafe</font>).
   - [as_ptr()](#as_ptr) - Converte uma slice de string em um ponteiro bruto.
   - [as_mut_ptr()](#as_mut_ptr) - Converte uma slice de string mutável em um ponteiro bruto.
   - [get()](#get) - Retorna uma subslice da ```str```.
   - [get_mut()](#get_mut) - Retorna uma subslice mutável da ```str```.
   - [get_unchecked()](#get_unchecked) - Retorna uma subslice não verificada da ```str``` (<font color="red">unsafe</font>).
   - [get_unchecked_mut()](#get_unchecked_mut) - Retorna uma subslice mutável não verificada da ```str``` (<font color="red">unsafe</font>).
   - [split_at()](#split_at) - Divide uma slice de string em duas. 
   - [split_at_mut()](#split_at_mut) - Divide uma slice mutável de string em duas. 
   - [split_at_checked()](#split_at_checked) - Divide uma slice de string em duas.
   - [split_at_mut_checked()](#split_at_mut_checked) - Divide uma slice mutável de string em duas.
   - [chars()](#chars) - Retorna um iterador sobre os caracteres de uma slice de string.
   - [char_indices()](#char_indices) - Retorna um iterador sobre os caracteres de uma slice de string e suas posições.
   - [bytes()](#bytes) - Retorna um iterador sobre os bytes de uma slice de string.
   - [split_whitespace()](#split_whitespace) - Divide uma slice de string conforme os espaços em branco.
   - [split_ascii_whitespace()](#split_ascii_whitespace) - Divide uma slice de string conforme os espaços em branco ASCII.
   - [lines()](#lines) - Retorna um iterador sobre as linhas de uma string, como slices de string.
   - encode_utf16() - Retorna um iterador de ```u16``` sobre a string codificada como UTF-16.
   - [contains()](#contains) - Retorna ```true``` se a ```str``` contém o padrão passado como argumento.
   - [starts_with()](#starts_with) - Retorna ```true``` se o padrão passado como argumento corresponde a um prefixo da ```str```.
   - [ends_with()](#ends_with) - Retorna ```true``` se o padrão passado como argumento corresponde a um sufixo da ```str```.
   - [find()](#find) - Retorna o índice de byte do primeiro caractere que corresponde ao padrão passado como argumento.
   - [rfind()](#rfind) - Retorna o índice de byte para o primeiro caractere da última correspondência do padrão passado como argumento.
   - [split()](#split) - Separa a ```str``` em subslices conforme o padrão passado como argumento.
   - [split_inclusive()](#split_inclusive) - Separa a ```str``` em subslices conforme o padrão passado como argumento; insere o padrão no final da subslice.
	- [rsplit()](#rsplit) - Separa a ```str``` em subslices conforme o padrão passado como argumento. Itera pelas subslices retornadas na ordem inversa, de trás pra frente, da direita para a esquerda.
	- [split_terminator()](#split_terminator) - Equivalente a ```split()```, exceto que a substring final é ignorada se estiver vazia.
	- [rsplit_terminator()](#rsplit_terminator) - Equivalente a ```split()```, exceto que a substring final é ignorada se estiver vazia.  Itera pelas subslices retornadas na ordem inversa, de trás pra frente, da direita para a esquerda.
	- [splitn()](#splitn) - Retorna um iterador sobre as substrings da slice de string fornecida, separadas por um padrão, restrito a retornar no máximo ```n``` itens.
	- [rsplitn()](#rsplitn) - Retorna um iterador sobre as substrings da slice de string fornecida, separadas por um padrão, iniciando no final da slice de string, restrito a retornar no máximo ```n``` itens.
	- [split_once()](#split_once) - Divide a ``str``` na primeira ocorrência do delimitador especificado e retorna o prefixo antes do delimitador e o sufixo após o delimitador.
	- [rsplit_once()](#rsplit_once) - Divide a ``str``` na última ocorrência do delimitador especificado e retorna o prefixo antes do delimitador e o sufixo após o delimitador.
	- [matches()](#matches) - Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento.
	- [rmatches()](#rmatches) - Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento, iniciando no final da ```str```.
	- [match_indices()](#match_indices) - Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento, assim como seus índices.
	- [rmatch_indices()](#rmatch_indices) - Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento, assim como seus índices, em ordem reversa.
	- [trim()](#trim) - Remove espaços em branco do início e do fim da ```str```.
	- [trim_start()](#trim_start) - Remove espaços em branco do início da ```str```.
	- [trim_end()](#trim_end) - Remove espaços em branco do final da ```str```.
	- [trim_matches()](#trim_matches) - Remove todos os prefixos e sufixos que correspondem a um padrão.
	- [strip_prefix()](#strip_prefix) - Remove o prefixo passado como argumento.
	- [strip_suffix()](#strip_suffix) - Remove o sufixo passado como argumento.
	- [trim_end_matches()](#trim_end_matches) - Remove todos os sufixos que correspondem a um padrão.
	- [parse()](#parse) - Converte a ```str``` em outro tipo.
	- [is_ascii()](#is_ascii) - Verifica se todos os caracteres estão dentro do intervalo ASCII. 
	- [eq_ignore_ascii_case()](#eq_ignore_ascii_case) - Verifica se duas strings correspondem em maiúsculas e minúsculas ASCII.
	- [make_ascii_uppercase()](#make_ascii_uppercase) - Converte a string em maiúsculas ASCII.
	- [make_ascii_lowercase()](#make_ascii_lowercase) - Converte a string em minúsculas ASCII.
	- [trim_ascii_start()](#trim_ascii_start) - Remove os espaços em branco ASCII do início.
	- [trim_ascii_end()](#trim_ascii_end) - Remove os espaços em branco ASCII do fim.
	- [trim_ascii()](#trim_ascii) - Remove os espaços em branco ASCII do início e do fim.
	- [escape_debug()](#escape_debug) - Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_debug```.
	- [escape_default()](#escape_default) - Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_default```.
	- [escape_unicode()](#escape_unicode) - Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_unicode```.
	- [into_boxed_bytes()](#into_boxed_bytes) - Converte um ```Box<str>``` em um ```Box<[u8]>``` sem copiar ou alocar.
	- [replace()](#replace) - Substitui todas as correspondências de um padrão por outra string.
	- [replacen()](#replacen) - Substitui as primeiras N correspondências de um padrão por outra string.
	- [to_lowercase()](#to_lowercase) - Retorna o equivalente em minúsculas desta slice de string, como uma nova ```String```.
	- [to_uppercase()](#to_uppercase) - Retorna o equivalente em maiúsculas desta slice de string, como uma nova ```String```.
	
	
---

 (<font color="red">unsafe</font>).



---

## Uso básico

Literais de string são slices de string: 


```
let hello_world = "Hello, World!";
```

Aqui, declaramos uma slice de string inicializada com um literal de string. Literais de string têm um tempo de vida estático, o que significa que a string ```hello_world``` é garantida como válida durante toda a duração do programa. Podemos especificar explicitamente o tempo de vida de ```hello_world``` também:

```
let hello_world: &'static str = "Hello, world!";
```

## Representação

Um ```&str``` é composto por um ponteiro para alguns bytes e um comprimento. Você pode ver esses componentes com os métodos ```as_ptr()``` e ```len()```:

```
use std::slice;
use std::str;

let story = "Once upon a time...";

let ptr = story.as_ptr();
let len = story.len();

// story tem dezenove bytes
assert_eq!(19, len);

// Podemos reconstruir uma str a partir de ptr e len. Isso tudo é inseguro porque
// somos responsáveis ​​por garantir que os dois componentes sejam válidos:
let s = unsafe {
    // Primeiro, construimos uma &[u8]...
    let slice = slice::from_raw_parts(ptr, len);

    // ... então convertemos a slice numa slice de string
    str::from_utf8(slice)
};

assert_eq!(s, Ok(story));
```
Nota: Este exemplo mostra os detalhes internos de ```&str```. ```unsafe``` não deve ser usado para obter uma slice de string em circunstâncias normais. Use ```as_str()``` em vez disso. 

## Invariante

Bibliotecas Rust podem assumir que slices de string são sempre UTF-8 válidas.

Construir uma slice de string não-UTF-8 não provoca um comportamento indefinido imediato, mas qualquer função chamada com uma slice de string pode assumir que ela é UTF-8 válida, o que significa que uma slice de string não-UTF-8 pode levar a um comportamento indefinido no futuro. 

## Métodos

## len()

```
len(&self) -> usize
```

Retorna o comprimento de ```self```.

Este comprimento está em bytes, não em caracteres ou grafemas. Em outras palavras, pode não ser o que um humano considera o comprimento da string.

```
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

## is_empty()

```
is_empty(&self) -> bool
```

Retorna ```true``` se ```self``` tem um comprimento de zero bytes.

```
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

## is_char_boundary()

```
is_char_boundary(&self, index: usize) -> bool
```

Verifica se o byte ```index``` é o primeiro byte em uma sequência de pontos de código UTF-8 ou o final da string.

O início e o fim da string (quando ```index == self.len()```) são considerados limites.

Retorna ```false``` se ```index``` for maior que ```self.len()```.

```
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// inicia em `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// segundo byte de `ö`
assert!(!s.is_char_boundary(2));

// terceiro byte de `老`
assert!(!s.is_char_boundary(8));
```

## as_bytes()

```
as_bytes(&self) -> &[u8]
```
 
Converte uma slice de string em uma slice de bytes. Para converter a slice de bytes de volta para uma slice de string, use a função ```from_utf8()```.

```
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
    
let b2 = "123".as_bytes();
assert_eq!(b2, [49,50,51]);
```

## as_bytes_mut()

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
as_bytes_mut(&mut self) -> &mut [u8]
```

Converte uma slice de string mutável em uma slice de bytes mutável.

### Segurança

O chamador deve garantir que o conteúdo da slice seja UTF-8 válido antes que o empréstimo termine e a ```str``` subjacente seja usada.

O uso de uma ```str``` cujo conteúdo não seja UTF-8 válido provoca um comportamento indefinido.

### Exemplos

Uso básico:

```
let mut s = String::from("Hello");
let bytes = unsafe { s.as_bytes_mut() };

assert_eq!(b"Hello", bytes);
```

Mutabilidade:

```
let mut s = String::from("🗻∈🌏");

unsafe {
    let bytes = s.as_bytes_mut();

    bytes[0] = 0xF0;
    bytes[1] = 0x9F;
    bytes[2] = 0x8D;
    bytes[3] = 0x94;
}

assert_eq!("🍔∈🌏", s);
```

## as_ptr()

```
as_ptr(&self) -> *const u8
```

Converte uma slice de string em um ponteiro bruto.

Como slices de string são slices de bytes, o ponteiro bruto aponta para um ```u8```. Este ponteiro apontará para o primeiro byte da slice de string.

O chamador deve garantir que o ponteiro retornado nunca seja escrito. Se você precisar alterar o conteúdo da slice de string, use ```as_mut_ptr()```.

```
let s = "Hello";
let ptr = s.as_ptr();
```

## as_mut_ptr()

```
as_mut_ptr(&mut self) -> *mut u8
```
 
Converte uma slice de string mutável em um ponteiro bruto.

Como slices de string são slices de bytes, o ponteiro bruto aponta para um ```u8```. Este ponteiro apontará para o primeiro byte da fatia de string.

É sua responsabilidade garantir que a slice de string seja modificada apenas de uma forma que a mantenha válida em UTF-8.

## get()

```
get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>
where
    I: SliceIndex<str>,
```

Retorna uma subslice da ```str```.

Esta é a alternativa para indexar a ```str``` que não gera pânico. Retorna ```None``` sempre que a operação de indexação equivalente causaria pânico.

``` 
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// índices fora dos limites de sequência UTF-8
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// fora dos limites
assert!(v.get(..42).is_none());
```

## get_mut()

```
get_mut<I>(
    &mut self,
    i: I,
) -> Option<&mut <I as SliceIndex<str>>::Output>
where
    I: SliceIndex<str>,
```

Retorna uma subslice mutável da ```str```.

Esta é a alternativa para indexar a ```str``` que não gera pânico. Retorna ```None``` sempre que a operação de indexação equivalente causaria pânico.

```
let mut v = String::from("hello");
// comprimento correto
assert!(v.get_mut(0..5).is_some());
// fora dos limites
assert!(v.get_mut(..42).is_none());
assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

assert_eq!("hello", v);
{
    let s = v.get_mut(0..2);
    let s = s.map(|s| {
        s.make_ascii_uppercase();
        &*s
    });
    assert_eq!(Some("HE"), s);
}
assert_eq!("HEllo", v);
```

## get_unchecked()

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Output
where
    I: SliceIndex<str>,
```

Retorna uma subslice não verificada da ```str```.

Esta é a alternativa não verificada para indexar a ```str```.

### Segurança

Os chamadores desta função são responsáveis por garantir que essas pré-condições sejam satisfeitas:

* O índice inicial não deve exceder o índice final;
* Os índices devem estar dentro dos limites da slice original;
* Os índices devem estar em limites de sequência UTF-8.

Caso contrário, a slice de string retornada pode referenciar memória inválida ou violar as invariantes comunicadas pelo tipo ```str```.

```
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

## get_unchecked_mut()

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
 get_unchecked_mut<I>(
    &mut self,
    i: I,
) -> &mut <I as SliceIndex<str>>::Output
where
    I: SliceIndex<str>,
```

 Retorna uma subslice mutável não verificada da ```str```.

Esta é a alternativa não verificada para indexar a ```str```.

### Segurança

Os chamadores desta função são responsáveis por garantir que essas pré-condições sejam satisfeitas:

* O índice inicial não deve exceder o índice final;
* Os índices devem estar dentro dos limites da slice original;
* Os índices devem estar em limites de sequência UTF-8.

Caso contrário, a slice de string retornada pode referenciar memória inválida ou violar as invariantes comunicadas pelo tipo ```str```.

```
let mut v = String::from("🗻∈🌏");
unsafe {
    assert_eq!("🗻", v.get_unchecked_mut(0..4));
    assert_eq!("∈", v.get_unchecked_mut(4..7));
    assert_eq!("🌏", v.get_unchecked_mut(7..11));
}
```

## split_at()

```
split_at(&self, mid: usize) -> (&str, &str)
```

Divide uma slice de string em duas.

O argumento, ```mid```, deve ser um deslocamento de byte do início da string. Ele também deve estar no limite de um ponto de código UTF-8.

As duas slices retornadas vão do início até ```mid```, e de ```mid``` até o final.

Para obter slices de string mutáveis, consulte o método ```split_at_mut()```.

### Pânico

Entra em pânico se ```mid```  não estiver em um limite de ponto de código UTF-8, ou se estiver além do final do último ponto de código da slice de string. Para uma alternativa que não entre em pânico, consulte ```split_at_checked()```.

```
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

## split_at_mut()

```
split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str)
```  

Divide uma slice mutável de string em duas.

O argumento, ```mid```, deve ser um deslocamento de byte do início da string. Ele também deve estar no limite de um ponto de código UTF-8.

As duas slices retornadas vão do início até ```mid```, e de ```mid``` até o final.

Para obter fatias de string imutáveis, consulte o método ```split_at()```. 


### Pânico

Entra em pânico se ```mid```  não estiver em um limite de ponto de código UTF-8, ou se estiver além do final do último ponto de código da slice de string. Para uma alternativa que não entre em pânico, consulte ```split_at_mut_checked()```.

```
let mut s = "Per Martin-Löf".to_string();
{
    let (first, last) = s.split_at_mut(3);
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);
```

## split_at_checked()

```
split_at_checked(&self, mid: usize) -> Option<(&str, &str)>
```

Divide uma slice de string em duas.

O argumento, ```mid```, deve ser um deslocamento de byte do início da string. Ele também deve estar no limite de um ponto de código UTF-8.

As duas slices retornadas vão do início até ```mid```, e de ```mid``` até o final.

Para obter slices de string mutáveis, consulte o método ```split_at_mut_checked()```.

```
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Dentro de “ö”
assert_eq!(None, s.split_at_checked(16));  // Além do final da string
```

## split_at_mut_checked()

```
split_at_mut_checked(
    &mut self,
    mid: usize,
) -> Option<(&mut str, &mut str)>
```

Divide uma slice mutável de string em duas.

O argumento, ```mid```, deve ser um deslocamento de byte do início da string. Ele também deve estar no limite de um ponto de código UTF-8.

As duas slices retornadas vão do início até ```mid```, e de ```mid``` até o final.

Para obter slices de string imutáveis, consulte o método ```split_at_checked()```.

```
let mut s = "Per Martin-Löf".to_string();
if let Some((first, last)) = s.split_at_mut_checked(3) {
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);

assert_eq!(None, s.split_at_mut_checked(13));  // Dentro de “ö”
assert_eq!(None, s.split_at_mut_checked(16));  // Além do fim da string
```

## chars()

```
chars(&self) -> Chars<'_>
```

Retorna um iterador sobre os caracteres de uma slice de string.

Como uma slice de string consiste em UTF-8 válido, podemos iterar por uma slice de string por caractere. Este método retorna tal iterador.

É importante lembrar que ```char``` representa um Valor Escalar Unicode e pode não corresponder à sua ideia do que é um 'caractere'. A iteração sobre clusters de grafemas pode ser o que você realmente deseja. Esta funcionalidade não é fornecida pela biblioteca padrão do Rust, procure algo em crates.io.

### Exemplos

Uso básico:

```
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Lembre-se, ```char``` pode não corresponder à sua intuição sobre caracteres: 

```
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // não 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

## char_indices()

```
char_indices(&self) -> CharIndices<'_>
```

Retorna um iterador sobre os caracteres de uma slice de string e suas posições.

Como uma slice de string consiste em UTF-8 válido, podemos iterar por uma slice de string por caractere. Este método retorna um iterador para ambos, tanto para o caractere, como para sua posição de byte.

O iterador produz tuplas. A posição é o primeiro elemento da tupla, o caractere é o segundo.

### Exemplos

Uso básico:

```
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Lembre-se, ```char``` pode não corresponder à sua intuição sobre caracteres:

```
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // não (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note o 3 aqui - o caractere anterior ocupou dois bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

## bytes()

```
bytes(&self) -> Bytes<'_>
```

Retorna um iterador sobre os bytes de uma slice de string.

Como uma slice de string consiste em uma sequência de bytes, podemos iterar por uma slice de string por byte. Este método retorna tal iterador.

```
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

## split_whitespace()

```
split_whitespace(&self) -> SplitWhitespace<'_>
```

Divide uma slice de string conforme os espaços em branco.

O iterador retornado retornará slices de string que são subslices da slice de string original, separadas por qualquer quantidade de espaço em branco.

Espaços em branco (whitespaces) são definidos de acordo com os termos da Unicode Derived Core Property ```White_Space```. Se você quiser dividir apenas em espaço em branco ASCII, use ```split_ascii_whitespace()```.

### Exemplos

Uso básico:

```
et mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Todos os tipos de espaços em branco são considerados:

```
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

Se a string estiver vazia ou cheia de espaços em branco, o iterador não produzirá nenhuma slice de string:

```
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

## split_ascii_whitespace()

```
split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>
```

Divide uma slice de string conforme os espaços em branco ASCII.

O iterador retornado retornará slices de string que são subslices da slice de string original, separadas por qualquer quantidade de espaço em branco ASCII.

Para dividir por espaço em branco Unicode, use ```split_whitespace()```.

### Exemplos

Uso básico:

```
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Todos os tipos de espaços em branco ASCII são considerados:

```
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

Se a string estiver vazia ou cheia de espaços em branco ASCII, o iterador não produzirá nenhuma slice de string:

```
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

## lines()

```
lines(&self) -> Lines<'_>
```

Retorna um iterador sobre as linhas de uma string, como slices de string.

As linhas são divididas em finais de linha que são novas linhas (\n) ou sequências de um retorno de carro seguido por uma quebra de linha (\r\n).

Os terminadores de linha não são incluídos nas linhas retornadas pelo iterador.

Observe que qualquer retorno de carro (\r) não seguido imediatamente por uma quebra de linha (\n) não divide uma linha. Esses retornos de carro são, portanto, incluídos nas linhas produzidas.

O final da linha final é opcional. Uma string que termina com um final de linha final retornará as mesmas linhas que uma string idêntica sem um final de linha final.

### Exemplos

Uso básico:

```
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// O retorno de carro final é incluído na última linha
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

A linha final não requer nenhum final:

```
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

## contains()

```
contains<P>(&self, pat: P) -> bool
```

Retorna ```true``` se o padrão fornecido(```pat```) corresponde a uma subslice desta slice de string.

Retorna ```false``` se não corresponder.

O padrão pode ser uma ```&str```, um ```char```, uma slice de chars ou uma função ou closure que determina se um caractere corresponde.

```
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

## starts_with()

```
starts_with<P>(&self, pat: P) -> bool
where
    P: Pattern,
```
 
Retorna ```true``` se o padrão fornecido (```pat```) corresponde a um prefixo da slice de string; senão retorna ```false```.

O padrão pode ser uma ```&str```, nesse caso, esta função retornará ```true``` se a ```&str``` for um prefixo da slice de string. 

O padrão também pode ser um caractere, uma slice de caracteres ou uma função ou closure que determina se um caractere corresponde. Neste caso, serão verificados apenas contra o primeiro caractere da slice de string. Veja o segundo exemplo abaixo sobre este comportamento. 

```
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

```
let bananas = "bananas";

// Observe que ambos são declarados com sucesso.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

## ends_with()

```
ends_with<P>(&self, pat: P) -> bool
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna ```true``` se o padrão fornecido (```pat```) corresponde a um sufixo da slice de string.

Retorna ```false``` se não corresponder.

O padrão pode ser uma ```&str```, ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

```
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

## find()

```
find<P>(&self, pat: P) -> Option<usize>
where
    P: Pattern,
``` 

Retorna o índice de byte do primeiro caractere desta slice de string que corresponde ao padrão passado como argumento(```pat```).

Retorna ```None``` se o padrão não corresponder.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Exemplos

Padrões simples:

```
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

Padrões mais complexos usando estilo sem ponto e closures: 

```
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Não encontrando o padrão:

```
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

## rfind()

```
rfind<P>(&self, pat: P) -> Option<usize>
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna o índice de byte para o primeiro caractere da última correspondência do padrão passado como argumento (```pat```).

Retorna ```None``` se o padrão não corresponder.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Exemplos

Padrões simples:

```
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

Padrões mais complexos com closures:

```
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Não encontrando o padrão:

```
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

## split()

```
 split<P>(&self, pat: P) -> Split<'_, P> ⓘ
where
    P: Pattern,
```
 
Retorna um iterador sobre as substrings desta slice de string, separadas por caracteres que correspondem ao padrão passado como argumento (```pat```)

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars, ou uma função ou closure que determina se um caractere corresponde. 

### Comportamento do iterador

O iterador retornado será um ```DoubleEndedIterator``` se o padrão permitir uma pesquisa reversa e a pesquisa para frente/trás produzir os mesmos elementos. Isso é verdade para, por exemplo, ```char```, mas não para ```&str```.

Se o padrão permitir uma pesquisa reversa, mas seus resultados puderem diferir de uma pesquisa para frente, o método ```rsplit``` pode ser usado. 

### Exemplos

Padrões simples:

```
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

Se o padrão for uma slice de caracteres, divida em cada ocorrência de qualquer um dos caracteres:

```
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

Um padrão mais complexo, usando uma closure:

```
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

Se uma string contiver múltiplos separadores contíguos, você acabará com strings vazias na saída:

```
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Separadores contíguos são separados pela string vazia. 

```
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separadores no início ou no final de uma string são vizinhos de strings vazias. 

```
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

Quando a string vazia é usada como separador, ela separa cada caractere na string, juntamente com o início e o fim da string. 

```
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Separadores contíguos podem levar a um comportamento possivelmente surpreendente quando espaços em branco são usados como separador. Este código está correto:

```
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Ele não te dá: 

```
assert_eq!(d, &["a", "b", "c"]);
```

Como poderia ser esperado; use ```split_whitespace()``` para este comportamento.

## split_inclusive()

```
split_inclusive<P>(&self, pat: P) -> SplitInclusive<'_, P> ⓘ
where
    P: Pattern,
```

Retorna um iterador sobre as substrings desta slice de string, separadas por caracteres correspondentes ao padrão passado como argumento (```pat```).

Difere do iterador produzido por ```split()``` no sentido em que ```split_inclusive()``` deixa a parte correspondente como o terminador da substring.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Exemplos

```
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

Se o último elemento da string for correspondido, esse elemento será considerado o terminador da substring precedente. Essa substring será o último item retornado pelo iterador. 

```
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

## rsplit()

```
rsplit<P>(&self, pat: P) -> RSplit<'_, P> ⓘ
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna um iterador sobre as substrings da slice de string fornecida, separadas por caracteres correspondentes ao um padrão passado como argumento (```pat```) e produzidas em ordem inversa.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Comportamento do iterador

O iterador retornado exige que o padrão suporte uma pesquisa reversa, e será um ```DoubleEndedIterator``` se uma pesquisa para frente/trás gerar os mesmos elementos.

Para iterar a partir do início, o método ```split()``` pode ser usado.

### Exemplos

Padrão simples:

```
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

Um padrão mais complexo, usando uma closure:

```
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

## split_terminator()

```
split_terminator<P>(&self, pat: P) -> SplitTerminator<'_, P> ⓘ
where
    P: Pattern,
``` 
 
Retorna um iterador sobre as substrings da slice de string fornecida, separadas por caracteres correspondentes ao padrão passado como argumento (```pat```).

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

Equivalente a ```split()```, exceto que a substring final é ignorada se estiver vazia.

Este método pode ser usado para dados de string que são terminados, em vez de separados por um padrão. 
 
### Comportamento do iterador 

O iterador retornado será um ```DoubleEndedIterator``` se o padrão permitir uma pesquisa reversa e a pesquisa para frente/trás gerar os mesmos elementos. Isso é verdade para, por exemplo, ```char```, mas não para ```&str```.

Se o padrão permitir uma pesquisa reversa, mas seus resultados puderem diferir de uma pesquisa para frente, o método ```rsplit_terminator()``` pode ser usado. 

```
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

## rsplit_terminator()

```
rsplit_terminator<P>(&self, pat: P) -> RSplitTerminator<'_, P> ⓘ
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna um iterador sobre as substrings de si mesmo, separadas por caracteres correspondidos pelo padrão passado como argumento (```pat```) e produzidos em ordem inversa.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

Equivalente a ```split()```, exceto que a substring final é ignorada se estiver vazia.

Este método pode ser usado para dados de string que são terminados, em vez de separados por um padrão.

### Comportamento do iterador

O iterador retornado exige que o padrão suporte uma pesquisa reversa, e será de dupla extremidade se uma pesquisa para frente/trás produzir os mesmos elementos.

Para iterar a partir do início, o método ```split_terminator()``` pode ser usado.

```
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

## splitn()

```
splitn<P>(&self, n: usize, pat: P) -> SplitN<'_, P> ⓘ
where
    P: Pattern,
```

Retorna um iterador sobre as substrings da slice de string fornecida, separadas por um padrão, restrito a retornar no máximo ```n``` itens.

Se ```n``` substrings forem retornadas, a última substring (a n-ésima substring) conterá o restante da string.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Comportamento do iterador

O iterador retornado não será de duas extremidades, pois não é eficiente para suportar.

Se o padrão permitir uma pesquisa reversa, o método ```rsplitn()``` pode ser usado.

### Exemplos

Padrão simples:

```
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

Um padrão mais complexo, usando uma closure:

```
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

## rsplitn()

```
rsplitn<P>(&self, n: usize, pat: P) -> RSplitN<'_, P> ⓘ
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna um iterador sobre as substrings da slice de string fornecida, separadas por um padrão, iniciando no final da slice de string, restrito a retornar no máximo ```n``` itens.

Se ```n``` substrings forem retornadas, a última substring (a n-ésima substring) conterá o restante da string.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de chars ou uma função ou closure que determina se um caractere corresponde.

### Comportamento do iterador

O iterador retornado não será de duas extremidades, pois não é eficiente para suportar.

Se separar a slice de string a partir do início, o método ```splitn()``` pode ser usado.

### Exemplos

Padrão simples:

```
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

Um padrão mais complexo, usando uma closure:

```
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

## split_once()

```
split_once<P>(&self, delimiter: P) -> Option<(&str, &str)>
where
    P: Pattern,
```

Divide a ```str``` na primeira ocorrência do delimitador especificado e retorna o prefixo antes do delimitador e o sufixo após o delimitador. 

```
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

## rsplit_once()

```
rsplit_once<P>(&self, delimiter: P) -> Option<(&str, &str)>
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Divide a ```str``` na última ocorrência do delimitador especificado e retorna o prefixo antes do delimitador e o sufixo após o delimitador.

```
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

## matches()

```
matches<P>(&self, pat: P) -> Matches<'_, P> 
where
    P: Pattern,
```

Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento (```pat```).

O padrão pode ser uma ```&str```, um ```char```, uma slice de chars ou uma função ou closure que determina se um caractere corresponde. 

### Comportamento do iterador

O iterador retornado será um ```DoubleEndedIterator``` se o padrão permitir uma pesquisa reversa e a pesquisa para frente/trás gerar os mesmos elementos. Isso é verdade para, por exemplo, ```char```, mas não para ```&str```.

Se o padrão permitir uma pesquisa reversa, mas seus resultados puderem diferir de uma pesquisa para frente, o método ```rmatches()``` pode ser usado.

```
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

## rmatches()

```
rmatches<P>(&self, pat: P) -> RMatches<'_, P> 
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento (```pat```), iniciando no final da ```str```.

O padrão pode ser uma ```&str```, um ```char```, uma slice de chars ou uma função ou closure que determina se um caractere corresponde. 

### Comportamento do iterador

O iterador retornado será um ```DoubleEndedIterator``` se o padrão permitir uma pesquisa reversa e a pesquisa para frente/trás gerar os mesmos elementos. Isso é verdade para, por exemplo, ```char```, mas não para ```&str```.

Se o padrão permitir uma pesquisa reversa, mas seus resultados puderem diferir de uma pesquisa para frente, o método ```rmatches()``` pode ser usado.

Para iterar a partir do ínicio da ```str```, o método ```matches()``` pode ser usado.

```
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

## match_indices()

```
match_indices<P>(&self, pat: P) -> MatchIndices<'_, P> 
where
    P: Pattern,
```

Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento (```pat```), assim como seus índices.

Para correspondências que se sobrepõe, apenas a primeira ocorrência será retornada.

O padrão pode ser uma ```&str```, um ```char```, uma slice de chars ou uma função ou closure que determina se um caractere corresponde.

### Comportamento do iterador

O iterador retornado será um ```DoubleEndedIterator``` se o padrão permitir uma pesquisa reversa e a pesquisa para frente/trás gerar os mesmos elementos. Isso é verdade para, por exemplo, ```char```, mas não para ```&str```.

Se o padrão permitir uma pesquisa reversa, mas seus resultados puderem diferir de uma pesquisa direta, o método ```rmatch_indices()``` pode ser usado.

```
    let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
    
    let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
    assert_eq!(v, [(1, "abc"), (4, "abc")]);
    
    let v: Vec<_> = "ababa".match_indices("aba").collect();
    assert_eq!(v, [(0, "aba")]); // only the first `aba`

    let v: Vec<_> = "1abc2abc3".match_indices(char::is_numeric).collect();
    assert_eq!(v, [(0,"1"),(4,"2"),(8,"3")]);
```

## rmatch_indices()

```
rmatch_indices<P>(&self, pat: P) -> RMatchIndices<'_, P> 
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```` 

Retorna um iterador sobre as subslices que correspondem ao padrão passado como argumento (```pat```), assim como seus índices, em ordem reversa.

Para correspondências que se sobrepõe, apenas a primeira ocorrência será retornada.

O padrão pode ser uma ```&str```, um ```char```, uma slice de chars ou uma função ou closure que determina se um caractere corresponde.

### Comportamento do iterador

O iterador retornado exige que o padrão suporte uma pesquisa reversa, e será um ```DoubleEndedIterator``` se uma pesquisa para frente/trás gerar os mesmos elementos.

Para iterar a partir do início, o método ```match_indices()``` pode ser usado.

```
    let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
    assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
    
    let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
    assert_eq!(v, [(4, "abc"), (1, "abc")]);
    
    let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
    assert_eq!(v, [(2, "aba")]); // only the last `aba`

    let v: Vec<_> = "1abc2abc3".rmatch_indices(char::is_numeric).collect();
    assert_eq!(v, [(8,"3"),(4,"2"),(0,"1")]);
```

## trim()

```
trim(&self) -> &str
```

Retorna uma slice de string com espaços em branco iniciais e finais removidos.

"Espaço em branco" é definido de acordo com os termos da Unicode Derived Core Property ```White_Space```, que inclui novas linhas.

```
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

## trim_start()

```
trim_start(&self) -> &str
```

Retorna uma slice de string com os espaços em branco iniciais removidos.

"Espaço em branco" é definido de acordo com os termos da Unicode Derived Core Property ```White_Space```, que inclui novas linhas. 

### Direcionalidade do texto

Uma string é uma sequência de bytes. "início" neste contexto significa a primeira posição dessa string de bytes; para uma linguagem da esquerda para a direita como inglês ou russo, isso será o lado esquerdo, e para linguagens da direita para a esquerda como árabe ou hebraico, isso será o lado direito.

### Exemplos

Uso básico:

```
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Direcionalidade:

```
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

## trim_end()

```
trim_end(&self) -> &str
```

Retorna uma slice de string com espaços em branco finais removidos.

"Espaço em branco" é definido de acordo com os termos da Unicode Derived Core Property ```White_Space```, que inclui novas linhas.

### Direcionalidade do texto

Uma string é uma sequência de bytes. "Fim", neste contexto, significa a última posição dessa string de bytes; para uma linguagem da esquerda para a direita como inglês ou russo, isso será o lado direito, e para linguagens da direita para a esquerda como árabe ou hebraico, isso será o lado esquerdo.

### Exemplos

Uso básico:

```
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Direcionalidade:

```
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

## trim_matches()

```
trim_matches<P>(&self, pat: P) -> &str
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> DoubleEndedSearcher<'a>,
```

Retorna uma slice de string com todos os prefixos e sufixos que correspondem a um padrão removido repetidamente.

O padrão pode ser um caractere, uma slice de caracteres ou uma função ou closure que determina se um caractere corresponde.

### Exemplos

Padrão simples:

```
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

Padrão mais complexo, usando uma closure:

```
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

## trim_start_matches()

```
trim_start_matches<P>(&self, pat: P) -> &str
where
    P: Pattern,
```
 
Retorna uma slice de string com todos os prefixos que correspondem a um padrão removidos repetidamente.

O padrão pode ser uma ```&str```, um ```char```, uma slice de ```char``` ou uma função ou closure que determina se um caractere corresponde.

### Direcionalidade do texto

Uma string é uma sequência de bytes. "início" neste contexto significa a primeira posição dessa string de bytes; para uma linguagem da esquerda para a direita como inglês ou russo, isso será o lado esquerdo, e para linguagens da direita para a esquerda como árabe ou hebraico, isso será o lado direito.
 
```
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

## strip_prefix()

```
strip_prefix<P>(&self, prefix: P) -> Option<&str>
where
    P: Pattern,
``` 

Retorna uma slice de string com o prefixo removido.

Se a string começar com o prefixo, retorna a substring após o prefixo, envolto em ```Some```. Ao contrário de ```trim_start_matches()```, este método remove o prefixo exatamente uma vez.

Se a string não começar com o prefixo, retorna ```None```.

O prefixo pode ser uma ```&str```, um ```char```, uma slice de ```char``` ou uma função ou closure que determina se um caractere corresponde.

```
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

## strip_suffix()

```
strip_suffix<P>(&self, suffix: P) -> Option<&str>
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
``` 
Retorna uma slice de string com o sufixo removido.

Se a string termina com o sufixo, retorna a substring antes do sufixo, envolto em ```Some```. Ao contrário de ```trim_end_matches()```, este método remove o sufixo exatamente uma vez.

Se a string não termina com o sufixo, retorna ```None```.

O padrão pode ser uma ```&str```, um ```char```, uma slice de ```char``` ou uma função ou closure que determina se um caractere corresponde.

```
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

## trim_end_matches()

```
trim_end_matches<P>(&self, pat: P) -> &str
where
    P: Pattern,
    <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,
```

Retorna uma slice de string com todos os sufixos que correspondem a um padrão removidos repetidamente.

O padrão pode ser uma ```&str```, um ```char```, uma fatia de ```char``` ou uma função ou closure que determina se um caractere corresponde.

### Direcionalidade do texto

Uma string é uma sequência de bytes. "Fim", neste contexto, significa a última posição dessa string de bytes; para uma linguagem da esquerda para a direita como inglês ou russo, isso será o lado direito, e para linguagens da direita para a esquerda como árabe ou hebraico, isso será o lado esquerdo.


### Exemplos

Padrão simples:

```
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

Um padrão mais complexo, usando closure:

```
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

## parse()

```
parse<F>(&self) -> Result<F, <F as FromStr>::Err>
where
    F: FromStr,
```

Converte a ```str``` em outro tipo.

Como pretende converter para qualquer tipo,  ```parse()``` pode causar problemas com inferência de tipo. Assim, ```parse()``` é uma das poucas vezes que você verá a sintaxe carinhosamente conhecida como "turbofish": ```::<>```. Isso ajuda o algoritmo de inferência a entender especificamente em qual tipo você está tentando analisar.

```parse()``` pode converter para qualquer tipo que implemente a trait ```FromStr```.

### Erro

Retornará ```Err``` se não for possível analisar esta slice de string no tipo desejado.

### Exemplos

Uso básico:

```
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Usando "turbofish":

```
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Falha ao tentar converter:

```
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

## is_ascii()

```
is_ascii(&self) -> bool
```
 
Verifica se todos os caracteres nesta string estão dentro do intervalo ASCII. 

```
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

## eq_ignore_ascii_case()

```
eq_ignore_ascii_case(&self, other: &str) -> bool
```

Verifica se duas strings correspondem em maiúsculas e minúsculas ASCII.

Igual a ```to_ascii_lowercase(a) == to_ascii_lowercase(b)```, mas sem alocar e copiar temporários.

```
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

## make_ascii_uppercase()

```
make_ascii_uppercase(&mut self)
```

Converte esta string para seu equivalente em maiúsculas ASCII no local.

Letras ASCII 'a' a 'z' são mapeadas para 'A' a 'Z', mas letras não ASCII permanecem inalteradas.

Para retornar um novo valor em maiúsculas sem modificar o existente, use ```to_ascii_uppercase()```.

```
let mut s = String::from("Grüße, Jürgen ❤");

s.make_ascii_uppercase();

assert_eq!("GRüßE, JüRGEN ❤", s);
```

## make_ascii_lowercase()

```
make_ascii_lowercase(&mut self)
```
 
Converte esta string para seu equivalente em minúsculas ASCII no local.

Letras ASCII 'A' a 'Z' são mapeadas para 'a' a 'z', mas letras não ASCII permanecem inalteradas.

Para retornar um novo valor em minúsculas sem modificar o existente, use ```to_ascii_lowercase()```.

```
let mut s = String::from("GRÜßE, JÜRGEN ❤");

s.make_ascii_lowercase();

assert_eq!("grÜße, jÜrgen ❤", s);
```

## trim_ascii_start()

```
trim_ascii_start(&self) -> &str
```

Retorna uma slice de string com os espaços em branco ASCII iniciais removidos.

"Espaço em branco" refere-se à definição usada por ```u8::is_ascii_whitespace```.

```
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

## trim_ascii_end()

```
trim_ascii_end(&self) -> &str
``` 
Retorna uma slice de string com espaços em branco ASCII finais removidos.

"Espaço em branco" refere-se à definição usada por ```u8::is_ascii_whitespace```.

```
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

## trim_ascii()

```
trim_ascii(&self) -> &str
``` 

Retorna uma slice de string com espaços em branco ASCII iniciais e finais removidos.

"Espaço em branco" refere-se à definição usada por ```u8::is_ascii_whitespace```.

```
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

## escape_debug()

```
escape_debug(&self) -> EscapeDebug<'_>
``` 

Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_debug```.

Observação: apenas os pontos de código de grafema estendidos que iniciam a string serão escapados.

### Exemplos

Como um iterador:

```
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Saída:

```
❤\n!
```

Usando ```println!``` diretamente:

```
println!("{}", "❤\n!".escape_debug());
```

Saída:

```
❤\n!
```

Os dois exemplos acima são equivalentes a:

```
println!("❤\\n!");
```

Usando ```to_string()```:

```
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

## escape_default()

```
escape_default(&self) -> EscapeDefault<'_>
```
 
Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_default```.

### Exemplos

Como um iterador:

```
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Saída:

```
\u{2764}\n!
```

Usando ```println!``` diretamente:

```
println!("{}", "❤\n!".escape_default());
```

Saída:

```
\u{2764}\n!
```

Os dois exemplos acima são equivalentes a:

```
println!("\\u{{2764}}\\n!");
```

Usando ```to_string()```:

```
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

## escape_unicode()

```
escape_unicode(&self) -> EscapeUnicode<'_>
``` 

Retorna um iterador que escapa cada caractere em ```self``` com ```char::escape_unicode```.

### Exemplos

Como um iterador:

```
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Usando ```println!``` diretamente:

```
println!("{}", "❤\n!".escape_unicode());
```

Ambos exemplos acima são equivalentes a:

```
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Usando ```to_string()```:

```
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

## into_boxed_bytes()

```
into_boxed_bytes(self: Box<str>) -> Box<[u8]>
```
 
Converte um ```Box<str>``` em um ```Box<[u8]>``` sem copiar ou alocar.

```
let s = "this is a string";
let boxed_str = s.to_owned().into_boxed_str();
let boxed_bytes = boxed_str.into_boxed_bytes();
assert_eq!(*boxed_bytes, *s.as_bytes());
```

## replace()

```
replace<P>(&self, from: P, to: &str) -> String
where
    P: Pattern,
``` 

Substitui todas as correspondências de um padrão por outra string.

```replace()``` cria uma nova ```String``` e copia os dados desta fatia de string para ela. Ao fazer isso, tenta encontrar correspondências de um padrão. Se encontrar alguma, substitui pela slice de string de substituição. 

### Exemplos

Uso básico:

```
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

Quando o padrão não corresponde, ```replace()``` retorna esta slice de string como String:

```
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

## replacen()

```
replacen<P>(&self, pat: P, to: &str, count: usize) -> String
where
    P: Pattern,
```
 
Substitui as primeiras N correspondências de um padrão por outra string.

```replacen()``` cria uma nova ```String``` e copia os dados desta slice de string para ela. Ao fazer isso, tenta encontrar correspondências de um padrão. Se encontrar alguma, substitui pela slice de string de substituição no máximo ```count``` vezes.

### Exemplos

Uso básico:

```
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

Quando o padrão não corresponde, retorna esta slice de string como ```String```:

```
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

## to_lowercase()

```
to_lowercase(&self) -> String
```
 
Retorna o equivalente em minúsculas desta slice de string, como uma nova ```String```.

"Minúsculas" é definido de acordo com os termos da Unicode Derived Core Property ```Lowercase```.

Como alguns caracteres podem se expandir para vários caracteres ao mudar para minúsculos, esta função retorna uma ```String``` em vez de modificar o parâmetro no local.

### Exemplos

Uso básico:

```
let s = "HELLO";

assert_eq!("hello", s.to_lowercase());
```

Um exemplo complicado, com sigma:

```
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// mas, no fim da palavra é ς, não σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
```

Linguagens sem minúsculas não são alteradas:

```
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

## to_uppercase()

```
to_uppercase(&self) -> String
```

Retorna o equivalente em maiúsculas desta slice de string, como uma nova ```String```.

"Maiúsculas" é definido de acordo com os termos da Unicode Derived Core Property ```Uppercase```.

Como alguns caracteres podem se expandir para vários caracteres ao mudar para maiúsculos, esta função retorna uma ```String``` em vez de modificar o parâmetro no local.

### Exemplos

Uso básico:

```
let s = "hello";

assert_eq!("HELLO", s.to_uppercase());
```

Idiomas sem maiúsculas não são alterados:

```
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

Um caractere pode se converter em muitos:

```
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

## into_string()

```
into_string(self: Box<str>) -> String
``` 

asd

---

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

<br>

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

---

## Referências

[std::str](https://doc.rust-lang.org/std/primitive.str.html)

---

arataca89@gmail.com

Última atualização: 20241212
