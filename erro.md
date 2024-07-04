# Tratamento de erros em Rust

O tratamento de de erros em programação pode ser basicamente dividido em dois ramos: ```manipulação de exceções``` e ```retorno de valores```. Rust opta por retornar valores. 

## Básico
Você pode pensar no tratamento de erros como sendo o uso de análise de casos para determinar se uma determinada tarefa ou cálculo foi bem-sucedido ou não. Como você verá, a chave para o tratamento ergonômico de erros é reduzir a quantidade de análise  de casos explícita que o programador precisa fazer enquanto mantém o código combinável.

Código combinável refere-se a um código onde os componentes ou módulos de software são organizados de maneira a serem facilmente combinados para criar novas funcionalidades. A ideia de código combinável defende a criação de pequenas unidades independentes que sejam tratadas como blocos de construção para sistemas maiores.

Manter o código combinável é importante porque, sem esse requisito, poderíamos entrar em pânico sempre que nos deparássemos com algo inesperado. (o pânico faz com que a tarefa atual seja interrompida e, na maioria dos casos, todo o programa é abortado.) Aqui está um exemplo:

```
// Adivinhe um número entre 1 e 10.
// Se seu número corresponde ao que temos em mente, retorne 'True'.
// Senão, retorne 'False'.
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Número inválido: {}", n);
    }
    n == 5
}

fn main() {
    guess(11);
}

```
Se você tentar executar este código, o programa irá travar com uma mensagem como esta:
```
thread 'main' panicked at src/main.rs:7:9:
Número inválido: 11
```
Aqui está outro exemplo. Um programa que aceita um número inteiro como argumento, duplica-o e imprime-o:
```
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // erro 1
    let n: i32 = arg.parse().unwrap(); // erro 2
    println!("{}", 2 * n);
}

```
Se você fornecer zero argumentos a este programa (erro 1) ou se o primeiro argumento não for um número inteiro (erro 2), o programa entrará em pânico como no primeiro exemplo.

## Unwrapping
No exemplo anterior, afirmamos que o programa simplesmente entraria em pânico se atingisse uma das duas condições de erro, mas o programa não inclui uma chamada explícita a ```panic!``` como no primeiro exemplo. Isso ocorre porque panic! está embutida nas chamadas a ```unwrap```.

“unwrap” algo em Rust é dizer: “Dê-me o resultado do cálculo e, se houver um erro, entre em pânico e pare o programa”. Seria melhor se mostrássemos o código de unwrap porque é muito simples, mas para fazer isso, primeiro precisaremos explorar os tipos ```Option``` e ```Result```. Ambos possuem um método chamado unwrap definido neles.

## Option
O tipo ```Option``` é definido na [biblioteca padrão](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/option/enum.Option.html):
```
enum Option<T> {
    None,
    Some(T),
}
```
O tipo ```Option``` é uma forma de usar o sistema de tipos do Rust para expressar a possibilidade de ausência. Codificar a possibilidade de ausência no sistema de tipos é um conceito importante porque fará com que o compilador force o programador a lidar com essa ausência. Vamos dar uma olhada em um exemplo que tenta encontrar um caractere em uma string:
```
// Procura em 'haystack' pelo caractere Unicode 'needle' . Se for encontrado,
// retorna o byte offset do caractere. Senão, 'None' é retornado.
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
```
Observe que quando esta função encontra um caractere correspondente, ela não retorna apenas o ```offset```. Em vez disso, retorna ```Some(offset)```. ```Some``` é uma variante ou construtor de valor para o tipo ```Option```. Você pode pensar nisso como uma função com o tipo ```fn<T>(value: T) -> Option<T>```. Da mesma forma, ```None``` também é um construtor de valor, exceto que não possui argumentos. Você pode pensar em ```None``` como uma função com o tipo ```fn<T>() -> Option<T>```.

