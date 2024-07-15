# minigrep - escrevendo um programa de linha de comando
Uma tradução livre do [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

Este projeto escreve uma versão simples da clássica ferramenta ```grep``` presente em sistemas "Unix like". O comando ```grep```, basicamente, procura por um padrão de string em um arquivo. Para isso ele recebe como argumentos um caminho de arquivo (path) e uma string. O comando então lê o arquivo, procura pelas linhas que têm a string procurada e imprime estas linhas na tela.
