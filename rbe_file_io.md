#### arataca89

# RBE - Arquivos E/S (Entrada/Saída)

A estrutura ```File``` representa um arquivo que foi aberto (ela encapsula um descritor de arquivo) e fornece acesso de leitura e/ou gravação ao arquivo subjacente.

Como muitas coisas podem dar errado ao realizar E/S de arquivo, todos os métodos de ```File``` retornam o tipo ```io::Result<T>```, que é um alias para ```Result<T, io::Error>```.

Isso torna a falha de todas as operações de E/S explícita. Graças a isso, o programador pode ver todos os caminhos de falha e é incentivado a lidar com eles de forma proativa.

[File::open()](#open)  

[File::create()](#create)

[std::fs::OpenOptions](#openoptions)

[read_lines()](#read_lines)

---

## open 

A função ```open()``` pode ser usada para abrir um arquivo no modo somente leitura.

Um arquivo tem a propriedade  de um recurso, o descritor de arquivo, e cuida do fechamento do arquivo quando ele é descartado.

```
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	// Cria um caminho(path) para o arquivo desejado
    let path = Path::new("hello.txt");
    let display = path.display();

    // Abre o path no modo somente leitura, retorna `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Erro ao tentar abrir {}: {}", display, why),
        Ok(file) => file,
    };

    // Lê o arquivo  e coloca em uma string, retorna `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Erro ao tentar ler {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` sai do escopo e o arquivo "hello.txt" é fechado
}
```


```display()``` retorna um objeto que implementa ```Display``` para impressão segura de paths que podem conter dados não Unicode.

```
    use std::path::Path;
    
    let path = Path::new("/tmp/foo.rs");
    
    println!("{}", path.display());
```

Saída:

```
/tmp/foo.rs
```
 
## create

A função ```create()``` abre um arquivo no modo somente escrita. Se o arquivo já existir, o conteúdo antigo será destruído. Caso contrário, um novo arquivo será criado.

```
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Abre um arquivo no modo somente gravação, retorna `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Erro ao tentar criar o arquivo {}: {}", display, why),
        Ok(file) => file,
    };

     // Escreve a string `LOREM_IPSUM` em `file`, retorna `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("Erro ao tentar escrever no arquivo {}: {}", display, why),
        Ok(_) => println!("Sucesso ao escrever no arquivo {}", display),
    }
}
```

A estrutura [OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) pode ser usada para configurar como um arquivo é aberto. 

## OpenOptions

```
pub struct OpenOptions(/* private fields */);
```

Define opções e flags que podem ser usados ​​para configurar como um arquivo é aberto.

Este construtor configura como um ```File``` é aberto e quais operações são permitidas. Os métodos ```File::open()``` e ```File::create()``` são alias para opções comumente usadas usando este construtor.

Em geral, ao usar ```OpenOptions```, você primeiro chamará ```OpenOptions::new()```, depois encadeará chamadas a métodos para definir cada opção, depois chamará ```OpenOptions::open()```, passando o caminho(path) do arquivo que você está tentando abrir. Isso lhe dará um ```io::Result``` com um arquivo dentro no qual você pode operar posteriormente.

Abrindo um arquivo para leitura:

```
use std::fs::OpenOptions;

let file = OpenOptions::new().read(true).open("foo.txt");
```

Abrindo um arquivo para leitura e escrita, e também criando-o se ele não existir:

```
use std::fs::OpenOptions;

let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("foo.txt");
```

## read_lines()

#### Uma abordagem ingênua

Esta pode ser uma primeira tentativa razoável para uma primeira implementação de um iniciante para ler linhas de um arquivo. 

```
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
```

```std::fs::read_to_string()``` lê todo o conteúdo de um arquivo para uma string.  

```
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
```

```
use std::fs;
fn main(){
    match fs::read_to_string("lorem.txt"){
        Err(e) => panic!("{e}"),
        Ok(s) => println!("{s}"),
    } 
}
```

```lines()``` retorna um iterador sobre as linhas de uma string, como slices de string.

Como o método ```lines()``` retorna um iterador sobre as linhas do arquivo, também podemos executar um mapeamento inline e coletar os resultados, gerando uma expressão mais concisa e fluente.

```
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // emite 'panic!' em caso de erro ao ler o arquivo
        .lines()  // retorna um iterador para cada linha como uma &str
        .map(String::from)  // converte cada &str numa String
        .collect()  // coleta cada String criando um vetor
}
```

Observe que em ambos os exemplos acima, devemos converter a referência ```&str``` retornada de ```lines()``` para o tipo de propriedade ```String```, usando ```to_string()``` e ```String::from()```, respectivamente.

#### Uma abordagem mais eficiente 

Aqui, transferimos a propriedade do arquivo aberto para uma estrutura ```BufReader```. ```BufReader``` usa um buffer interno para reduzir alocações intermediárias.

Também atualizamos ```read_lines()``` para retornar um iterador em vez de alocar novos objetos ```String``` na memória para cada linha.

```
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // O arquivo hosts.txt deve existir 
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consome o iterador, retorna uma String (Opcional)
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// A saída é encapsulada em um 'Result' para permitir a correspondência em erros.
// Retorna um 'Iterator' para o 'Reader' das linhas do arquivo.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

Executar este programa simplesmente imprime as linhas individualmente. 

```
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

(Observe que, como ```File::open``` espera um ```AsRef<Path>``` genérico como argumento, definimos nosso método genérico ```read_lines()``` com a mesma restrição genérica, usando a palavra-chave ```where```.)

Este processo é mais eficiente do que criar uma ```String``` na memória com todo o conteúdo do arquivo. Isso pode causar problemas de desempenho, especialmente ao trabalhar com arquivos maiores.

```flatten()``` Cria um iterador que achata a estrutura aninhada. Isso é útil quando você tem um iterador de iteradores ou um iterador de coisas que podem ser transformadas em iteradores e você deseja remover um nível de indireção.
 

## Referências

[Rust by Example - File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)

[std::Path::display()](https://doc.rust-lang.org/std/path/struct.Path.html#method.display)

[std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

[std::fs::read_to_string()](https://doc.rust-lang.org/beta/std/fs/fn.read_to_string.html)

[std::Iterator::flatten()](https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.flatten)

[std::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)

---

arataca89@gmail.com

Última atualização: 20241222