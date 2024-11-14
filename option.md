# Option

```std::option::Option``` é uma enumeração definida na biblioteca padrão da linguagem Rust.

O tipo ```Option``` representa um valor opcional: cada ```Option``` é ```Some``` e contém um valor, ou ```None```, e não contém.

```
pub enum Option<T> {
    None,
    Some(T),
}
```

Tipos ```Option``` são muito comuns no código Rust, pois têm uma série de usos: 

* Valores iniciais
* Valores de retorno para funções que não são definidas em todo o intervalo de entrada (funções parciais)
* Valor de retorno para relatar erros simples, onde ```None``` é retornado em caso de erro
* Campos de estrutura opcionais
* Campos de estrutura que podem ser emprestados ou "tomados"
* Argumentos de função opcionais
* Ponteiros que podem ser nulos
* Trocar coisas em situações difíceis

```Option``` é comumente usada com correspondência de padrões para consultar a presença de um valor e tomar medidas, sempre levando em consideração o caso ```None```.

```
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// The return value of the function is an option
let result = divide(2.0, 3.0);

// Pattern match to retrieve the value
match result {
    // The division was valid
    Some(x) => println!("Result: {x}"),
    // The division was invalid
    None    => println!("Cannot divide by 0"),
}
```

### Métodos

[>>>](#is_some) is_some( ) - retorna ```true``` se o ```Option``` é um ```Some```.

[>>>](#is_some_and) is_some_and() - retorna ```true``` se a ```Option``` é um ```Some``` e o valor dentro dela atende a closure passada como argumento.

[>>>](#is_none) is_none() - retorna ```true``` se o ```Option``` é um ```None```.

[>>>](#is_none_or) is_none_or() - retorna ```true``` se a ```Option``` é um ```None``` ou o valor ```Some``` dentro dela atende a closure passada como argumento.

[>>>]

---

### is_some()

Retorna ```true``` se o ```Option``` é um ```Some```.

```
let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);

let x: Option<u32> = None;
assert_eq!(x.is_some(), false);
```

---

### is_some_and()

Retorna ```true``` se a ```Option``` é um ```Some``` e o valor dentro dela atende a closure passada como argumento.

```
let x: Option<u32> = Some(2);
assert_eq!(x.is_some_and(|x| x > 1), true);

let x: Option<u32> = Some(0);
assert_eq!(x.is_some_and(|x| x > 1), false);

let x: Option<u32> = None;
assert_eq!(x.is_some_and(|x| x > 1), false);
```

---

### is_none()

Retorna ```true``` se o ```Option``` é um ```None```.

```
let x: Option<u32> = Some(2);
assert_eq!(x.is_none(), false);

let x: Option<u32> = None;
assert_eq!(x.is_none(), true);
```

---

### is_none_or()

Retorna ```true``` se a ```Option``` é um ```None``` ou o valor ```Some``` dentro dela atende a closure passada como argumento.

```
let x: Option<u32> = Some(2);
assert_eq!(x.is_none_or(|x| x > 1), true);

let x: Option<u32> = Some(0);
assert_eq!(x.is_none_or(|x| x > 1), false);

let x: Option<u32> = None;
assert_eq!(x.is_none_or(|x| x > 1), true);
```

---

### as_ref()

asd

---

## Referências

[Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html)

[Módulo Option](https://doc.rust-lang.org/stable/std/option/index.html)

---

arataca89@gmail.com

Última atualização: 20241114