Abaixo usamos a função ```find()``´:
```
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("Nenhuma extensão de arquivo encontrada."),
        Some(i) => println!("Extensão do arquivo: {}", &file_name[i+1..]),
    }
}
```
Este código usa correspondência de padrões para fazer análise de caso no ```Option<usize>``` retornado pela função ```find()```. Na verdade, a análise de caso é a única maneira de obter o valor armazenado dentro de um ```Option<T>```. Isso significa que você, como programador, deve lidar com o caso quando ```Option<T>``` for ```None``` em vez de ```Some(t)```.

Mas espere,  e com relação a ```unwrap```, que usamos anteriormente? Não houve análise de caso lá! Em vez disso, a análise do caso foi colocada dentro do método ```unwrap``` para você. Você mesmo pode definir se quiser:
```
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("chamada a `Option::unwrap()` em um valor `None`"),
        }
    }
}
```
O método ```unwrap``` abstrai a análise do caso. É exatamente isso que torna o uso de ```unwrap``` ergonômico. Infelizmente, esse ```panic!``` significa que ```unwrap``` não é combinável, não é adequado quando queremos escrever código combinável.

## Compondo valores ```Option<T>```
No exemplo anterior da função ```find()```, vimos como usar ```find()``` para descobrir a extensão em um nome de arquivo. É claro que nem todos os nomes de arquivos possuem o ```.``` (ponto) neles, então é possível que o nome do arquivo não tenha extensão. Esta possibilidade de ausência é codificada nos tipos usando ```Option<T>```. Em outras palavras, o compilador nos forçará a abordar a possibilidade de não existir uma extensão. No nosso caso, apenas imprimimos uma mensagem dizendo isso.

Obter a extensão de um nome de arquivo é uma operação bastante comum, então faz sentido colocá-la em uma função:
```
fn find(haystack: &str, needle: char) -> Option<usize> { haystack.find(needle) }
// Retorna a extensão do nome de arquivo dado, onde a extensão é definida
// como todos os caracteres depois do primeiro caractere `.` (ponto).
// Se `file_name` não tiver o caractere `.`, então `None` é retornado.
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}
```
(Dica: não use este código. Em vez disso, use o método [extension](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/path/struct.Path.html#method.extension) da biblioteca padrão.)

O código permanece simples, mas o importante a notar é que o tipo de ```find()``` nos obriga a considerar a possibilidade de ausência. Isso é bom porque significa que o compilador não nos deixará esquecer acidentalmente o caso em que o nome de um arquivo não possui extensão. Por outro lado, fazer análises explícitas de casos como fizemos em ```extension_explicit()``` todas as vezes pode ser um pouco cansativo.

Na verdade, a análise de caso em ```extension_explicit()``` segue um padrão muito comum: mapear uma função para o valor dentro de um ```Option<T>```, a menos que a opção seja ```None```, nesse caso, retorna ```None```.

Rust possui polimorfismo paramétrico, então é muito fácil definir um combinador que abstraia esse padrão:
```
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
```
Na verdade, ```map``` é [definido como um método](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/option/enum.Option.html#method.map) em ```Option<T>``` na biblioteca padrão. Como método, ele possui uma assinatura ligeiramente diferente: os métodos tomam ```self```, ```&self``` ou ```&mut self``` como seu primeiro argumento.

Armados com nosso novo combinador, podemos reescrever nosso método ```extension_explicit``` para nos livrarmos da análise de caso:
```
fn find(haystack: &str, needle: char) -> Option<usize> { haystack.find(needle) }
// Retorna a extensão de um dado nome de arquivo, onde a extensão é definida
// como todos os caracteres após o caractere '.' (ponto).
// Se `file_name` não possuir o caractere `.`, então `None` é retornado.
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
```
Um outro padrão que comumente encontramos é atribuir um valor padrão ao caso em que um valor de ```Option``` é ```None```. Por exemplo, talvez o seu programa assuma que a extensão de um arquivo é ```rs``` caso não tenha nenhuma extensão. Como você pode imaginar, a análise de caso para isso não é específica para extensões de arquivo - ela pode funcionar com qualquer ```Option<T>```:
```
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```
Como no ```map``` usado acima, a implementação da biblioteca padrão é um método em vez de uma função livre.

O truque aqui é que o valor padrão deve ter o mesmo tipo que o valor que pode estar dentro do ``` Option<T>```. Usá-lo é muito simples em nosso caso:
```
fn main() {
    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");
}
```
(Observe que ```unwrap_or``` é [definido como um método](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/option/enum.Option.html#method.unwrap_or) em ```Option<T>``` na biblioteca padrão, então usamos ele aqui em vez da função independente que definimos acima. Não se esqueça de verificar o método ```unwrap_or_else``` mais geral.)


### Referências
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#the-basics

https://www.howtogeek.com/devops/what-is-composable-code-and-how-can-you-create-it/#:~:text=Composable%20code%20describes%20classes%20and,more%20powerful%20higher-level%20constructs.
