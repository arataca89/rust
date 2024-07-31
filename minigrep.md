# minigrep - escrevendo um programa de linha de comando
Baseado no projeto descrito no [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

Este projeto escreve uma versão simples da clássica ferramenta ```grep``` presente em sistemas "Unix like". O comando ```grep```, basicamente, procura por um padrão de string em um arquivo. Para isso ele recebe como argumentos um caminho de arquivo (path) e uma string. O comando então lê o arquivo, procura pelas linhas que têm a string procurada e imprime estas linhas na tela.

[1. Recebendo argumentos da linha de comando](#1.-Recebendo-argumentos-da-linha-de-comando)

[2. Lendo o arquivo](#2.-Lendo-o-arquivo)


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

## 6. Criando um construtor para o tipo  ```Config```
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


asdfg
