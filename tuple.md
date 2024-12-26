#### arataca89

# Tupla

Uma tupla é uma forma de agrupar valores com uma variedade de tipos em um único tipo composto. As tuplas têm um comprimento fixo: uma vez declaradas, elas não podem crescer ou diminuir de tamanho.

Criamos uma tupla escrevendo uma lista de valores separados por vírgulas entre parênteses. Cada posição na tupla tem um tipo, e os tipos dos diferentes valores na tupla não precisam ser os mesmos. Adicionamos anotações de tipo opcionais neste exemplo: 

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

A variável **tup** está vinculada à tupla inteira porque uma tupla é considerada um único elemento composto. Para obter os valores individuais de uma tupla, podemos usar a correspondência de padrões para desestruturar um valor de tupla.

```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor de 'y' é : {y}");
}
``` 
Este programa primeiro cria uma tupla e a vincula à variável **tup**. Em seguida, ele usa um **let** para pegar **tup** e transformá-la em três variáveis ​​separadas, **x**, **y** e **z**. Isso é chamado de **desestruturação** porque quebra a tupla em três partes. Finalmente, o programa imprime o valor de **y**, que é 6.4.

Também podemos acessar um elemento de tupla diretamente usando um ponto (**.**) seguido pelo índice do valor que queremos acessar. Por exemplo:

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Este programa cria a tupla **x** e então acessa cada elemento da tupla usando seus respectivos índices. Como na maioria das linguagens de programação, o primeiro índice em uma tupla é 0.

A tupla sem nenhum valor tem um nome especial, **unidade**. Este valor e seu tipo correspondente são ambos escritos ```()``` e representam um valor vazio ou um tipo de retorno vazio. Expressões implicitamente retornam o valor da unidade se não retornarem nenhum outro valor. 

## Referências

[The Book - Chapter 3 - The Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

---

arataca89@gmail.com

Última atualização: 20241226