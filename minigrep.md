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
    dbg!(args);
}
```
A função ```std::env::args``` é fornecida pela biblioteca padrão do Rust. Ela retorna um iterator para os argumentos passados na linha de comando. Um iterator produz uma série de valores, correspondendo neste caso a cada item passado na linha de comando; e ele pode ser tranformado numa coleção através o uso do método ```collect```. Note que isto é feito aqui. Assim, os argumentos da linha de comando são transformados em um vetor de strings.

