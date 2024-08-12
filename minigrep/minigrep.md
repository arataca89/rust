# minigrep - escrevendo um programa de linha de comando

Baseado no projeto descrito no [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)


Este projeto escreve uma versão simples da clássica ferramenta ```grep``` presente em sistemas "Unix like". O comando ```grep```, basicamente, procura por um padrão de string em um arquivo. Para isso ele recebe como argumentos um caminho de arquivo (path) e uma string. O comando então lê o arquivo, procura pelas linhas que têm a string procurada e imprime estas linhas na tela.

[1. Recebendo argumentos da linha de comando](#1-Recebendo-argumentos-da-linha-de-comando)

[2. Lendo o arquivo](#2-Lendo-o-arquivo)

[3. Organização do código](#3-Organização-do-código)

[4. Retirando a análise dos argumentos da linha de comando para fora de ```main()```](#4-Retirando-a-análise-dos-argumentos-da-linha-de-comando-para-fora-de-main)

[5. Juntando os valores referentes a string a ser procurada e o nome do arquivo em uma única estrutura](#5-Juntando-os-valores-referentes-a-string-a-ser-procurada-e-o-nome-do-arquivo-em-uma-única-estrutura)

[6. Criando um construtor para o tipo ```Config```](#6-Criando-um-construtor-para-o-tipo-Config)

[7. Tratamento de erro](#7-Tratamento-de-erro)

[8. Retirando a lógica principal de ```main()```](#8-Retirando-a-lógica-principal-de-main)

[9. Refatorando ```run()``` para que execute o tratamento de erro](#9-Refatorando-run-para-que-execute-o-tratamento-de-erro)

[10. Separando a lógica para um crate de biblioteca](#10-Separando-a-lógica-para-um-crate-de-biblioteca)

[11. Desenvolvendo a funcionalidade da biblioteca usando TDD](#11-Desenvolvendo-a-funcionalidade-da-biblioteca-usando-TDD)

[12. Escrevendo o teste que falha](#12-Escrevendo-o-teste-que-falha)

[13. Escrevendo código para o teste passar](#13-Escrevendo-código-para-o-teste-passar)

[14. Inserindo a chamada a ```search()``` em ```run()```](#14-Inserindo-a-chamada-a-search-em-run)

[15. Adicionando o recurso ```case insensitive```](#15-Adicionando-o-recurso-case-insensitive)

[16. Inserindo a nova funcionalidade em ```run()```](#16-Inserindo-a-nova-funcionalidade-em-run)

[17. Enviando as mensagens de erro para ```strerr```](#17-Enviando-as-mensagens-de-erro-para-strerr)

---

## 1. Recebendo argumentos da linha de comando

Inicialmente nosso programa tem que aceitar argumentos da linha de comando. No site [https://crates.io](https://crates.io) existem bibliotecas que permitem manipular argumentos de linha de comando, mas neste projeto esta tarefa será implementada pois é um projeto de estudo.

Abaixo temos um código que recebe argumentos da linha de comando.

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len(){
        println!{"arg[{}]: {}",i,args[i]};
    }
}
```
A função ```std::env::args``` é fornecida pela biblioteca padrão do Rust. Ela retorna um iterator para os argumentos passados na linha de comando. Um iterator produz uma série de valores, correspondendo neste caso a cada item passado na linha de comando; e ele pode ser tranformado numa coleção através do uso do método ```collect```. Note que isto é feito aqui. Assim, os argumentos da linha de comando são transformados em um vetor de strings.
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run -- string arquivo
   Compiling minigrep v0.1.0 (C:\Users\arataca89\Documents\rust\packages\minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
     Running `target\debug\minigrep.exe string arquivo`
arg[0]: target\debug\minigrep.exe
arg[1]: string
arg[2]: arquivo

C:\Users\arataca89\Documents\rust\packages\minigrep>
```

## 2. Lendo o arquivo
```
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // for i in 0..args.len(){
    //     println!{"arg[{}]: {}",i,args[i]};
    // }
    let arquivo  = fs::read_to_string(&args[2])
    .expect("Erro ao tentar ler o arquivo.");

    println!("{}",arquivo);
}
```
```fs::read_to_string()``` recebe um caminho de arquivo (path) e retorna ```Result<String>```. ```Result<T, E>``` é o tipo usado para retornar e propagar erros. É um ```enum``` com as variantes ```Ok(T)```, representando sucesso e contendo um valor, e ```Err(E)```, representando erro e contendo um valor de erro. [Result](https://doc.rust-lang.org/std/result/).
```
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```
Como ```Result``` pode retornar um erro usamos o método ```expect()``` que, em caso de erro, executa a macro ```panic!```, encerrando o programa, e exibe a string passada como argumento.

Abaixo temos a execução deste código sendo passado um arquivo inexistente.
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run -- string arquivo
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\minigrep.exe string arquivo`
thread 'main' panicked at src/main.rs:10:6:
Erro ao tentar ler o arquivo.: Os { code: 2, kind: NotFound, message: "O sistema não pode encontrar o arquivo especificado." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\minigrep.exe string arquivo` (exit code: 101)

C:\Users\arataca89\Documents\rust\packages\minigrep>
```
E agora temos a execução com a passagem de um aquivo texto inserido no diretório do pacote.
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run -- string rust.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\minigrep.exe string rust.txt`
Por que Rust?

Desempenho

Rust é extremamente rápido e gerencia memória eficientemente:
sem runtime ou garbage collector, podendo potencializar a
performance de serviços críticos, rodar em sistemas embarcados,
e facilmente integrar-se a outras linguagens.

Confiabilidade

O rico sistema de tipos de Rust e seu modelo de ownership garantem
segurança de memória e segurança de concorrência — e permite que
você elimine muitas categorias de erros durante a compilação.

Produtividade

Rust possui uma ótima documentação, um compilador amigável com
mensagens de erros úteis, e ferramental de primeira qualidade — uma
ferramenta integrada de compilação e gerenciamento de pacotes,
suporte inteligente para múltiplos editores com autocompleção e
inspeções de tipos, um formatador automático, e muito mais.


C:\Users\arataca89\Documents\rust\packages\minigrep>
```

## 3. Organização do código

A comunidade Rust desenvolveu diretrizes (guidelines) para a organização dos projetos de modo a facilitar o entendimento e manutenção do código. Não convém que todo o código fique na função ```main()```. Para pequenos projetos de estudo e testes rápidos isso é aceitável, mas quando o projeto começa a crescer o entendimento e manutenção já ficarão prejudicados. As diretrizes da comunidade Rust para a organização do código são:

* Divida o projeto nos arquivos ```main.rs``` e ```lib.rs```. ```lib.rs``` deve conter a lógica do programa;
* Enquanto a lógica de análise da linha de comando for pequena ela pode permanecer em ```main.rs```;
* Quando a lógica de análise da linha de comando começar a ficar complicada mova-a para ```lib.rs```;

Após este processo as responsabilidades da função ```main()``` devem ser:

* Chamar a lógica de análise da linha de comando com os valores dos argumentos passados na linha de comando;
* Ajustar qualquer outro tipo de configuração;
* Chamar a função ```run()``` que deve estar em ```lib.rs``` e vai executar a lógica principal do programa;
* Manipular erros retornados por ```run()```.

Este padrão separa as tarefas: ```main.rs``` se encarrega de rodar o programa; e ```lib.rs``` é responsável por toda a lógica.

Note que esta organização não permite testar ```main()``` diretamente pois ela apenas chama a lógica que está em funções em ```lib.rs```. Assim, os testes devem ser executados por funções que estarão em ```lib.rs```. ```main()``` terá um código pequeno e simples o suficiente para que a análise visual do código identifique erros.

## 4. Retirando a análise dos argumentos da linha de comando para fora de ```main()```
```
use std::env;
use std::fs;

fn parse_args(args: &[String]) -> (&str,&str) {
    let string = &args[1];
    let filepath = &args[2];
    (string, filepath)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // for i in 0..args.len(){
    //     println!{"arg[{}]: {}",i,args[i]};
    // }

    let(string, filepath) = parse_args(&args);

    let file  = fs::read_to_string(filepath)
    .expect("Erro ao tentar ler o arquivo.");

    println!("string  : {}", string);
    println!("filepath:\n{}",file);
}
```

## 5. Juntando os valores referentes a string a ser procurada e o nome do arquivo em uma única estrutura

Note que atualmente o código da função que avalia os argumentos da linha de comando, ```parse_args()```, retorna uma tupla e imediatamente a desmembra em duas variáveis. 
```
 let(string, filepath) = parse_args(&args);
```
Isto indica que provavelmente seria interessante melhorar nossa abstração e criar uma estrutura para encapsular estes dois valores. 

Nosso programa precisa de uma string a ser procurada e de um arquivo onde procurar por tal string. Note que estes valores estão relacionados. Atualmente a tupla que recebe os valores da função ```parse_args()``` funciona mas não transmite bem esse significado. Não indica que os valores estão relacionados. Colocar estes valores em uma ```struct``` e atribuir a eles nomes significativos facilitará o entendimento, indicando que os valores estão relacionados e qual seu propósito.
Isso facilitará futuras alterações ou manutenção no código.
```
use std::env;
use std::fs;

struct Config{
    string: String,
    filepath: String,
}

fn parse_args(args: &[String]) -> Config {
    let string = args[1].clone();
    let filepath = args[2].clone();
    Config{string, filepath}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);

    let file  = fs::read_to_string(config.filepath)
    .expect("Erro ao tentar ler o arquivo.");

    println!("string  : {}", config.string);
    println!("filepath:\n{}",file);
}
```
Observe que a variável ```args``` na função ```main()``` é proprietária do vetor de argumentos da linha de comando e apenas empresta este vetor para a função ```parse_args()```.
```
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);
```
Isto significa que se a função ```parse_args()``` tentar assumir a propriedade dos argumentos para construir o objeto ```Config``` a ser retornado, isso irá violar as regras de empréstimo de Rust (borrowing). Uma maneira simples, porém ineficiente, de resolver este problema é usar o método ```clone()``` para criar uma cópia dos dados que poderão ser apropriados pelo objeto ```Config```.
```
fn parse_args(args: &[String]) -> Config {
    let string = args[1].clone();
    let filepath = args[2].clone();
    Config{string, filepath}
}
```
A solução melhor, porém mais complicada, seria configurar o tempo de vida (lifetime) das referências.

## 6. Criando um construtor para o tipo ```Config```
Note que a função ```parse_args()``` recebe o vetor com os argumentos da linha de comando, cria um objeto ```Config```, o configura e o retorna. Podemos melhorar nosso código transformando essa função em um método ```new()``` , associado a  ```struct Config``` , que cria um novo objeto ```Config```. Esta alteração tornará nosso código mais idiomático, mais no jeito Rust de ser, já que normalmente os tipos da biblioteca são criados assim.
```
use std::env;
use std::fs;

struct Config{
    string: String,
    filepath: String,
}

impl Config{
    fn new(args: &[String]) -> Config {
        let string = args[1].clone();
        let filepath = args[2].clone();
        Config{string, filepath}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::new(&args);

    let file  = fs::read_to_string(config.filepath)
    .expect("Erro ao tentar ler o arquivo.");

    println!("string  : {}", config.string);
    println!("filepath:\n{}",file);
}

```

## 7. Tratamento de erro
Atualmente nosso programa não está tratando erro. Se tentarmos executar sem passar os argumentos esperados receberemos uma saida semelhante a:
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\minigrep.exe`
thread 'main' panicked at src/main.rs:11:18:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\minigrep.exe` (exit code: 101)

C:\Users\arataca89\Documents\rust\packages\minigrep>
```
Inicialmente podemos simplesmente verificar se os argumentos foram passados e, caso não tenham sido, executar a macro ```panic!```.
```
impl Config{
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Erro. Poucos argumentos.\nUso: minigrep string arquivo");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        Config{string, filepath}
    }
}
```
Tal modificação já melhora, mas é indicada enquanto estamos no desenvolvimento. Ela nos dará uma saída semelhante a:
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run
   Compiling minigrep v0.1.0 (C:\Users\arataca89\Documents\rust\packages\minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target\debug\minigrep.exe`
thread 'main' panicked at src/main.rs:12:13:
Erro. Poucos argumentos.
Uso: minigrep string arquivo
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\minigrep.exe` (exit code: 101)

C:\Users\arataca89\Documents\rust\packages\minigrep>
```
Apesar da mensagem mais amigável, ainda não é o ideal. Além do mais ```panic!``` é mais apropriada para um problema do programa que para um erro do usuário, que é o que temos aqui. O usuário deve fornecer a string e o arquivo para que nosso programa faça o trabalho dele. Código idiomático Rust deve retornar ```Result``` de modo que quem chama possa tratar o possível erro.

Assim, nosso método que constrói um objeto ```Config``` deve retornar um ```Result<T,E>```, ou seja, deve retornar um objeto ```Config``` em caso de sucesso ou um erro em caso de falha. Para isso iremos alterar o nome do métoto ```new()``` para ```build()``` porque código idiomático Rust considera que ```new()``` nunca falha.
```
use std::env;
use std::fs;
use std::process;

struct Config{
    string: String,
    filepath: String,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    let file  = fs::read_to_string(config.filepath)
    .expect("Erro ao tentar ler o arquivo.");

    println!("string  : {}", config.string);
    println!("filepath:\n{}",file);
}
```
Agora, se o usuário não entrar com os argumentos corretamente, a saída será:
```
C:\Users\arataca89\Documents\rust\packages\minigrep>cargo run
   Compiling minigrep v0.1.0 (C:\Users\arataca89\Documents\rust\packages\minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target\debug\minigrep.exe`
Erro:Poucos argumentos.
Uso: minigrep string arquivo
error: process didn't exit successfully: `target\debug\minigrep.exe` (exit code: 1)

C:\Users\arataca89\Documents\rust\packages\minigrep>
```
Note que não aparece a indicação de pânico nem a nota referindo-se a RUST_BACKTRACE. O encerramento ficou bem mais elegante. 

A função que cria um objeto ```Config``` agora é assim:
```
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
```
Ela retorna um ```Result``` com uma instância ```Config``` em caso de sucesso, e uma ```&'static str``` em caso de erro. Os valores de erro retornados em situações semelhantes a essa sempre serão literais de string que possuem lifetime ```'static```.

Retornar um valor ```Err``` de ```Config::build```, em caso de erro, permite que ```main()``` possa processar o erro de maneira adequada.
```
    let config =  Config::build(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("Uso: minigrep string arquivo");
        process::exit(1);
    });
```
O método ```unwrap_or_else()``` é útil em situações semelhantes a essa. Em caso de sucesso ele retorna o valor encapsulado na variante ```Ok``` de ```Result```. Em caso de erro ele executa a closure passada como argumento. Neste caso, nossa closure recebe a mensagem de erro retornada por ```build()```, exibe esta mensagem na tela e encerra o programa com ```process::exit()```. 

## 8. Retirando a lógica principal de ```main()```
A fim de seguir as melhores práticas, citadas mais acima, e tornar o código mais fácil de ler, entender e manutenir vamos criar uma função chamada ```run()``` a qual conterá toda a lógica principal que atualmente ainda está em ```main()```. Após esta alteração, ```main()``` estará livre de manipular os argumentos passados na linha de comando e do respectivo tratamento de erro, ficando concisa e fácil de ser verificada por uma simples inspeção visual. Esta alteração também facilitará a escrita de testes para o código que executa a lógica principal.
```
use std::env;
use std::fs;
use std::process;

struct Config{
    string: String,
    filepath: String,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
}

fn run(config: Config){
    let file  = fs::read_to_string(config.filepath)
    .expect("Erro ao tentar ler o arquivo.");

    println!("string  : {}", config.string);
    println!("file    :\n{}",file);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    run(config);
}
```
## 9. Refatorando ```run()``` para que execute o tratamento de erro
Similar ao que foi feito com ```Config::build()``` vamos refatorar ```run()``` para que, em vez de chamar ```panic!```, ela retorne um ```Result<T,E>```.
```
use std::env;
use std::fs;
use std::process;
use std::error::Error;


struct Config{
    string: String,
    filepath: String,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    println!("string  : {}", config.string);
    println!("file    :\n{}",file);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    if let Err(e) = run(config){
        println!("Erro:{e}");
        process::exit(1);
    }
}
```
A primeira observação acerca deste código recai no tipo de retorno da função ```run()```. Agora a função retorna ```Result<(),Box<dyn Error>>```. Observe que, em caso de sucesso, a função retorna um ```Ok()``` contendo o [tipo unidade](https://doc.rust-lang.org/std/primitive.unit.html), ```()```; segundo a referência Rust, O tipo ```()``` tem exatamente um valor, que é ```()```, e é usado quando não há outro valor significativo a ser retornado. Este valor é normalmente usado implicitamente em funções que não apresentam valor de retorno em suas implementações, ou seja, os códigos abaixo são equivalentes:
```
fn myfunction() -> () {}

fn myfunction() {}
```
Em caso de erro, ```run()``` agora retorna um objeto trait do tipo ```Box<dyn Error>```; para isso há necessidade de importar ```std::error::Error```. Este retorno indica que a função retornará um tipo que implementa a trait ```Error```, mas não especificamos qual tipo será. Isso nos dá flexibilidade para retornar valores de erro que podem ser de tipos diferentes em diferentes casos de erro. A palavra-chave ```dyn``` aqui significa dinâmico (dynamic).

Note também que ```expect``` foi substituída pelo operador ```?```. Em vez de chamar ```panic!``` em caso de erro, o operador ```?``` retornará o valor do erro da função atual para o chamador manipular.

A sintaxe retornando um ```Ok()``` com um tipo unidade, ```Ok(())``` pode parecer estranha, mas é a maneira idiomática Rust de indicar que a função executará seu trabalho sem retornar nada.

Note também que em ```main()```, ao chamar ```run()``` nós agora só precisamos tratar o erro, pois ```run()``` não retornará nada em caso de sucesso. Isto nos permite usar a construção ```if let``` em vez de termos que usar  ```unwrap_or_else```.

## 10. Separando a lógica para um crate de biblioteca

Para melhor organização do projeto vamos dividir o código entre os arquivos ```main.rs``` e ```lib.rs```, assim poderemos testar o código e deixar ```main.rs``` com poucas responsabilidades. Vamos mover todo o código que não está na função ```main()``` para o arquivo ```lib.rs```:
* A função ```run()```;
* As declarações ```use``` necessárias;
* A estrutura ```Config```;
* A implementação dos métodos para ```Config```, no caso apenas a função ```build()```.

Abaixo temos o código do arquivo ```lib.rs```
```
// lib.rs
use std::fs;
use std::error::Error;

pub struct Config{
    pub string: String,
    pub filepath: String,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    println!("string  : {}", config.string);
    println!("file    :\n{}",file);

    Ok(())
}
```
E aqui temos como ficou o arquivo ```main.rs```:
```
// main.rs
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("Erro:{e}");
        process::exit(1);
    }
}
```
Note a utilização da palavra-chave ```pub``` em ```lib.rs```. Como será a API do nosso programa a estrutura e funções deste crate devem ter acesso público. Em ```main.rs``` tivemos que importar as funcionalidades usando ```use```.

## 11. Desenvolvendo a funcionalidade da biblioteca usando TDD

TDD, "Test-Driven Development", "Desenvolvimento Dirigido por Testes" numa tradução livre, é muito usado em Rust. Neste ponto extraímos a lógica principal e colocamos em ```lib.rs```; e deixamos em ```main.rs``` apenas a coleta dos argumentos de linha de comando e o tratamento de erros. Agora é muito mais fácil escrever testes para a funcionalidade principal do nosso código. Podemos chamar funções diretamente com vários argumentos e verificar valores de retorno sem precisar chamar o binário na linha de comando, apenas usaremos as ferramentas de testes do Rust.

Adicionaremos a lógica principal do nosso ```minigrep``` para procurar por uma string e exibir cada linha do arquivo que tenha esta string. Para isso usaremos o TDD que, basicamente, consiste em:

1. Escrever um teste que deve falhar;
2. Alterar o código para que o teste passe, tenha sucesso;
3. Refatorar o código quando necessário, certificando-se que continua a passar no teste;
4. Voltar ao passo 1.

Este típo de desenvolvimento ajuda muito pois quando escrevemos o teste temos que pensar em como nossa função deve comportar-se, o que deve receber, o que deve retornar, etc... ou seja, ajuda inclusive no design da função propriamente dita.

## 12. Escrevendo o teste que falha
```
// lib.rs
use std::fs;
use std::error::Error;

pub struct Config{
    pub string: String,
    pub filepath: String,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        
        Ok(Config{string, filepath})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    //println!("string  : {}", config.string);
    //println!("file    :\n{}",file);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let query = "string";
        let contents = "\
linha 1 
linha 2 string
linha 3 ";
        assert_eq!(vec!["linha 2 string"], search(query,contents));
    }
}
```
Foi adicionado o módulo ```tests``` e uma primeira função de teste. A função ```search()``` foi escrita para o teste falhar, ela retorna um vetor ```&str``` vazio enquanto a função de teste compara este vetor retornado com um vetor que possui a linha que seria retornada numa situação de funcionamento normal do programa.

A barra invertida logo após as aspas duplas, no início da string "contents" diz ao Rust para não colocar o caractere de nova linha no início do conteúdo desta string literal.

Note que foi necessário definir um lifetime ```'a``` na função ```search()```. Este lifetime conecta o parâmetro ```contents``` e o valor de retorno. Isto é necessário porque não há criação de novos valores aqui, os valores são emprestados. Aqui indicamos que o vetor retornado possui referências a slices de string que estão no parâmetro ```contents``` e para que essas referências existam ```contents``` deve existir. Se este ajuste de lifetime não for feito, Rust emitirá uma erro de compilação.

## 13. Escrevendo código para o teste passar

Para o teste passar devemos ajustar a função ```search()``` que é quem irá procurar pela string nas linhas. Esta codificação é facilitada pela biblioteca Rust que possui duas funções interessantes para este programa. Para percorrer as linhas do arquivo podemos usar o  método ```lines()``` que itera linha por linha numa string; e para verificar se a linha possui a string podemos usar o método ```contains()``` que, como o nome sugere, verifica se um slice de string contém uma "sub-slice" de string passada como argumento. Abaixo temos o código de ```search()```.
```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}
```
Agora se executarmos o comando ```cargo test```, o teste irá passar.

## 14. Inserindo a chamada a ```search()``` em ```run()```

Com ```search``` funcionando beleza basta inserir uma chamada em ```run()```.

```
pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    for line in search(&config.string, &file){
        println!("{line}");
    }

    Ok(())
}
```
## 15. Adicionando o recurso ```case insensitive```

Vamos adicionar o recurso para que a busca ignore a caixa das letras. Inicialmente, seguindo o desenvolvimento dirigido a testes, TDD, vamos criar o teste para a função que executará esta tarefa.

```
// lib.rs

.....
(código omitido)
.....

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    //let mut results = Vec::new();

    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }

    // results

    vec![]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "string";
        let contents = "\
linha 1 
linha 2 string
linha 3 ";
        assert_eq!(vec!["linha 2 string"], search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "String";
        let contents = "\
linha 1 
linha 2 String
linha 3 string";
        assert_eq!(vec!["linha 2 String","linha 3 string"], search_insensitive(query,contents));
    }

}
```

Executando ```cargo test``` nosso teste insensitive deve falhar. Agora vamos implementar o código para que o teste passe.

```
// lib.rs
.....
(código omitido)
.....

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results

}
.....
(código omitido)
.....
```

Note a utilização do método de biblioteca ```to_lowercase()``` para converter as slices de string para caixa baixa e poder fazer a comparação apropriadamente.

Uma variável local auxiliar foi criada na função ```search_insensitive()``` para armazenar o valor de ```query``` convertido para caixa baixa. Isto é possível pois ```to_lowercase()``` retorna uma nova ```String```. Isto permite a comparação sem alterar o valor original. O mesmo acontece com a variável ```line``` do loop ```for```. Veja que a comparação no loop ```for``` é feita com o objeto retornado por ```to_lowercase()``` e não com ```line```. Assim, ```line``` insere a slice de string original no vetor, se a comparação for verdadeira.

Como essa variável auxiliar local ```query``` vai receber um objeto ```String```, temos que inserir o ```&``` na chamada a ```contains()```, pois este método recebe um slice de string e não um objeto String.


## 16. Inserindo a nova funcionalidade em ```run()```

Para executar ativando ```ignore case``` , primeiro vamos inserir um campo booleano na estrutura ```Config``` que nos permita escolher qual das funções ```search``` usar:

```
.....
(código omitido)
.....
pub struct Config{
    pub string: String,
    pub filepath: String,
    pub ignore_case: bool,
}
.....
(código omitido)
.....
```

Depois devemos modificar ```run()``` para que verifique o valor de ```ignore_case``` e escolha a função correta:
```
.....
(código omitido)
.....
pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_insensitive(&config.string, &file)
    } else {
        search(&config.string, &file)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
.....
(código omitido)
.....
```

Finalmente devemos verificar o valor da variável de ambiente que será usada para ligar ou desligar ```ignore_case```. 

```
.....
(código omitido)
.....
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config{string, filepath, ignore_case})
    }
    
.....
(código omitido)
.....
```

Foi usada a função ```env::var()```. Esta função recebe o nome de uma variável de ambiente e retorna um ```Result```. Este ```Result``` retornará como ```Ok``` , com o valor da variável de ambiente, se ela exitir; ou retornará um ```Err``` caso ela não exista no ambiente. Foi usado também o método ```is_ok()``` no ```Result``` retornado para verificar a existência da variável de ambiente. Se a variável de ambiente não estiver sido configurada, ```is_ok()``` retornará ```false```.


Note que não nos interessa, neste caso, o valor da variável de ambiente, apenas se ela está definida ou não. Por isso que aqui foi usada a função ```is_ok()``` e não ```unwrap```, ```expect``` ou qualquer outro método que normalmente se usa com ```Result```.

Para executar o programa com a opção ```ignore case``` devemos primeiro ajustar a variável de ambiente. Em ambiente Linux o comando será:

```
$ IGNORE_CASE=1 cargo run -- String rust.txt
```

Em ambiente Windows devemos criar a variável de ambiente no ```cmd``` do Windows. Para isso use o comando:
```
SET IGNORE_CASE=1
```
Agora, ao executar ```cargo run``` nosso programa vai ativar a opção ```ignore case```.

```
cargo run -- String rust.txt
```

Para desativar a variável de ambiente use o comando:
```
SET IGNORE_CASE=
```

Após ativar essa variável de ambiente a saída do programa irá exibir as linhas sem considerar a caixa, como é esperado.


## 17. Enviando as mensagens de erro para ```strerr```

Normalmente, os programas de linha de comando enviam as mensagens de erro para a saída de erro padrão, de modo que ainda possamos ver mensagens de erro na tela mesmo se redirecionarmos a saída bem sucedida para um arquivo.

Para fazer isto basta alterar a macro ```println!``` para ```eprintln!``` em ```main()```.
```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        eprintln!("{err}");
        eprintln!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Erro:{e}");
        process::exit(1);
    }
}
```

---
arataca89@gmail.com

Última atualização: 20240812
