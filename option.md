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

[>>>](#as_ref) as_ref() - converte de ```&Option<T>``` para ```Option<&T>.```

[>>>](#as_mut) as_mut() - converte de ```&mut Option<T>``` para ```Option<&mut T>```.

[>>>](#as_pin_ref) as_pin_ref() - converte de ```Pin<&Option<T>>``` para ```Option<Pin<&T>>```.)

[>>>](#as_pin_mut) as_pin_mut() - converte de ```Pin<&mut Option<T>>``` para ```Option<Pin<&mut T>>```. )

[>>>](as_slice) as_slice() - retorna uma slice do valor contido, se houver. Se este for ```None```, uma slice vazia é retornada.

[>>>](#as_mut_slice) as_mut_slice() - retorna uma slice mutável do valor contido, se houver. Se este for ```None```, uma slice vazia é retornada.

[>>>](#expect) expect(&str) - retorna o valor contido em ```Some```, consumindo ```self```. Se o valor for ```None``` emite ```panic!``` com a mensagem passada como argumento.

[>>>](#unwrap) unwrap() - retorna o valor contido em ```Some```, consumindo ```self```. Caso o valor seja ```None``` emite ```panic!```.

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

Converte de ```&Option<T>``` para ```Option<&T>.```

Exemplo:

Calcula o comprimento de um ```Option<String>``` como um ```Option<usize>``` sem mover o ```String```. O método ```map()``` recebe o argumento ```self``` por valor, consumindo o original, então esta técnica usa ```as_ref()``` para primeiro pegar um ```Option``` com uma referência ao valor dentro do original.

```
 let text: Option<String> = Some("Hello, world!".to_string());
// Primeiro, converte `Option<String>` para `Option<&String>` com `as_ref`,
// então consome essa `Option<&String>` com `map`, deixando `text` na pilha.
let text_length: Option<usize> = text.as_ref().map(|s| s.len());
println!("still can print text: {text:?}");
```

---

### as_mut()

Converte de ```&mut Option<T>``` para ```Option<&mut T>```.

```
let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
```

---

### as_pin_ref()

Converte de ```Pin<&Option<T>>``` para ```Option<Pin<&T>>```.

[std::pin::Pin](https://doc.rust-lang.org/stable/std/pin/struct.Pin.html)

[Module std::pin](https://doc.rust-lang.org/stable/std/pin/index.html)

---

### as_pin_mut()

Converte de ```Pin<&mut Option<T>>``` para ```Option<Pin<&mut T>>```. 

[std::pin::Pin](https://doc.rust-lang.org/stable/std/pin/struct.Pin.html)

[Module std::pin](https://doc.rust-lang.org/stable/std/pin/index.html)

---

### as_slice()

Retorna uma slice do valor contido, se houver. Se este for ```None```, uma slice vazia é retornada. Isso pode ser útil para ter um único tipo de iterador sobre um ```Option``` ou slice.

Nota: Caso você tenha um ```Option<&T>``` e deseje obter uma slice de ```T```, você pode descompactá-lo via ```opt.map_or(&[], std::slice::from_ref)```.

```
assert_eq!(
    [Some(1234).as_slice(), None.as_slice()],
    [&[1234][..], &[][..]],
);
```

O inverso desta função é (descontando empréstimos) ```[_]::first```:

```
for i in [Some(1234_u16), None] {
    assert_eq!(i.as_ref(), i.as_slice().first());
}
```

---

### as_mut_slice()

Retorna uma slice mutável do valor contido, se houver. Se este for ```None```, uma slice vazia é retornada. Isso pode ser útil para ter um único tipo de iterador sobre um ```Option``` ou slice.

Nota: Se você tiver um ```Option<&mut T>``` em vez de um ```&mut Option<T>```, que este método recebe, você pode obter uma fatia mutável via ```opt.map_or(&mut [], std::slice::from_mut)```.

```
assert_eq!(
    [Some(1234).as_mut_slice(), None.as_mut_slice()],
    [&mut [1234][..], &mut [][..]],
);
```

O resultado é uma slice mutável de zero ou um item que aponta para nossa ```Option``` original: 

```
let mut x = Some(1234);
x.as_mut_slice()[0] += 1;
assert_eq!(x, Some(1235));
```

O inverso desta função é (descontando empréstimos) ```[_]::first```:

```
assert_eq!(Some(123).as_mut_slice().first_mut(), Some(&mut 123))
```

---

### expect()

Retorna o valor contido em ```Some```, consumindo ```self```.

Emite um pânico se o valor for ```None``` com uma mensagem de pânico personalizada fornecida pelo argumento passado.

```
let x = Some("value");
assert_eq!(x.expect("fruits are healthy"), "value");
```

```
let x: Option<&str> = None;
x.expect("fruits are healthy"); // emite um pânico com a mensagem `fruits are healthy`
```

Recomenda-se que as mensagens passadas como argumento sejam usadas para descrever o motivo pelo qual você espera que a ```Option``` seja ```Some```.

```
let item = slice.get(0)
    .expect("a slice não deve ser vazia"); 
```

Dica: Se você tiver problemas para criar mensagens de erro significativas, lembre-se de focar na palavra "deve", como em "a variável de ambiente deve ser definida por blah" ou "o binário fornecido deve estar disponível e executável pelo usuário atual". 

Para mais detalhes sobre os estilos de mensagens esperados e o raciocínio por trás de nossa recomendação, consulte a seção [Common Message Styles](https://doc.rust-lang.org/stable/std/error/index.html#common-message-styles) na documentação do módulo [std::error](https://doc.rust-lang.org/stable/std/error/index.html).

---

### unwrap()

Retorna o valor contido em ```Some```, consumindo ```self```. Caso o valor seja ```None``` emite ```panic!```.

Como essa função pode causar pânico, seu uso é geralmente desencorajado. Em vez disso, prefira usar correspondência de padrões (```match```) e lidar com o caso ```None``` explicitamente, ou chamar ```unwrap_or```, ```unwrap_or_else``` ou ```unwrap_or_default```.

```
let x = Some("air");
assert_eq!(x.unwrap(), "air");
```

```
let x: Option<&str> = None;
assert_eq!(x.unwrap(), "air"); // lança panic!
```

---

### unwrap_or

Retorna o valor contido em ```Some``` ou o valor default fornecido.

Argumentos passados para ```unwrap_or``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```unwrap_or_else```. 

```
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
```

---

### unwrap_or_else

asd

---

## Referências

[Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html)

[Módulo Option](https://doc.rust-lang.org/stable/std/option/index.html)

---

arataca89@gmail.com

Última atualização: 20241119
