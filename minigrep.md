# minigrep - escrevendo um programa de linha de comando
Projeto descrito no [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

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
```fs::read_to_string()``` recebe um caminho de arquivo (path) e retorna ```Result<String>```. ```Result<T, E>``` é o tipo usado para retornar e propagar erros. É um enum com as variantes ````Ok(T)```, representando sucesso e contendo um valor, e ```Err(E)```, representando erro e contendo um valor de erro. [Result](https://doc.rust-lang.org/std/result/).
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
