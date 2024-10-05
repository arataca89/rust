# Linguagem Rust - iterators

Iteradores permitem que você execute alguma operação em uma sequência de itens, um de cada por vez. Um iterador (iterator) é responsável pela lógica de iterar sobre cada item e indicar quando a sequência terminou. Quando você usa iteradores, você não precisa reimplementar essa lógica sozinho. 

Em Rust, iteradores são preguiçosos, o que significa que eles não têm efeito até que você chame métodos que consomem o iterador para usá-lo. Por exemplo, o código abaixo cria um iterador sobre os itens do vetor ```v1``` chamando o método ```iter()``` definido em ```Vec<T>```. Este código por si só não faz nada útil.

```
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

No exemplo abaixo, separamos a criação do iterador do uso do iterador no loop for. Quando o loop for é chamado usando o iterador em ```v1_iter```, cada elemento no iterador é usado em uma iteração do loop, que imprime cada valor. 

```
fn main(){

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Iterando sobre: {val}");
    }
}
```

Em linguagens que não possuem iteradores fornecidos por suas bibliotecas padrão, você provavelmente escreveria essa mesma funcionalidade iniciando uma variável no índice 0, usando essa variável para indexar o vetor e obter um valor e incrementando o valor da variável em um loop até que ele atingisse o número total de itens no vetor. 

Iteradores cuidam de toda essa lógica para você, reduzindo a repetição de código que você poderia potencialmente estragar. Iteradores lhe dão mais flexibilidade para usar a mesma lógica com muitos tipos diferentes de sequências, não apenas estruturas de dados que você pode indexar, como vetores. Vamos examinar como os iteradores fazem isso. 


[1. A trait Iterator e o método next](#1-A-trait-Iterator-e-o-método-next)

[2. Métodos que consomem um iterator](#2-Métodos-que-consomem-um-iterator)

[3. Metodos que produzem outros iteradores](#3-Métodos-que-produzem-outros-iteradores)

[4. Usando closures que capturam seu ambiente](#4-Usando-closures-que-capturam-seu-ambiente)

---

## 1. A trait Iterator e o método next

Todos os iteradores implementam a trait ```Iterator``` que é definida na biblioteca padrão.

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // métodos com implementações padrão omitidas
}
```

Observe que esta definição usa uma nova sintaxe: ```type Item``` e ```Self::Item```, que estão definindo um tipo associado a esta trait. Falaremos sobre tipos associados mais a frente. Por enquanto, tudo o que você precisa saber é que este código diz que implementar a trait ```Iterator``` requer que você também defina um tipo ```Item```, e este tipo é usado como retorno do método ```next()```. Em outras palavras, o tipo ```Item``` será o tipo retornado pelo iterador. 

A trait ```Iterator``` exige que os implementadores definam apenas um método: ```next()```, que retorna um item do iterador de cada vez, encapsulado em ```Some``` e, quando a iteração termina, retorna  ```None```. 

Podemos chamar o método ```next()``` diretamente.

```
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

Observe que precisamos tornar ```v1_iter``` mutável: chamar o método ```next()``` em um iterador altera o estado interno que o iterador usa para acompanhar onde ele está na sequência. Em outras palavras, este código consome, ou usa, o iterador. Cada chamada a ```next()``` consome um item do iterador. Não precisamos tornar ```v1_iter``` mutável quando usamos um loop for porque o loop assumiu a propriedade de ```v1_iter``` e o tornou mutável nos bastidores. 

Observe também que os valores que obtemos das chamadas a ```next()``` são referências imutáveis para os valores no vetor. O método ```iter()``` produz um iterador sobre referências imutáveis. Se quisermos criar um iterador que assuma a propriedade de ```v1``` e retorne a propriedade dos valores, podemos chamar ```into_iter()``` em vez de ```iter()```. Da mesma forma, se quisermos iterar sobre referências mutáveis, podemos chamar ```iter_mut()```em vez de ```iter()```.

```
#[test]
fn into_iter_demonstration() {
	let v = [1, 2, 3];
	let mut iter = v.into_iter();

	assert_eq!(Some(1), iter.next());
	assert_eq!(Some(2), iter.next());
	assert_eq!(Some(3), iter.next());
	assert_eq!(None, iter.next());
}
```


## 2. Métodos que consomem um iterator

A trait ```Iterator``` possui vários métodos diferentes com implementações padrão fornecidas pela biblioteca padrão; você pode descobrir sobre esses métodos consultando a documentação da API da biblioteca padrão para a [trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html). Alguns desses métodos chamam o método ```next()``` em sua definição, razão pela qual você é obrigado a implementar o método ```next()``` ao implementar a trait ```Iterator```.

Métodos que chamam ```next()``` são chamados "consuming adaptors" (adaptadores consumidores), porque chamá-los consome o iterador. Um exemplo é o método ```sum()```, que assume a propriedade do iterador e itera pelos itens chamando ```next()``` repetidamente, consumindo assim o iterador. À medida que itera, ele adiciona cada item a um total e retorna este total quando a iteração estiver completa. 

```
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

