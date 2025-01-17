#### arataca89

## Rust - dicas

[Desabilitar as mensagens de warning](#desabilitar-as-mensagens-de-warning)

[Converter um Vec&lt;char&gt; em uma String](#converter-um-vecchar-em-uma-string)

[Ler um arquivo texto para uma String](#ler-um-arquivo-texto-para-uma-string)

[Implementar Display para imprimir com ```{}```](#implementar-display-para-imprimir-com-)

[Diferença entre clone() e to_owned()](#diferença-entre-clone-e-to_owned)

[Implementar um smart pointer](#implementar-um-smart-pointer)

[Lendo argumentos da linha de comando](#lendo-argumentos-da-linha-de-comando)

---

# Desabilitar as mensagens de warning
Insira a seguinte atributo no início do seu arquivo fonte
```rust
#![allow(warnings)]
```

Um atributo é um metadado geral e de forma livre que é interpretado de acordo com o nome, convenção, idioma e versão do compilador. Atributos internos, escritos com um ponto de exclamação (!) após a cerquilha (#), aplicam-se ao item dentro do qual o atributo é declarado. Atributos externos, escritos sem o ponto de exclamação após a cerquilha, aplicam-se à coisa que segue o atributo. 

[Atributos](https://doc.rust-lang.org/reference/attributes.html)

---

# Converter um Vec&lt;char&gt; em uma String

Se não for precisar mais do vetor pode usar ```into_inter()``` e ```collect()```.

```rust
fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let s: String = v.into_iter().collect();
    println!("{}", s);
    //println!("{:?}", v);// ERRO. v foi movido devido a into_iter()
}
```

Se for usar o vetor depois pode usar ```iter()``` e ```collect()```.

```rust
fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let s: String = v.iter().collect();
    println!("{}", s);
    println!("{:?}", v);
}
```

Fonte: [https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string](https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string)

---

# Ler um arquivo texto para uma String

```rust
use std::fs;
use std::io;
/********************************************************************
fs::read_to_string()
    Lê todo o conteúdo de um arquivo para uma String.
********************************************************************/
fn main(){
    let texto =  fs::read_to_string("lorem.txt");
    match texto {
        Ok(s)  => println!("{s}"),
        Err(e) => println!("Erro: {}", e),
    }
}
/********************************************************************
Se o arquivo 'lorem.txt' existir, seu conteudo será exibido,
senão a saída será:

Erro: O sistema não pode encontrar o arquivo especificado. (os error 2)
********************************************************************/
```

[std::read_to_string()](https://doc.rust-lang.org/beta/std/fs/fn.read_to_string.html)

---

# Implementar Display para imprimir com ```{}```


```rust
// Importa o módulo 'fmt'
use std::fmt;

// Estrutura que implementará 'fmt::Dispaly'.
struct Structure(i32);

// Para imprimir usando '{}', o tipo tem que implementar 'fmt::Display'
impl fmt::Display for Structure {
    // Esta trait requer o método 'fmt' com esta assinatura.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

---

# Diferença entre clone() e to_owned()

Em Rust, tanto `clone()` quanto `to_owned()` são usados para criar cópias de dados, mas eles têm propósitos e comportamentos ligeiramente diferentes, dependendo do contexto em que são utilizados.

### `clone()`
- **Propósito**: O método `clone()` é usado para criar uma cópia profunda (deep copy) de um valor. Isso significa que ele copia tanto o valor em si quanto qualquer dado associado que o valor possua (por exemplo, se for uma `String`, ele copia a string inteira, incluindo seu conteúdo na heap).
- **Implementação**: O método `clone()` é definido na trait `Clone`, que pode ser implementada por tipos que desejam permitir a clonagem.
- **Uso comum**: `clone()` é frequentemente usado quando você precisa de uma cópia independente de um valor, especialmente quando o valor é armazenado na heap (como `String`, `Vec`, etc.).

Exemplo:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // s2 é uma cópia independente de s1
```

### `to_owned()`
- **Propósito**: O método `to_owned()` é usado para converter um tipo que pode ser referenciado (como uma fatia `&str`) em um tipo que possui seus dados (como `String`). Em outras palavras, ele cria uma cópia dos dados e os coloca em uma nova alocação na heap, transferindo a propriedade (ownership) para o novo valor.
- **Implementação**: O método `to_owned()` é definido na trait `ToOwned`, que é usada para tipos que podem ser convertidos em um tipo "owned" (ou seja, que possui seus dados).
- **Uso comum**: `to_owned()` é frequentemente usado para converter referências (como `&str`) em tipos owned (como `String`).

Exemplo:
```rust
let s1 = "hello"; // s1 é do tipo &str (uma fatia de string)
let s2 = s1.to_owned(); // s2 é do tipo String, uma cópia owned de s1
```

### Resumo das diferenças:
- **`clone`**: Cria uma cópia profunda de um valor, independentemente de ser uma referência ou um tipo owned. É mais geral e pode ser usado em qualquer tipo que implemente a trait `Clone`.
- **`to_owned`**: Converte uma referência (como `&str`) em um tipo owned (como `String`). É mais específico para conversões de referências para tipos owned.

Em resumo, `clone` é mais geral e pode ser usado para copiar qualquer tipo que implemente `Clone`, enquanto `to_owned` é mais específico para converter referências em tipos owned.

[deepseek](https://www.deepseek.com)

---

# Implementar um smart pointer

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

---

# Lendo argumentos da linha de comando

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("\nNúmero de argumentos passados: {}\n", args.len());
    for s in args{
        println!("{s}");
    }
}
```

Saída:

```
>cargo run -- arg1 arg2 arg3

Número de argumentos passados: 4

target\debug\to_pfix.exe
arg1
arg2
arg3
```

Note que o primeiro argumento passado (`args[0]`) é o nome do programa sendo executado.

---

arataca89@gmail.com

Última atualização: 20250117
