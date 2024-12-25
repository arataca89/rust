#### arataca89

# RBE - Impressão formatada

[Introdução](#introdução)

[Debug](#debug)

[Display](#display)

[Exemplo List](#exemplo-list)

[Formatação](#formatação)

---

## Introdução

A impressão é gerenciada por uma série de ```macros``` definidas em ```std::fmt```, algumas das quais são:

* ```format!```: escreve texto formatado numa ```String```.
* ```print!```: igual a ```format!```, mas o texto é impresso no console (```io::stdout```).
* ```println!```: igual a ```print!``` mas um caractere de nova linha é adicionado ao final.
* ```eprint!```: igual a ```print!```, mas o texto é impresso na saída de erro padrão (```io::stderr```).
* ```eprintln!```: igual a ```eprint!``` mas com uma nova linha anexada ao final. 

Todos analisam o texto da mesma maneira. Como um bônus, Rust verifica a correção da formatação em tempo de compilação.

```
fn main() {
    // Em geral, o `{}` será automaticamente substituído por quaisquer
    // argumentos. Eles serão tranformados em string.
    println!("{} days", 31);

    // Argumentos posicionais podem ser usados. Especificar um inteiro dentro de `{}`
    // determina qual argumento adicional será substituído. Os argumentos começam
    // em 0 imediatamente após a string de formato.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Argumentos nomeados também são aceitos.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Uma formatação diferente pode ser invocada especificando o caractere de formato
    // após um `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // Você pode justificar à direita o texto com uma largura especificada. Isso irá
    // gerar "[    1]". (Quatro espaços em branco e um "1", para uma largura total de 5.)
    println!("[{number:>5}]", number=1);

    // Você pode preencher números com zeros extras,
    println!("[{number:0>5}]", number=1); // [00001]
    // e alinhar à esquerda invertendo o sinal. Isso produzirá "[10000]".
    println!("[{number:0<5}]", number=1); // [10000]

    // Você pode usar argumentos nomeados no especificador de formato acrescentando `$`.
    println!("[{number:0>width$}]", number=1, width=5); // [00001]

    // O Rust ainda verifica se o número correto de argumentos está sendo usado.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Adicione o argumento que falta: "James"

    // Somente tipos que implementam 'fmt::Display' podem ser formatados com `{}`.
    // Tipos definidos pelo usuário não implementam 'fmt::Display' por padrão.

    #[allow(dead_code)] // desabilita `dead_code` que avisa sobre módulo não utilizado
    struct Structure(i32);

    // Isso não será compilado porque `Structure` não implementa 'fmt::Display'.
    //println!("Esta struct `{}` não imprimirá...", Structure(3));
    // TODO ^ Tente descomentar esta linha

    // A partir do Rust 1.58, você pode capturar o argumento de uma variável externa a macro.
    // Assim como o acima, isso produzirá "[    1]", 4 espaços em branco e um "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("[{number:>width$}]");
}
```

```std::fmt``` contém muitas traits que governam a exibição de texto. As formas básicas de duas importantes são listadas abaixo:

* ```fmt::Debug```: Usa o marcador ```{:?}```. Formata texto para fins de depuração.
* ```fmt::Display```: Usa o marcador ```{}```. Formata texto de forma mais elegante e amigável ao usuário.

Aqui, usamos ```fmt::Display``` porque a biblioteca padrão fornece implementações para esses tipos. Para imprimir texto para tipos personalizados, são necessárias mais etapas.

Implementar a trait ```fmt::Display``` implementa automaticamente a trait ```ToString```, que permite converter o tipo para ```String```.

## Debug

Todos os tipos que desejam usar as traits de formatação ```std::fmt``` exigem uma implementação para serem imprimíveis. Implementações automáticas são fornecidas apenas para tipos da biblioteca padrão. Todos os outros devem ser implementados manualmente de alguma forma.

A trait ```fmt::Debug``` torna isso muito simples. Todos os tipos podem derivar, usando ```derive```, e criar automaticamente a implementação de  ```fmt::Debug```. Isso não é verdade para ```fmt::Display```, que deve ser implementado manualmente.

```
// Esta estrutura não pode ser impressa nem com 'fmt::Display'
// nem com  'fmt::Debug'.
struct UnPrintable(i32);

// O atributo 'derive' cria automaticamente uma implementação
// para imprimir esta estrutura com 'fmt::Debug'
#[derive(Debug)]
struct DebugPrintable(i32);
```
 
Todos os tipos da biblioteca padrão são automaticamente imprimíveis com ```{:?}```:

```
#![allow(warnings)] // desabilita mensagens de warning

// Deriva a implementação `fmt::Debug` para `Structure`. `Structure`
// é uma estrutura que contém um único `i32`.
#[derive(Debug)]
struct Structure(i32);

// Coloca uma `Structure` dentro da estrutura `Deep` que também é imprimível
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Imprimir com '{:?}' é similar a imprimir com '{}'
    println!("O ano tem {:?} meses.", 12);
    println!("{1:?} {0:?} é um {actor:?} ator.",
             "Slater",
             "Christian",
             actor="grande");

    // `Structure` é imprimível!
    println!("Imprimindo 'Structure': {:?}", Structure(3));

    // O problema com 'derive' é que não há controle sobre como
    // os resultados são exibidos. E se eu quiser que isso mostre apenas um '7'?
    println!("Imprimindo 'Deep': {:?}", Deep(Structure(7)));
}
```

Saída:

```
O ano tem 12 meses.
"Christian" "Slater" é um "grande" ator.
Imprimindo 'Structure': Structure(3)
Imprimindo 'Deep': Deep(Structure(7))
```

Observe que ```fmt::Debug``` junto com ```{:?}``` imprime numa forma para depuração que pode não ser o que queremos para nosso programa.

Se você quiser ter mais controle sobre a impressão deverá implementar ```fmt::Display```

## Display

```fmt::Debug``` dificilmente parece compacto e limpo, então muitas vezes é vantajoso personalizar a aparência da saída. Isso é feito implementando manualmente ```fmt::Display```, que usa o marcador de impressão ```{}```. Abaixo temos um exemplo de implemetação:

```
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

```fmt::Display``` pode ser mais limpo que ```fmt::Debug```, mas isso apresenta um problema para a biblioteca padrão. Como tipos ambíguos devem ser exibidos? Por exemplo, se a biblioteca padrão implementasse um único estilo para todos os ```Vec<T>```, qual estilo deveria ser? Seria um desses dois?

* ```Vec<path>```: /:/etc:/home/username:/bin (split em :)
* ```Vec<number>```: 1,2,3 (split em ,) 

Não, porque não existe um estilo ideal para todos os tipos e a biblioteca padrão não pretende ditar um. ```fmt::Display``` não é implementado para ```Vec<T>``` ou para qualquer outro contêiner genérico. ```fmt::Debug``` deve então ser usado para esses casos genéricos.

Porém, isso não é um problema porque para qualquer novo tipo de contêiner que não seja genérico, ```fmt::Display``` pode ser implementado.

```
// Importa 'fmt'
use std::fmt; 

// A trait 'Debug' será derivada de modo que os resultados sejam
// comparados com o resultado de 'Display'
#[derive(Debug)]
struct MinMax(i64, i64);

// Implementa 'Display'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Comparando as saídas:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("big: {big}\nsmall: {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Comparando as saídas:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.

    // Erro. Tanto `Debug` quanto `Display` foram implementados, mas `{:b}`
    // requer que `fmt::Binary` seja implementado. Isso não vai funcionar.
    //println!("Como é o Point2D em binário: {:b}?", point);
}
```


Saída:

```
// Importa 'fmt'
use std::fmt; 

// A trait 'Debug' será derivada de modo que os resultados sejam
// comparados com o resultado de 'Display'
#[derive(Debug)]
struct MinMax(i64, i64);

// Implementa 'Display'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Comparando as saídas:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("big: {big}\nsmall: {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Comparando as saídas:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.

    // Erro. Tanto `Debug` quanto `Display` foram implementados, mas `{:b}`
    // requer que `fmt::Binary` seja implementado. Isso não vai funcionar.
    //println!("Como é o Point2D em binário: {:b}?", point);
}
```

Saída sem o erro da última linha:

```
Comparando as saídas:
Display: (0, 14)
Debug: MinMax(0, 14)
big: (-300, 300)
small: (-3, 3)
Comparando as saídas:
Display: x: 3.3, y: 7.2
Debug: Point2D { x: 3.3, y: 7.2 }
```

Portanto, ```fmt::Display``` foi implementado, mas ```fmt::Binary``` não, e portanto não pode ser usado. ```std::fmt``` possui muitas dessas traits e cada uma requer sua própria implementação. 

## Exemplo List

Implementar ```fmt::Display``` para uma estrutura onde os elementos devem ser tratados sequencialmente é complicado. O problema é que cada ```write!``` gera um ```fmt::Result```. O tratamento adequado disso requer lidar com todos os resultados. Rust fornece o operador ```?``` exatamente para esse propósito.

Usar ```?``` em ```write!```se parece com isso:

```
// Tente `write!` para ver se há erro.
// Se houver erro, retorne o erro.
// Caso contrário, continue.
write!(f, "{}", value)?;
```

Com ```?```, implementar ```fmt::Display``` para um ```Vec``` é direto:

```
// Importa o módulo 'fmt'.
use std::fmt; 

// Define a estrutura 'List' que contém um 'Vec'.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.

        // Extrai o valor usando indexação de tupla,
        // e cria uma referência para `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Itera sobre 'v' em 'vec' enquanto conta a iteração em 'count'.
        for (count, v) in vec.iter().enumerate() {
            // Para cada elemento, exceto o primeiro, adiciona uma vírgula.
            // Usa o operador ? para retornar erro, se houver.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

Saída:

```
[1, 2, 3]
```

## Formatação

Vimos que a formatação é especificada por meio de uma string de formato: 

* format!("{}", foo) -> "3735928559"
* format!("0x{:X}", foo) -> "0xDEADBEEF"
* format!("0o{:o}", foo) -> "0o33653337357"

A mesma variável (**foo**) pode ser formatada de forma diferente dependendo do tipo de argumento usado: **X**, **o** ou não especificado.

Essa funcionalidade de formatação é implementada por meio de traits, e existe uma trait para cada tipo de argumento. A trait de formatação mais comum é ```Display```, que lida com casos em que o tipo de argumento não é especificado: ```{}``` por exemplo.

```
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` é um buffer, e este método deve escrever a string formatada nele.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` é como `format!`, mas escreverá a string formatada
        // em um buffer (o primeiro argumento).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{{},{},{}}}", self.red, self.green, self.blue)
    }   
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

    println!("");

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("Debug  : {:?}", color);
        println!("Display: {}", color);
        println!("");
    }
}
```

Saída:

```
Dublin: 53.348°N 6.260°W
Oslo: 59.950°N 10.750°E
Vancouver: 49.250°N 123.100°W

Debug  : Color { red: 128, green: 255, blue: 90 }
Display: {128,255,90}

Debug  : Color { red: 0, green: 3, blue: 254 }
Display: {0,3,254}

Debug  : Color { red: 0, green: 0, blue: 0 }
Display: {0,0,0}
```

Você pode ver uma [lista completa das traits de formatação](https://doc.rust-lang.org/std/fmt/#formatting-traits) e seus tipos de argumento na documentação de [std::fmt](https://doc.rust-lang.org/std/fmt/). 


## Referências

[Rust By Example - Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)

[std::format](https://doc.rust-lang.org/std/macro.format.html)

[std::fmt](https://doc.rust-lang.org/std/fmt/)

[std::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)

[std::write](https://doc.rust-lang.org/std/macro.write.html)

[std::Formatter](https://doc.rust-lang.org/std/fmt/struct.Formatter.html)

[std::fmt::Result](https://doc.rust-lang.org/std/fmt/type.Result.html)


---

arataca89@gmail.com

Última atualização: 20241225