Não podemos usar ```v1_iter``` após a chamada a ```sum()``` porque ```sum()``` assume a propriedade do iterador no qual o chamamos. 

## 3. Métodos que produzem outros iteradores

Adaptadores de iterador (Iterator adaptors) são métodos definidos na trait ```Iterator``` que não consomem o iterador. Em vez disso, eles produzem iteradores diferentes alterando algum aspecto do iterador original. 

O exemplo abaixo mostra o uso do método adaptador de iterador ```map()```, que recebe uma closure para chamar em cada item à medida que os itens são iterados. O método ```map()``` retorna um novo iterador que produz os itens modificados. A closure aqui cria um novo iterador em que cada item do vetor será incrementado de 1.

```
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```

Porém, este código emite advertências ao ser compilado.

```
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = v1.iter().map(|x| x + 1);
  |     +++++++

warning: `iterators` (bin "iterators") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`
```

O código não faz nada; a closure que especificamos nunca é chamada. O aviso nos lembra o porquê: adaptadores de iterador são preguiçosos, e precisamos consumir o iterador aqui. 

Para corrigir este aviso e consumir o iterador, usaremos o método ```collect()```. Este método consome o iterador e coleta os valores resultantes em um tipo de dados de coleção. 

No código abaixo, coletamos os resultados da iteração sobre o iterador retornado da chamada para ```map()``` em um vetor. Este vetor acabará contendo cada item do vetor original incrementado de 1.

```
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

Porque ```map()``` recebe uma closure, podemos especificar qualquer operação que queremos realizar em cada item. Este é um ótimo exemplo de como closures permitem que você personalize algum comportamento enquanto reutiliza o comportamento de iteração que a trait ```Iterator``` fornece. 
 
Você pode encadear várias chamadas para adaptadores de iterador para realizar ações complexas de forma legível. Mas, como todos os iteradores são preguiçosos, você precisa chamar um dos métodos de adaptador consumidor para obter resultados das chamadas para adaptadores de iterador. 

## 4. Usando closures que capturam seu ambiente

Muitos adaptadores de iterador recebem closures como argumentos, e comumente as closures que especificaremos como argumentos para adaptadores de iterador serão closures que capturam seu ambiente. 

Neste exemplo, usaremos o método ```filter()``` que recebe uma closure. A closure recebe um item do iterador e retorna um bool. Se a closure retornar true, o valor será incluído na iteração produzida por ```filter()```. Se a closure retornar false, o valor não será incluído.

O método ```filter()``` foi usado com uma closure que captura a variável ```shoe_size``` do seu ambiente para iterar sobre uma coleção de instâncias da estrutura ```Shoe```. Ele retornará apenas sapatos que são do tamanho especificado. 

```
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
``` 
A função ```shoes_in_size()``` assume a propriedade de um ```Vec<Shoe>``` e um tamanho como parâmetros. Ela retorna um vetor contendo apenas sapatos do tamanho especificado. 

No corpo de ```shoes_in_size()```, chamamos ```into_iter()``` para criar um iterador que assume a propriedade do vetor. Então chamamos ```filter()``` para adaptar esse iterador em um novo iterador que contém apenas elementos para os quais a closure retorna ```true```. 

A closure captura o parâmetro ```shoe_size``` do ambiente e compara o valor com o tamanho de cada sapato, mantendo apenas os sapatos do tamanho especificado. Finalmente, ```collect()``` é chamado e coleta os valores retornados pelo iterador adaptado em um vetor que é retornado pela função. 

---

## Referências

[capítulo 13 do "Livro"](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

[into_iter() reference](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#tymethod.into_iter)


---

arataca89@gmail.com

Última atualização: 20241005
