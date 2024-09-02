# Tipos genéricos, traits e lifetime em Rust

Toda linguagem de programação tem ferramentas para efetivamente tratar a duplicação de conceitos e códigos. Em Rust, uma destas ferramentas são os genéricos. Como o próprio nome já indica, o tipo genérico em um código significa que este código poderá ser executado com qualquer tipo concreto e funcionará do mesmo jeito.

[1. Evitando a duplicação de código usando uma função](#1-Evitando-a-duplicação-de-código-usando-uma-função)


---

## 1. Evitando a duplicação de código usando uma função

Dada a função abaixo, que encontra o maior valor em uma lista de inteiros.

```
fn main() {
    let numeros = vec![34, 50, 25, 100, 65];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 100
}

```
Apesar deste código funcionar, se quisermos procurar o maior valor em outra lista de inteiros no mesmo programa, teremos que fazer algo semelhante a:

```
fn main() {
    let numeros = vec![34, 50, 25, 100, 65];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 100

    let numeros = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut maior = &numeros[0];

    for n in &numeros {
        if n > maior {
            maior = n;
        }
    }

    println!("O maior valor é {maior}"); // 6000
}

```
Note que houve duplicação de código e isto é cansativo e propenso a erros. Outra grande desvantagem da duplicação de código é no momento de realizar alguma correção ou alteração. Você terá que alterar seu código em vários locais e esquecer de um deles é bem comum.

Uma maneira de evitar esta duplicação de código seria criar uma função que recebe uma lista de inteiros e retorna seu maior valor.

```

fn maior(vetor_int: &[i32]) -> &i32 {

    let mut maior_valor = &vetor_int[0];

    for n in vetor_int {
        if n > maior_valor {
            maior_valor = n;
        }
    }

    maior_valor
}

fn main() {
    let v1 = vec![34, 50, 25, 100, 65];

    let x = maior(&v1);

    println!("O maior valor é {x}"); // 100

    let v2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let x = maior(&v2);

    println!("O maior valor é {x}"); // 6000
}

```
 

---
## Referências

[Capítulo 10 do livro](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

arataca89@gmail.com

Última atualização: 20240902
