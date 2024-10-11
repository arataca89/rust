# Tratamento de erros em Rust

Como a maioria das linguagens de programação, Rust incentiva o programador a lidar com erros de uma maneira específica. Em termos gerais, o tratamento de erros é dividido em duas categorias amplas: exceções e valores de retorno. Rust opta por valores de retorno.

Nesta seção, pretendemos fornecer um tratamento abrangente de como lidar com erros em Rust. Mais do que isso, tentaremos introduzir o tratamento de erros uma parte de cada vez para que você saia com um conhecimento prático sólido de como tudo se encaixa.

Quando feito de forma ingênua, o tratamento de erros em Rust pode ser prolixo e irritante. Esta seção explorará esses obstáculos e demonstrará como usar a biblioteca padrão para tornar o tratamento de erros conciso e ergonômico.

* [Básico](#Básico)
	- [Explicando unwrap](#Explicando-unwrap)
	- [O tipo Option](#O-tipo-Option)
		- [Valores Option componíveis](#Valores-Option-componíveis)
* ASD
	- qwe


## Básico

Você pode pensar no tratamento de erros como usar análise de casos para determinar se uma computação foi bem-sucedida ou não. Como você verá, a chave para o tratamento de erros ergonômico é reduzir a quantidade de análise de casos explícita que o programador tem que fazer, mantendo o código componível. Um código componível refere-se a característica de componibilidade do software. Componibilidade se refere à capacidade de diferentes componentes ou elementos serem combinados ou conectados de várias maneiras para criar sistemas ou estruturas maiores e mais complexas. 

Manter o código componível é importante, pois sem esse requisito, poderíamos ter que acionar ```panic``` sempre que nos deparássemos com algo inesperado. (```panic``` faz com que a tarefa atual seja desfeita e, na maioria dos casos, o programa inteiro seja abortado.) Aqui está um exemplo: 

```
// Adivinha um número entre 1 e 10.
// Se o número for adivinhado retorna 'true'; senão retorna 'false.
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
thread 'main' panicked at src/main.rs:5:9:
```

Aqui está outro exemplo que é um pouco menos artificial. Um programa que aceita um inteiro como argumento, dobra o valor do inteiro e o imprime. 

```
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // erro 1
    let n: i32 = arg.parse().unwrap(); // erro 2
    println!("{}", 2 * n);
}
```

Se você não fornecer argumentos (erro 1) ou se o primeiro argumento não for um inteiro (erro 2), o programa entrará em pânico, como no primeiro exemplo. 

Você pode pensar nesse estilo de tratamento de erros como um touro correndo em uma loja de porcelana. O touro chegará aonde quer, mas causará estrago no processo. 


## Explicando unwrap

No exemplo anterior, afirmamos que o programa simplesmente entraria em pânico se atingisse uma das duas condições de erro, no entanto, o programa não inclui uma chamada explícita a ```panic!``` como o primeiro exemplo. Isso ocorre porque o pânico está embutido nas chamadas a ```unwrap```. 

Na linguagem Rust, executar "unwrap" em algo significa dizer: "Dê-me o resultado do processamento, e se houver um erro, entre em pânico e pare o programa." Seria melhor se mostrássemos o código de ```unwrap```, pois é muito simples, mas para fazer isso, primeiro precisamos explorar os tipos ```Option``` e ```Result```. Estes dois tipos têm um método chamado ```unwrap()``` definido neles. 

## O tipo Option

O tipo [Option](https://doc.rust-lang.org/std/option/enum.Option.html) é definido da biblioteca padrão.

```
enum Option<T> {
    None,
    Some(T),
}
```

O tipo ```Option``` é uma forma de usar o sistema de tipos do Rust para expressar a possibilidade de ausência. Codificar a possibilidade de ausência no sistema de tipos é um conceito importante porque fará com que o compilador force o programador a lidar com essa ausência. Vamos dar uma olhada em um exemplo que tenta encontrar um caractere em uma string:

```
// Procura pelo caractere Unicode 'agulha' em 'palheiro'.
// Se 'agulha' for encontrado, 'Some(n)'  é retornado;
// onde 'n' é o índice do caractere.
// Se 'agulha' não for encontrado 'None' é retornado.
fn find(palheiro: &str, agulha: char) -> Option<usize> {
    for (offset, c) in palheiro.char_indices() {
        if c == agulha {
            return Some(offset);
        }
    }
    None
}

#[test]
fn find_test(){
    assert_eq!(find("asdfg",'d'), Some(2));
    assert_eq!(find("asdfg",'z'), None);
}
```

Observe que quando esta função encontra um caractere correspondente, ela não retorna apenas o índice do caractere. Em vez disso, ela retorna ```Some(índice)```. ```Some``` é uma variante ou um construtor de valor para o tipo ```Option```. Você pode pensar nisso como uma função com o tipo ```fn<T>(valor: T) -> Option<T>```. Correspondentemente, ```None``` também é um construtor de valor, exceto que não tem argumentos. Você pode pensar em ```None``` como uma função com o tipo ```fn<T>() -> Option<T>```.

Isso pode parecer muito barulho por nada, mas esta é apenas metade da história. A outra metade é usar a função de busca que escrevemos. Vamos tentar usá-la para encontrar a extensão em um nome de arquivo. 

```
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("Não foi encontrada nenhuma extensão de arquivo."),
        Some(i) => println!("Extensão do arquivo: {}", &file_name[i+1..]),
    }
}
```

Este código usa [correspondência de padrões](patterns/README.md) para fazer análise de casos na ```Option<usize>``` retornada pela função ```find()```. Na verdade, a análise de casos é a única maneira de acessar o valor armazenado dentro de uma ```Option<T>```. Isso significa que você, como programador, deve analisar também o caso em que uma ```Option<T>``` é ```None``` em vez de ```Some(T)```.

Mas espere, e quanto ao ```unwrap```, que usamos anteriormente? Não houve análise de caso lá! Em vez disso, a análise de caso foi colocada dentro do método ```unwrap()``` para você. Você mesmo poderia definir o método ```unwrap()```:


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
              panic!("'Option::unwrap()' chamado em um valor 'None'"),
        }
    }
}
```

O método ```unwrap()``` abstrai a análise de casos. Esta é precisamente a coisa que torna ```unwrap``` ergonômico de usar. Infelizmente, esse ```panic!``` significa que ```unwrap``` não é componível: ele é o touro na loja de porcelana. 

## Valores Option componíveis

Em um exemplo anterior, vimos como usar ```find()``` para descobrir a extensão em um nome de arquivo. Claro, nem todos os nomes de arquivos têm um caractere ```.``` neles, então é possível que o nome do arquivo não tenha extensão. Essa possibilidade de ausência é codificada nos tipos usando ```Option<T>```. Em outras palavras, o compilador nos forçará a lidar com a possibilidade de que uma extensão não exista. No nosso caso, apenas imprimimos uma mensagem dizendo isso. 

Obter a extensão de um nome de arquivo é uma operação bastante comum, então faz sentido colocá-la em uma função: 

```
// Retorna a extensão em um nome de arquivo, onde a extensão
// é definida por todos os caracteres após o primeiro '.'.
// Se o nome do arquivo não tiver '.', 'None' é retornado.
fn get_extension(filename: &str) -> Option<&str> {
    match find(filename, '.') {
        None => None,
        Some(i) => Some(&filename[i+1..]),
    }
}

#[test]
fn get_extension_test(){
    let filename1 = "arquivo.ext";
    let filename2 = "arquivo";
    assert_eq!(get_extension(filename1), Some("ext"));
    assert_eq!(get_extension(filename2), None);
}
```

(Dica profissional: não use este código. Use o método [extension()](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension) da biblioteca padrão.) 



asd



## Referências
[https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#error-handling](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#error-handling)

[char_indices()](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices)

[extension()](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension)


---

arataca89@gmail.com

Última atualização: 20241011
