# RBE - manipulação de erros

O tratamento de erros é o processo de lidar com a possibilidade de falha. Por exemplo, falhar em ler um arquivo e continuar usando essa entrada ruim seria claramente problemático. Perceber e gerenciar explicitamente esses erros salva o resto do programa de várias armadilhas. 

Existem várias maneiras de lidar com erros em Rust, que são descritas a seguir. Todos eles têm diferenças mais ou menos sutis e casos de uso diferentes. Como regra geral: 

Um pânico explícito é principalmente útil para testes e lidar com erros irrecuperáveis. Para prototipagem, pode ser útil, por exemplo, ao lidar com funções que ainda não foram implementadas, mas nesses casos, o ```unimplemented``` mais descritivo é melhor. Em testes, o pânico é uma maneira razoável de falhar explicitamente. 

O tipo ```Option``` é para quando um valor é opcional ou quando a falta de um valor não é uma condição de erro. Por exemplo, o pai de um diretório - / e C: não têm um. Ao lidar com Options, ```unwrap``` é bom para prototipagem e casos em que é absolutamente certo que haverá um valor. No entanto, ```expect``` é mais útil, pois permite especificar uma mensagem de erro caso algo dê errado de qualquer maneira.

Quando há uma chance de que as coisas dêem errado e o chamador tenha que lidar com o problema, use ```Result```. Você pode usar ```unwrap``` e ```expect``` também (por favor, não faça isso a menos que seja um teste ou protótipo rápido).

Para uma discussão mais rigorosa sobre o tratamento de erros, consulte a seção de tratamento de erros no [livro oficial](https://doc.rust-lang.org/book/ch09-00-error-handling.html). 
 

* [panic](#panic)


---

## panic

asd

---

## asd

---

## Referências

[RBE - Error handling](https://doc.rust-lang.org/rust-by-example/error.html)


---

arataca89@gmail.com

Última atualização: 20241024