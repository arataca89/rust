# Closures em Rust

Closures em Rust são funções anônimas que você pode salvar numa variável ou passar como argumento para outras funções. Você pode criar uma closure em um lugar e chamá-la em outro para avaliá-la em outro contexto. Diferente das funções tradicionais, closures podem capturar valores do escopo onde são definidas. Os recursos das closures permitem reutilização de código e customização de comportamento.

[1. Capturando o ambiente com closures](#1-Capturando-o-ambiente-com-closures)


---

## 1. Capturando o ambiente com closures

Closures podem capturar valores do ambiente onde foram definidas para uso posterior.

Para exemplificar o uso deste recurso foi criado um cenário onde uma certa empresa que comercializa camisetas faz a seguinte promoção: será sorteada uma camiseta exclusiva entre as pessoas que se inscreverem na lista para receber emails promocionais da empresa. No momento da inscrição a pessoa pode também declarar sua cor favorita. Se a pessoa sorteada tiver declarado sua cor favorita, ganhará a camiseta dessa cor. Senão ganhará uma camiseta da cor que a empresa mais tenha no momento.



---
## Referências

[capítulo 13 do "Livro"](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

arataca89@gmail.com

Última atualização: 20240830
