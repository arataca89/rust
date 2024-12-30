#### arataca89

# Linguagem Rust - panic!

Provoca pânico da thread atual, encerrando o programa imediatamente e fornecendo feedback ao chamador do programa.

```
macro_rules! panic {
    ($($arg:tt)*) => { ... };
}
```

Esta macro é a maneira perfeita de afirmar condições (assert) em código de exemplo e em testes.

 ```panic!``` está intimamente ligado ao método ```unwrap()``` das enumerações ```Option``` e ```Result```. Ambas as implementações chamam ```panic!``` quando são definidas como variantes ```None``` ou ```Err```.

Quando usa ```panic!()```, você pode especificar uma string a ser exibida no feedback.

O comportamento do código que roda diretamente após ```panic!``` ser invocado, é imprimir a mensagem de erro para ```stderr``` junto com as informações de arquivo/linha/coluna da chamada ```panic!()```.

Você pode substituir isso usando ```std::panic::set_hook()```. 

Um ```panic!``` pode ser acessado como um ```&dyn Any + Send```, que contém um ```&str``` ou ```String``` para invocações regulares ```panic!()```. 

Para entrar em pânico com um valor de outro tipo, ```panic_any()``` pode ser usado.

Veja também a macro [compile_error!](https://doc.rust-lang.org/std/macro.compile_error.html), para gerar erros durante a compilação. 

### panic! versus Result

A linguagem Rust fornece dois sistemas complementares para construir/representar, relatar, propagar, reagir e descartar erros. Essas responsabilidades são conhecidas coletivamente como "tratamento de erros". ```panic!``` e ```Result``` são semelhantes no sentido de que cada um é a interface primária de seus respectivos sistemas de tratamento de erros; no entanto, o significado que essas interfaces atribuem a seus erros e as responsabilidades que elas cumprem dentro de seus respectivos sistemas de tratamento de erros diferem.

O macro ```panic!``` é usada para construir erros que representam um bug que foi detectado em seu programa. Com ```panic!```, você fornece uma mensagem que descreve o bug e a linguagem então constrói um erro com essa mensagem, reporta-o e propaga-o para você.

```Result```, por outro lado, é usado para encapsular outros tipos que representam o resultado bem-sucedido de alguma computação, ```Ok(T)```, ou tipos de erro que representam um modo de falha de tempo de execução antecipado dessa computação, ```Err(E)```. ```Result``` é usado junto com tipos definidos pelo usuário que representam os vários modos de falha de tempo de execução antecipados que a computação associada pode encontrar. ```Result``` deve ser propagado manualmente, geralmente com a ajuda do operador ```?``` (interrogação) e da trait ```Try```, e deve ser relatado manualmente, geralmente com a ajuda da trait ```Error```.

Para obter informações mais detalhadas sobre o tratamento de erros, consulte [Tratamento de erro](erro2.md#arataca89) ou a documentação do módulo [std::result](https://doc.rust-lang.org/std/result/index.html). 

### Exemplos

```
panic!();
panic!("this is a terrible mistake!");
panic!("this is a {} {message}", "fancy", message = "message");
std::panic::panic_any(4); // panic with the value of 4 to be collected elsewhere
```

## Referências

[std::panic](https://doc.rust-lang.org/std/macro.panic.html)

---

arataca89@gmail.com

Última atualização: 20241230