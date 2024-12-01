# Linguagem Rust - String

```String``` é o tipo de string mais comum. Ele possui a propriedade do conteúdo da string, armazenado em um buffer alocado na memória heap. Ele está intimamente relacionado à sua contraparte emprestada, a ```str``` primitiva. 


* [Exemplos](#exemplos)
* [UFT8](#utf8)
* [Deref](#deref)


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

asd

---

## Referências

[std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)

---

arataca89@gmail.com

Última atualização: 20241019
