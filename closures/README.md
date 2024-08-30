# Closures em Rust

Closures em Rust são funções anônimas que você pode salvar numa variável ou passar como argumento para outras funções. Você pode criar uma closure em um lugar e chamá-la em outro para avaliá-la em outro contexto. Diferente das funções tradicionais, closures podem capturar valores do escopo onde são definidas. Os recursos das closures permitem reutilização de código e customização de comportamento.

[1. Capturando o ambiente com closures](#1-Capturando-o-ambiente-com-closures)


---

## 1. Capturando o ambiente com closures


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


---
## Referências

[capítulo 13 do "Livro"](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

arataca89@gmail.com

Última atualização: 20240830
