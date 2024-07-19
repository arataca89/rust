# minigrep - escrevendo um programa de linha de comando
Baseado no projeto descrito no [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

Este projeto escreve uma versão simples da clássica ferramenta ```grep``` presente em sistemas "Unix like". O comando ```grep```, basicamente, procura por um padrão de string em um arquivo. Para isso ele recebe como argumentos um caminho de arquivo (path) e uma string. O comando então lê o arquivo, procura pelas linhas que têm a string procurada e imprime estas linhas na tela.

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

asdfg
