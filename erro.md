# Tratamento de erros em Rust

Como a maioria das linguagens de programação, Rust incentiva o programador a lidar com erros de uma maneira específica. Em termos gerais, o tratamento de erros é dividido em duas categorias amplas: exceções e valores de retorno. Rust opta por valores de retorno.

Nesta seção, pretendemos fornecer um tratamento abrangente de como lidar com erros em Rust. Mais do que isso, tentaremos introduzir o tratamento de erros uma parte de cada vez para que você saia com um conhecimento prático sólido de como tudo se encaixa.

Quando feito de forma ingênua, o tratamento de erros em Rust pode ser prolixo e irritante. Esta seção explorará esses obstáculos e demonstrará como usar a biblioteca padrão para tornar o tratamento de erros conciso e ergonômico.

* [Básico](#Básico)
	- [Explicando unwrap](#Explicando-unwrap)
	- [O tipo Option](#O-tipo-Option)
		- [Valores Option componíveis](#Valores-Option-componíveis)
	- [O tipo Result](#O-tipo-Result)
		- [Analisando inteiros](#Analisando-inteiros)
		- [Definindo alias para Result](#Definindo-alias-para-Result)
	- [Quando usar unwrap](#Quando-usar-unwrap)
* [Trabalhando com múltiplos tipos de erro](#Trabalhando-com-múltiplos-tipos-de-erro)
	- [Misturando Option e Result](#Misturando-Option-e-Result)
	- [Os limites dos combinadores](#-Os-limites-dos-combinadores)
	- [Retorno antecipado](#Retorno-antecipado)
	- [O operador ?](#O-operador-)
	- [Definindo seu próprio tipo de erro](#Definindo-seu-próprio-tipo-de-erro)
* asdfg
	- hjklç


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

[nth()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth) retorna o enésimo elemento do iterador.

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
    for (i, c) in palheiro.char_indices() {
        if c == agulha {
            return Some(i);
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

Observe que quando esta função encontra um caractere correspondente ela não retorna apenas o índice do caractere. Em vez disso, ela retorna ```Some(índice)```. ```Some``` é uma variante ou um construtor de valor para o tipo ```Option```. Você pode pensar nisso como uma função com o tipo ```fn<T>(valor: T) -> Option<T>```. Correspondentemente, ```None``` também é um construtor de valor, exceto que não tem argumentos. Você pode pensar em ```None``` como uma função com o tipo ```fn<T>() -> Option<T>```.

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

O código permanece simples, mas o importante a notar é que o tipo de retorno da função ```find()```nos força a considerar a possibilidade de ausência. Isso é bom porque significa que o compilador não nos deixará esquecer acidentalmente do caso em que um nome de arquivo não tem uma extensão. Por outro lado, fazer análise de caso explícita toda vez, como fizemos em ```get_extension()```, pode ficar um pouco cansativo. 

Na verdade, a análise de caso em ```get_extension()``` segue um padrão muito comum: mapear uma função para o valor dentro de uma ```Option<T>```, a menos que a opção seja ```None```, nesse caso, retornar ```None```.

Rust tem polimorfismo paramétrico, então é muito fácil definir um combinador que abstrai esse padrão: 

```
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
```
De fato, o método ```map()``` é definido como um método em ```Option<T>``` na biblioteca padrão. Como um método, ele tem uma assinatura ligeiramente diferente: métodos recebem ```self```, ```&self``` ou ```&mut self``` como seu primeiro argumento. 

Armados com nosso novo combinador, podemos reescrever nosso método ```get_extension()``` para nos livrar da análise de casos:

```
fn get_extension(filename: &str) -> Option<&str> {
    find(filename, '.').map(|i| &filename[i+1..])
}
```

Outro padrão que encontramos com frequência é atribuir um valor padrão ao caso em que um valor ```Option``` é ```None```. Por exemplo, talvez seu programa assuma que a extensão de um arquivo é ```rs``` mesmo que nenhuma extensão esteja presente. Como você pode imaginar, a análise de caso para isso não é específica para extensões de arquivo - pode funcionar com qualquer ```Option<T>```:

```
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```

Assim como em ```map()``` mostrado acima, a implementação da biblioteca padrão é um método em vez de uma função livre.

O truque aqui é que o valor padrão deve ter o mesmo tipo que o valor que pode estar dentro da ```Option<T>```. Usá-lo é muito simples no nosso caso: 

```
#[test]
fn unwrap_or_test(){
    assert_eq!(get_extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(get_extension("foobar").unwrap_or("rs"), "rs");
}
```

(Observe que [unwrap_or()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) é definido como um método em ```Option<T>``` na biblioteca padrão, então nós usamos isso aqui em vez da função independente que definimos acima. Não se esqueça de verificar também o método [unwrap_or_else()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) mais geral.) 

Existe mais um combinador que acreditamos valer a pena prestar atenção especial: ```and_then```. Ele facilita a composição de computações distintas que admitem a possibilidade de ausência. Por exemplo, muito do código nesta seção trata de encontrar uma extensão dado um nome de arquivo. Para fazer isso, você precisa primeiro do nome do arquivo, que normalmente é extraído de um path. Embora a maioria dos paths tenha um nome de arquivo, nem todos eles têm. Por exemplo, ```.```, ```..``` ou ```/```.

Então, somos encarregados do desafio de encontrar uma extensão dado um path. Vamos começar com a análise de caso explícita: 

```
fn get_extension_from_path(filepath: &str) -> Option<&str> {
    match filename(filepath) {
        None => None,
        Some(name) => match get_extension(name) {
            None => None,
            Some(ext) => Some(ext),
        }
    }
}

fn filename(filepath: &str) -> Option<&str> {
  // código omitido
  unimplemented!()
}
```

Você pode pensar que poderíamos usar o combinador ```map``` para reduzir a análise de caso, mas neste contexto não se encaixa exatamente... 

```
fn get_extension_from_path(filepath: &str) -> Option<&str> {
    filename(filepath).map(|x| get_extension(x)) // Isto provocará um erro de compilação.
}
```

A função ```map()``` aqui encapsula o valor retornado pela função ```get_extension()``` dentro de um ```Option<_>``` e, como a própria função ```get_extension()``` retorna um ```Option<&str>```, a expressão ```filename(filepath).map(|x| get_extension(x))``` retorna um ```Option<Option<&str>>```. 

Mas como ```get_extension_from_path()``` retorna apenas ```Option<&str>``` (e não ```Option<Option<&str>>```), obtemos um erro de compilação.
 
O resultado da função recebida por ```map()``` como entrada é sempre reencapsulado com ```Some```. Em vez disso, precisamos de algo como ```map```, mas que permita que o chamador retorne um ```Option<_>``` diretamente sem encapsulá-lo em outro ```Option<_>```.

Sua implementação genérica é ainda mais simples que map:

```
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
        where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
```

Agora podemos reescrever nossa função ```get_extension_from_path()``` sem análise de caso explícita: 

```
fn get_extension_from_path(filepath: &str) -> Option<&str> {
    filename(filepath).and_then(get_extension)
}
```

Nota lateral: Como o ```and_then``` funciona essencialmente como o ```map```, mas retorna um ```Option<_>``` em vez de um ```Option<Option<_>>```, ele é conhecido como ```flatMap``` em algumas outras linguagens. 

O tipo [Option](https://doc.rust-lang.org/std/option/enum.Option.html) possui muitos outros combinadores definidos na biblioteca padrão. É uma boa ideia dar uma olhada rápida nesta lista e se familiarizar com o que está disponível - eles podem frequentemente reduzir a análise de casos para você. Familiarizar-se com esses combinadores renderá dividendos porque muitos deles também são definidos (com semântica semelhante) para ```Result```, sobre o qual falaremos a seguir. 

Combinadores tornam o uso de tipos como ```Option``` ergonômico porque reduzem a análise de casos explícita. Eles também são compostos porque permitem que o chamador lide com a possibilidade de ausência à sua maneira. Métodos como ```unwrap()``` removem escolhas porque eles entram em pânico se ```Option<T>``` for ```None```.

## O tipo Result

O tipo [Result](https://doc.rust-lang.org/std/result/) também é definido na biblioteca padrão:

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

O tipo ```Result``` é uma versão mais rica de ```Option```. Em vez de expressar a possibilidade de ausência como ```Option``` faz, ```Result``` expressa a possibilidade de erro. Normalmente, o erro é usado para explicar por que a execução de algum processamento falhou. Esta é uma forma estritamente mais geral de ```Option```. Considere o seguinte alias de tipo, que é semanticamente equivalente ao real ```Option<T>``` em todos os sentidos: 

```
type Option<T> = Result<T, ()>;
```

Isso corrige o segundo parâmetro de tipo de ```Result``` para ser sempre ```()``` (pronunciado "unidade" ou "tupla vazia"). Exatamente um valor habita o tipo ```()```: ```()```. (Sim, os termos de nível de tipo e valor têm a mesma notação!) 

O tipo ```Result``` é uma forma de representar um dos dois resultados possíveis em um processamento. Por convenção, um resultado é considerado esperado ou "Ok", enquanto o outro resultado é considerado inesperado ou "Err". 

Assim como ```Option```, o tipo ```Result``` também possui um método [unwrap()](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) definido na biblioteca padrão. Vamos defini-lo: 

```
enum Result<T, E> { Ok(T), Err(E) }
impl<T, E: ::std::fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) =>
              panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}
```
Isso é efetivamente o mesmo que nossa definição para ```Option::unwrap()```, exceto que inclui o valor de erro na mensagem ```panic!``` Isso torna a depuração mais fácil, mas também exige que adicionemos uma restrição ```Debug``` ao parâmetro de tipo ```E``` (que representa nosso tipo de erro). Como a grande maioria dos tipos deve satisfazer a restrição ```Debug```, isso tende a funcionar na prática. (```Debug``` em um tipo simplesmente significa que existe uma maneira razoável de imprimir uma descrição legível por humanos de valores com esse tipo.) 

Ok, vamos passar para um exemplo. 

## Analisando inteiros

A biblioteca padrão do Rust torna a conversão de strings para inteiros extremamente simples. É tão fácil, na verdade, que é muito tentador escrever algo como o seguinte: 

```
fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let n: i32 = double_number("10");
    assert_eq!(n, 20);
}
```

Neste ponto, você deve ser cético em relação à chamada de ```unwrap()```. Por exemplo, se a string não for analisada como um número, você terá um pânico: 

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', /home/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libcore/result.rs:729
```

Isso é bastante feio, e se isso acontecesse dentro de uma biblioteca que você está usando, você poderia ficar compreensivelmente irritado. Em vez disso, devemos tentar lidar com o erro em nossa função e deixar o chamador decidir o que fazer. Isso significa mudar o tipo de retorno de ```double_number()```. Mas para quê? Bem, isso requer olhar para a assinatura do método ```parse()``` na biblioteca padrão: 

```
impl str {
    fn parse<F: FromStr>(&self) -> Result<F, F::Err>;
}
```

Vemos que precisamos usar um ```Result```. Certamente, é possível que isso pudesse ter retornado uma ```Option```. Afinal, uma string ou é analisada como um número ou não, certo? Essa é certamente uma maneira razoável de fazer isso, mas a implementação distingue internamente por que a string não foi analisada como um inteiro. (Seja uma string vazia, um dígito inválido, muito grande ou muito pequeno.) Portanto, usar um ```Result``` faz sentido porque queremos fornecer mais informações do que simplesmente "ausência". Queremos dizer por que a análise falhou. Você deve tentar emular essa linha de raciocínio quando confrontado com uma escolha entre ```Option``` e ```Result```. Se você puder fornecer informações detalhadas sobre o erro, provavelmente deveria. 

OK, mas como escrevemos nosso tipo de retorno? O método ```parse()``` conforme definido acima é genérico sobre todos os tipos de números definidos na biblioteca padrão. Poderíamos (e provavelmente deveríamos) também tornar nossa função genérica, mas vamos favorecer a explicitude por enquanto. Só nos importamos com [i32](https://doc.rust-lang.org/std/primitive.i32.html), então precisamos encontrar sua implementação de [FromStr](https://doc.rust-lang.org/std/primitive.i32.html#impl-FromStr-for-i32) e olhar para seu tipo associado ```Err```. Fizemos isso para que pudéssemos encontrar o tipo de erro concreto. Neste caso, é [std::num::ParseIntError](https://doc.rust-lang.org/std/num/struct.ParseIntError.html). Finalmente, podemos reescrever nossa função:

```
use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}

```

Isso está um pouco melhor, mas agora escrevemos muito mais código! A análise de casos nos incomodou novamente. 

Combinadores são a solução novamente! Assim como ```Option```, ```Result``` tem muitos combinadores definidos como métodos. Existe uma grande interseção de combinadores comuns entre ```Result``` e ```Option```. Em particular, ```map``` faz parte dessa interseção:

```
use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}

```

 
Os métodos normalmente usados com ```Option``` estão todos lá para ```Result```, incluindo [unwrap_or](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) e [and_then](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then). Além disso, como ```Result``` tem um segundo parâmetro de tipo, existem combinadores que afetam apenas o tipo de erro, como [map_err](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) (em vez de [map](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)) e [or_else](https://doc.rust-lang.org/std/result/enum.Result.html#method.or_else) (em vez de [and_then](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then)). 

## Definindo alias para Result

Na biblioteca padrão, você pode ver frequentemente tipos como ```Result<i32>```. Mas espere, definimos ```Result``` para ter dois parâmetros de tipo. Como podemos especificar apenas um? A chave é definir um alias de tipo ```Result``` que fixa um dos parâmetros de tipo com um tipo específico. Normalmente, o tipo fixo é o tipo de erro. Por exemplo, nosso exemplo anterior de análise de inteiros poderia ser reescrito assim:

```
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    unimplemented!();
}
```
 
Por que faríamos isso? Bem, se tivermos muitas funções que podem retornar ```ParseIntError```, é muito mais conveniente definir um alias que sempre use ```ParseIntError``` para que não precisemos escrevê-lo o tempo todo.

O lugar mais proeminente onde esse idioma é usado na biblioteca padrão é com [io::Result](https://doc.rust-lang.org/std/io/type.Result.html). Normalmente, escreve-se ```io::Result<T>```, o que deixa claro que você está usando o alias de tipo do módulo ```io``` em vez da definição de ```std::result``` (Esse idioma também é usado para [fmt::Result](https://doc.rust-lang.org/std/fmt/type.Result.html)). 
 
## Quando usar unwrap

Você pode ter notado que eu tenho sido bastante rigoroso em relação a chamar métodos como ```unwrap()``` que podem causar pânico e abortar seu programa. De modo geral, este é um bom conselho. 

No entanto, ```unwrap``` ainda pode ser usado com cautela. O que exatamente justifica o uso de ```unwrap``` é um tanto nebuloso e as opiniões podem variar. Vou resumir algumas das minhas opiniões sobre o assunto. 

* **Em exemplos e códigos rápidos**. Às vezes você está escrevendo exemplos ou um programa rápido, e o tratamento de erros simplesmente não é importante. Superar a conveniência de ```unwrap``` pode ser difícil em tais cenários, por isso é muito atraente. 
* **Quando o pânico indica um bug no programa**. Quando as invariantes do seu código devem impedir que um determinado caso aconteça (como, digamos, remover de uma pilha vazia), então o pânico pode ser permitido. Isso ocorre porque ele expõe um bug no seu programa. Isso pode ser explícito, como um ```assert!``` falhando, ou pode ser porque seu índice em um array estava fora dos limites. 

Esta provavelmente não é uma lista exaustiva. Além disso, ao usar uma ```Option```, geralmente é melhor usar seu método ```expect()```. ```expect()``` faz exatamente a mesma coisa que ```unwrap()```, exceto que imprime uma mensagem que você fornece. Isso torna o pânico resultante um pouco mais agradável de lidar, pois mostrará sua mensagem em vez de “called unwrap on a None value.”.

Meu conselho se resume a isso: use o bom senso. Existe uma razão pela qual as palavras "nunca faça X" ou "Y é considerado prejudicial" não aparecem em meus escritos. Há compensações em todas as coisas, e cabe a você, como programador, determinar o que é aceitável para seus casos de uso. Meu objetivo é apenas ajudá-lo a avaliar as compensações com a maior precisão possível. 

Agora que cobrimos os fundamentos do tratamento de erros em Rust e explicamos o uso de ```unwrap()``` e suas variantes, vamos começar a explorar mais da biblioteca padrão. 

## Trabalhando com múltiplos tipos de erro

Até agora, analisamos o tratamento de erros onde tudo era ```Option<T>``` ou ```Result<T, SomeError>```. Mas o que acontece quando você tem um tipo ```Option``` e um tipo ```Result```? Ou e se você tiver um ```Result<T, Error1>``` e um ```Result<T, Error2>```? Lidar com a composição de tipos de erros distintos é o próximo desafio que enfrentamos, e será o tema principal ao longo do restante desta seção. 

## Misturando Option e Result

Até agora, falei sobre combinadores definidos para ```Option``` e combinadores definidos para ```Result```. Podemos usar esses combinadores para compor resultados de diferentes computações sem fazer análise de caso explícita. 

Claro, no código real, as coisas nem sempre são tão limpas. Às vezes você tem uma mistura de tipos ```Option``` e ```Result```. Devemos recorrer à análise de casos explícita, ou podemos continuar usando combinadores? 

Por enquanto, vamos revisitar um dos primeiros exemplos desta seção: 

```
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // erro 1
    let n: i32 = arg.parse().unwrap(); // erro 2
    println!("{}", 2 * n);
}
```

Dada a nossa nova compreensão de ```Option```, ```Result``` e seus diversos combinadores, devemos tentar reescrever isso para que os erros sejam tratados adequadamente e o programa não entre em pânico se houver um erro.

O aspecto complicado aqui é que ```argv.nth(1)``` produz um ```Option``` enquanto ```arg.parse()``` produz um ```Result```. Esses não são diretamente componíveis. Quando confrontados com um ```Option``` e um ```Result```, a solução geralmente é converter o ```Option``` para um ```Result```. No nosso caso, a ausência de um parâmetro de linha de comando (de ```env::args()```) significa que o usuário não invocou o programa corretamente. Poderíamos usar uma String para descrever o erro. Vamos tentar:

```
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Por favor passe pelo menos um argumento".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

Há algumas coisas novas neste exemplo. A primeira é o uso do combinador ```Option::ok_or()```. Esta é uma forma de converter um ```Option``` em um ```Result```. A conversão exige que você especifique qual erro usar se ```Option``` for ```None```. Como os outros combinadores que vimos, sua definição é muito simples: 

```
fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}
```

[to_owned()](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned) cria dados proprietários a partir de dados emprestados, normalmente por clonagem.

[and_then()](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then) chama a closure passada como argumento se o ```Result``` for ```Ok```, caso contrário retorna o valor ```Err```. 

O outro novo combinador usado aqui é [Result::map_err()](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err). Este é como [Result::map()](https://doc.rust-lang.org/std/result/enum.Result.html#method.map), exceto que ele mapeia uma função para a parte de erro de um valor ```Result```. Se o ```Result``` é um valor ```Ok(...)```, então ele é retornado sem modificações. 

Usamos ```map_err()``` aqui porque é necessário que os tipos de erro permaneçam os mesmos (devido ao nosso uso de ```and_then()```. Como escolhemos converter o ```Option<String>``` (de ```argv.nth(1)```) para um ```Result<String, String>```, também devemos converter o ```ParseIntError``` de ```arg.parse()``` para ```String```. 


## Os limites dos combinadores

Fazer I/O e analisar entrada é uma tarefa muito comum, e é uma que eu pessoalmente fiz muito em Rust. Portanto, usaremos (e continuaremos a usar) I/O e várias rotinas de análise para exemplificar o tratamento de erros. 

Vamos começar com algo simples. Nossa tarefa é abrir um arquivo, ler todo o seu conteúdo e converter seu conteúdo para um número. Então, multiplicar por 2 e imprimir a saída. 

Embora eu tenha tentado convencê-lo a não usar ```unwrap()```, pode ser útil escrever seu código usando ```unwrap()``` primeiro. Isso permite que você se concentre no seu problema em vez do tratamento de erros, e expõe os pontos onde o tratamento de erros adequado precisa ocorrer. Vamos começar por aí para que possamos ter uma ideia do código e, em seguida, refatorá-lo para usar um tratamento de erros melhor. 

```
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // error 2
    let n: i32 = contents.trim().parse().unwrap(); // error 3
    2 * n
}

fn main() {
    let doubled = file_double("foobar");
    println!("{}", doubled);
}
```

OBSERVAÇÃO: ```AsRef<Path>``` é usado porque é o mesmo trait bound usado em [std::fs::File::open()](https://doc.rust-lang.org/std/fs/struct.File.html#method.open). Isso torna ergonômico o uso de qualquer tipo de string como um caminho de arquivo.

Existem três erros diferentes que podem ocorrer aqui: 

* Um problema ao abrir o arquivo.
* Um problema ao ler dados do arquivo.
* Um problema ao analisar os dados como um número. 

Os dois primeiros problemas são descritos pelo tipo [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html). Sabemos disso por causa dos tipos de retorno de [std::fs::File::open()](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) e [std::io::Read::read_to_string()](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string). (Observe que ambos usam o idioma de alias do tipo ```Result``` descrito anteriormente. Se você clicar no tipo ```Result```, verá o [alias do tipo](https://doc.rust-lang.org/std/io/type.Result.html) e, consequentemente, o tipo subjacente [io::Error](https://doc.rust-lang.org/std/io/struct.Error.html).) O terceiro problema é descrito pelo tipo [std::num::ParseIntError](https://doc.rust-lang.org/std/num/struct.ParseIntError.html). O tipo ```io::Error``` em particular é difundido em toda a biblioteca padrão. Você o verá repetidamente.

Vamos começar o processo de refatoração da função ```file_double()```. Para tornar essa função componível com outros componentes do programa, ela não deve entrar em pânico se alguma das condições de erro acima for atendida. Efetivamente, isso significa que a função deve retornar um erro se alguma de suas operações falhar. Nosso problema é que seu tipo de retorno é ```i32```, o que não nos dá nenhuma maneira útil de relatar um erro. Portanto, devemos começar mudando o tipo de retorno de ```i32``` para algo diferente.

A primeira coisa que precisamos decidir: devemos usar ```Option``` ou ```Result```? Certamente poderíamos usar ```Option``` muito facilmente. Se qualquer um dos três erros ocorrer, poderíamos simplesmente retornar ```None```. Isso funcionará e é melhor do que entrar em pânico, mas podemos fazer muito melhor. Em vez disso, devemos passar alguns detalhes sobre o erro que ocorreu. Como queremos expressar a possibilidade de erro, devemos usar ```Result<i32, E>```. Mas o que ```E``` deve ser? Como dois tipos diferentes de erros podem ocorrer, precisamos convertê-los para um tipo comum. Um desses tipos é ```String```. Vamos ver como isso impacta nosso código: 

```
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

Este código parece um pouco complicado. Pode levar algum tempo antes que código como este se torne fácil de escrever. A maneira como o escrevemos é seguindo os tipos. Assim que mudamos o tipo de retorno de ```file_double()``` para ```Result<i32, String>```, tivemos que começar a procurar os combinadores certos. Neste caso, usamos apenas três combinadores diferentes: ```and_then```, ```map``` e ```map_err```.

```and_then``` é usado para encadear múltiplas computações onde cada computação pode retornar um erro. Após abrir o arquivo, há duas outras computações que podem falhar: ler do arquivo e analisar o conteúdo como um número. Correspondentemente, há duas chamadas para ```and_then```. 
 
```map``` é usado para aplicar uma função ao valor ```Ok(...)``` de um ```Result```. Por exemplo, a última chamada a ```map``` multiplica o valor de ```Ok(...)``` (que é um i32) por 2. Se um erro tivesse ocorrido antes desse ponto, essa operação teria sido ignorada devido à forma como o ```map``` é definido.

 
```map_err``` é o truque que faz tudo isso funcionar. ```map_err``` é como ```map```, exceto que ele aplica uma função ao valor ```Err(...)``` de um ```Result```. Neste caso, queremos converter todos os nossos erros para um único tipo: ```String```. Como ambos ```io::Error``` e ```num::ParseIntError``` implementam ```ToString```, podemos chamar o método ```to_string()``` para convertê-los.
 
Dito isso, o código ainda é complicado. Dominar o uso de combinadores é importante, mas eles têm seus limites. Vamos tentar uma abordagem diferente: retornos antecipados. 

## Retorno antecipado

Gostaria de pegar o código da seção anterior e reescrevê-lo usando retornos antecipados. Retornos antecipados permitem que você saia da função mais cedo. Não podemos retornar cedo em ```file_double()``` de dentro de outra closure, então precisaremos voltar à análise de caso explícita. 

```
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}

```

Alguns podem discordar sobre se este código é melhor do que o código que usa combinadores, mas se você não está familiarizado com a abordagem do combinador, este código parece mais simples de ler para mim. Ele usa análise de caso explícita com ```match``` e ```if let```. Se um erro ocorrer, ele simplesmente para de executar a função e retorna o erro (convertendo-o para string). 

Mas isso não é um passo para trás? Anteriormente, dissemos que a chave para o tratamento de erros ergonômico é reduzir a análise de casos explícitos, mas voltamos à análise de casos explícitos aqui. Acontece que existem várias maneiras de reduzir a análise de casos explícitos. Combinadores não são a única maneira.

## O operador ?

O operador ```?``` colocado após um valor ```Result``` é definido para funcionar quase da mesma forma que as expressões match. Se o valor do ```Result``` for um ```Ok```, o valor dentro do ```Ok``` será retornado
dessa expressão e o programa continuará. Se o valor for um ```Err```, o ```Err``` será retornado de toda a função como se tivéssemos usado a palavra-chave ```return```, de modo que o valor de erro seja propagado para o código que chamou.
 
```
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    let n = contents.trim().parse::<i32>().map_err(|e| e.to_string())?;
    Ok(2 * n)
}

fn main() {
    match file_double("foobar.txt") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```


## Definindo seu próprio tipo de erro

asd



## Referências
[https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#error-handling](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html#error-handling)

[char_indices()](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices)

[extension()](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension)

[Option](https://doc.rust-lang.org/std/option/enum.Option.html)

[Option::map()](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)

[Option::unwrap_or()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)

[Option::unwrap_or_ele()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else)

[Result](https://doc.rust-lang.org/std/result/enum.Result.html)

[Result::unwrap()](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap)

[i32](https://doc.rust-lang.org/std/primitive.i32.html)

[i32::FromStr](https://doc.rust-lang.org/std/primitive.i32.html#impl-FromStr-for-i32)

[std::num::ParseIntError](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)

[nth()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)

[to_owned()](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned)

[Result::map_err()](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)

[Result::map()](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)

[Result::and_then()](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then) 

[File::open()](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)

[std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html)

[std::io::Read::read_to_string()](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)

[std::io::Result](https://doc.rust-lang.org/std/io/type.Result.html)

[o operador ?](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)

---

arataca89@gmail.com

Última atualização: 20241017
