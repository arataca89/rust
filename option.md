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

[>>>](#unwrap_or_else) unwrap_or_else(closure) - retorna o valor contido em ```Some``` ou, em caso de ```None```, calcula o valor a partir da closure passada como argumento.

[>>>](#unwrap_or_default) unwrap_or_default() - retorna o valor contido em ```Some``` ou o valor default para o tipo de dados em questão.

[>>>](#unwrap_unchecked) unwrap_unchecked() - retorna o valor contido em ```Some```, consumindo ```self```, sem verificar se o valor não é ```None```.

[>>>](#map) map(closure) - mapeia ```Option<T>``` para ```Option<U>``` aplicando a closure passada como argumento ao valor contido (se ```Some```) ou retorna ```None``` (se ```None```).

[>>>](#inspect) inspect(closure) - executa a closure passada como argumento com uma referência ao valor contido em ```Some```, se houver. Retorna a ```Option``` original.

[>>>](#map_or) map_or(default, closure) - se a ```Option``` for ```None``` retorna o valor default fornecido como primeiro argumento. Se a ```Option``` for ```Some``` executa a closure passada como segundo argumento com o valor dentro de ```Some```.

[>>>](#map_or_else) map_or_else(closure1, closure2) - se a ```Option``` for ```None``` executa a closure1 passada como primeiro argumento. Se a ```Option``` for ```Some``` executa a closure2 passada como segundo argumento com o valor dentro de ```Some```.

[>>>](#ok_or) ok_or() - transforma um ```Option<T>``` em um ```Result<T, E>```, mapeando ```Some(v)``` para ```Ok(v)``` e ```None``` para ```Err(err)```.

[>>>](#ok_or_else) ok_or_else(closure) - transforma um ```Option<T>``` em um ```Result<T, E>```, mapeando ```Some(v)``` para ```Ok(v)``` e ```None``` para ```Err(err())```, onde ```err()``` é a closure passada como argumento.

[>>>](#as_deref) as_deref() - converte de ```Option<T>``` (ou ```&Option<T>```) para ```Option<&T::Target>```.

[>>>](#as_deref_mut) as_deref_mut() - converte de ```Option<T>``` (ou ```&mut Option<T>```) para ```Option<&mut T::Target>```.

[>>>](#iter) iter() - retorna um iterador sobre o valor possivelmente contido. 

[>>>](#iter_mut) iter_mut() - retorna um iterador mutável sobre o valor possivelmente contido. 

[>>>](#and) and(option) - se a ```Option``` for ```None```, retorna ```None```; caso contrário retorna o argumento passado (```option```).

[>>>](#and_then) and_then(closure) - se a ```Option``` for ```None```, retorna ```None``` , caso contrário, chama a closure passada como argumento com o valor encapsulado em ```Some``` e retorna o resultado.

[>>>](#or) or(option) - retorna a ```Option``` se ela contém um valor, caso contrário retorna a opcão passada como argumento (```option```).

[>>>](#or_else) or_else(closure) - retorna a ```Option``` se ela contém um valor, caso contrário chama a closure passada como argumenmto e retorna o resultado.

[>>>](#xor) xor(option) - retorna ```Some``` se um dos dois: ```self``` ou a opção passada como argumento(```option```) for ```Some```, somente um dos dois; caso contrário retorna ```None```.

[>>>](#insert) insert(value) - insere um valor na ```Option``` e, em seguida, retorna uma referência mutável para ele. Se a ```Option``` já contém um valor, o valor antigo é descartado.

[>>>](#get_or_insert) get_or_insert(value) - insere um valor na ```Option``` se ela for ```None```, então retorna uma referência mutável para o valor contido.

[>>>](#get_or_insert_with) get_or_insert_with(closure) - se a ```Option``` for ```None```, insere o valor calculado a partir da closure passada como argumento e retorna uma referência mutável para o valor.

[>>>](#take) take() - move o valor da ```Option``` deixando ```None``` em seu lugar.

[>>>](#take_if) take_if(closure) - se a closure predicado passada como argumento for ```true```, move o valor da ```Option``` deixando ```None``` em seu lugar.

[>>>](#replace) replace(value) - substitui o valor atual da ```Option``` retornando o valor antigo se presente.

[>>>](#zip) zip(Option) - combina ```self``` com outra ```Option```.

[>>>](#unzip) unzip() - descompacta uma ```Option``` contendo uma tupla de duas ```Option```.

[>>>](#copied) copied() - mapeia um ```Option<&T>``` para um ```Option<T>``` copiando o conteúdo da opção.

[>>>](#cloned) cloned() - mapeia um ```Option<&T>``` para um ```Option<T>``` clonando o conteúdo da opção.

[>>>](#copied) copied() - mapeia um ```Option<&mut T>``` para um ```Option<T>``` copiando o conteúdo da opção.

[>>>](#cloned) cloned() - mapeia um ```Option<&mut T>``` para um ```Option<T>``` clonando o conteúdo da opção.

[>>>](#transpose) transpose() - transpõe uma ```Option``` contendo um ```Result``` para um ```Result``` contendo uma ```Option```.

[>>>](#flatten) flatten() - converte de ```Option<Option<T>>``` para ```Option<T>```.

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

### unwrap_or()

Retorna o valor contido em ```Some``` ou o valor default fornecido.

Argumentos passados para ```unwrap_or``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```unwrap_or_else```. 

```
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
```

---

### unwrap_or_else()

Retorna o valor contido em ```Some``` ou, em caso de ```None```, calcula o valor a partir da closure passada como argumento.

```
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
```

---

### unwrap_or_default()

Retorna o valor contido em ```Some``` ou o valor default para o tipo de dados em questão.

Consome ```self``` e, se for um ```Some```, retorna o valor contido, caso contrário, se for um ```None```, retorna o valor default para esse tipo.

```
let x: Option<u32> = None;
let y: Option<u32> = Some(12);

assert_eq!(x.unwrap_or_default(), 0);
assert_eq!(y.unwrap_or_default(), 12);
```

---

### unwrap_unchecked()

Retorna o valor contido em ```Some```, consumindo ```self```, sem verificar se o valor não é ```None```.


Chamar este método em um ```None``` provoca um comportamento indefinido. 

```
let x = Some("air");
assert_eq!(unsafe { x.unwrap_unchecked() }, "air");
```

```
let x: Option<&str> = None;
assert_eq!(unsafe { x.unwrap_unchecked() }, "air"); // comportamento indefinido
```

---

### map()

Mapeia ```Option<T>``` para ```Option<U>``` aplicando a closure passada como argumento ao valor contido (se ```Some```) ou retorna ```None``` (se ```None```).

Exemplo:

Calcula o comprimento de um ```Option<String>``` como um ```Option<usize>```, consumindo o original: 

```
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` pega `self` por valor, consumindo `maybe_some_string`
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));

let x: Option<&str> = None;
assert_eq!(x.map(|s| s.len()), None);
```

---

### inspect()

Executa a closure passada como argumento com uma referência ao valor contido em ```Some```, se houver. Retorna a ```Option``` original.

```
let list = vec![1, 2, 3];

// exibe na tela "got: 2"
let x = list
    .get(1)
    .inspect(|x| println!("got: {x}"))
    .expect("list should be long enough");

// não exibe nada
list.get(5).inspect(|x| println!("got: {x}"));
```

---

### map_or()

Se a ```Option``` for ```None``` retorna o valor default fornecido como primeiro argumento. Se a ```Option``` for ```Some``` executa a closure passada como segundo argumento com o valor dentro de ```Some```.

Argumentos passados para ```map_or()``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```map_or_else```.

```
let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);

let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);
```

---

###  map_or_else()

Se a ```Option``` for ```None``` executa a closure passada como primeiro argumento. Se a ```Option``` for ```Some``` executa a closure passada como segundo argumento com o valor dentro de ```Some```.

```
let k = 21;

let x = Some("foo");
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

let x: Option<&str> = None;
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
```

Lidando com um "plano B" baseado em ```Result```

Uma ocorrência um tanto comum ao lidar com valores opcionais em combinação com ```Result<T, E>``` é o caso em que se deseja invocar um "plano B" se a opção não estiver presente. Este exemplo analisa um argumento de linha de comando (se presente) ou o conteúdo de um arquivo para um inteiro. No entanto, ao contrário de acessar o argumento da linha de comando, a leitura do arquivo é falível, portanto, deve ser encapsulada em um ```Ok```. 

```
#![allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v: u64 = std::env::args()
       .nth(1)
       .map_or_else(|| std::fs::read_to_string("/etc/someconfig.conf"), Ok)?
       .parse()?;
      Ok(())
}
```

---

### ok_or()

Transforma um ```Option<T>``` em um ```Result<T, E>```, mapeando ```Some(v)``` para ```Ok(v)``` e ```None``` para ```Err(err)```.

Argumentos passados para ```ok_or()``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```ok_or_else()```.

```
let x = Some("foo");
assert_eq!(x.ok_or(0), Ok("foo"));

let x: Option<&str> = None;
assert_eq!(x.ok_or(0), Err(0));
```

---

### ok_or_else()

Transforma um ```Option<T>``` em um ```Result<T, E>```, mapeando ```Some(v)``` para ```Ok(v)``` e ```None``` para ```Err(err())```, onde ```err()``` é a closure passada como argumento.

```
let x = Some("foo");
assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

let x: Option<&str> = None;
assert_eq!(x.ok_or_else(|| 0), Err(0));
```

---

### as_deref()

Converte de ```Option<T>``` (ou ```&Option<T>```) para ```Option<&T::Target>```.

Deixa o ```Option``` original no lugar, criando um novo com uma referência ao original, além de forçar o conteúdo via ```Deref```.

```
let x: Option<String> = Some("hey".to_owned());
assert_eq!(x.as_deref(), Some("hey"));

let x: Option<String> = None;
assert_eq!(x.as_deref(), None);
```

---

### as_deref_mut()

Converte de ```Option<T>``` (ou ```&mut Option<T>```) para ```Option<&mut T::Target>```.

Deixa o ```Option``` original no lugar, criando um novo contendo uma referência mutável para o tipo ```Deref::Target``` do tipo interno. 

```
let mut x: Option<String> = Some("hey".to_owned());
assert_eq!(x.as_deref_mut().map(|x| {
    x.make_ascii_uppercase();
    x
}), Some("HEY".to_owned().as_mut_str()));
```

---

### iter()

Retorna um iterador sobre o valor possivelmente contido. 

```
let x = Some(4);
assert_eq!(x.iter().next(), Some(&4));

let x: Option<u32> = None;
assert_eq!(x.iter().next(), None);
```

---

### iter_mut()

Retorna um iterador mutável sobre o valor possivelmente contido. 

```
let mut x = Some(4);
match x.iter_mut().next() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));

let mut x: Option<u32> = None;
assert_eq!(x.iter_mut().next(), None);
```

---

### and()

Se a ```Option``` for ```None```, retorna ```None```; caso contrário retorna o argumento passado.

Argumentos passados para ```and()``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```and_then()```. 

```
let x = Some(2);
let y: Option<&str> = None;
assert_eq!(x.and(y), None);

let x: Option<u32> = None;
let y = Some("foo");
assert_eq!(x.and(y), None);

let x = Some(2);
let y = Some("foo");
assert_eq!(x.and(y), Some("foo"));

let x: Option<u32> = None;
let y: Option<&str> = None;
assert_eq!(x.and(y), None);
```

---

### and_then()

Se a ```Option``` for ```None```, retorna ```None``` , caso contrário, chama a closure passada como argumento com o valor encapsulado em ```Some``` e retorna o resultado. 

Algumas linguagens chamam essa operação de flatmap. 

```
fn sq_then_to_string(x: u32) -> Option<String> {
    x.checked_mul(x).map(|sq| sq.to_string())
}

assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
assert_eq!(None.and_then(sq_then_to_string), None);
```

Frequentemente usado para encadear operações falíveis que podem retornar ```None```.

```
let arr_2d = [["A0", "A1"], ["B0", "B1"]];

let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
assert_eq!(item_0_1, Some(&"A1"));

let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
assert_eq!(item_2_0, None);
```

---

### filter()

Se a ```Option``` for ```None```, retorna ```None```; caso contrário, chama a closure predicado com o valor encapsulado em ```Some``` e retorna:

* ```Some(t)``` se a closure retornar verdadeiro (onde ```t``` é o valor encapsulado), e
* ```None``` se a closure retornar falso.

A closure passada como argumento deve ser um predicado, ou seja, deve retornar ```true``` ou ```false```.

Esta função funciona de forma semelhante ao ```Iterator::filter()```. Você pode imaginar o ```Option<T>``` como um iterador sobre um ou zero elementos. ```filter()``` permite que você decida quais elementos manter. 

```
fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

assert_eq!(None.filter(is_even), None);
assert_eq!(Some(3).filter(is_even), None);
assert_eq!(Some(4).filter(is_even), Some(4));
```

---

### or()

Retorna a ```Option``` se ela contém um valor, caso contrário retorna a opcão passada como argumento.

Argumentos passados para ```or()``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```or_else()```. 

```
let x = Some(2);
let y = None;
assert_eq!(x.or(y), Some(2));

let x = None;
let y = Some(100);
assert_eq!(x.or(y), Some(100));

let x = Some(2);
let y = Some(100);
assert_eq!(x.or(y), Some(2));

let x: Option<u32> = None;
let y = None;
assert_eq!(x.or(y), None);
```

---

### or_else()

Retorna a ```Option``` se ela contém um valor, caso contrário chama a closure passada como argumenmto e retorna o resultado.

```
fn nobody() -> Option<&'static str> { None }
fn vikings() -> Option<&'static str> { Some("vikings") }

assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
assert_eq!(None.or_else(vikings), Some("vikings"));
assert_eq!(None.or_else(nobody), None);
```

---

### xor()

Retorna ```Some``` se um dos dois: ```self``` ou a opção passada como argumento for ```Some```, somente um dos dois; caso contrário retorna ```None```.

```
let x = Some(2);
let y: Option<u32> = None;
assert_eq!(x.xor(y), Some(2));

let x: Option<u32> = None;
let y = Some(2);
assert_eq!(x.xor(y), Some(2));

let x = Some(2);
let y = Some(2);
assert_eq!(x.xor(y), None);

let x: Option<u32> = None;
let y: Option<u32> = None;
assert_eq!(x.xor(y), None);
```

---

### insert()

Insere um valor na ```Option``` e, em seguida, retorna uma referência mutável para ele.

Se a ```Option``` já contém um valor, o valor antigo é descartado.

Veja também ```Option::get_or_insert()```, que não atualiza o valor se a opção já contém ```Some```.

```
let mut opt = None;
let val = opt.insert(1);
assert_eq!(*val, 1);
assert_eq!(opt.unwrap(), 1);
let val = opt.insert(2);
assert_eq!(*val, 2);
*val = 3;
assert_eq!(opt.unwrap(), 3);
```

---

### get_or_insert()

Insere um valor na ```Option``` se ela for ```None```, então retorna uma referência mutável para o valor contido.

Veja também ```Option::insert()```, que atualiza o valor mesmo que a opção já contenha ```Some```.

```
let mut x = None;

{
    let y: &mut u32 = x.get_or_insert(5);
    assert_eq!(y, &5);

    *y = 7;
}

assert_eq!(x, Some(7));
```

---

### get_or_insert_with()

Se a ```Option``` for ```None```, insere o valor calculado a partir da closure passada como argumento e retorna uma referência mutável para o valor.

```
let mut x = None;

{
    let y: &mut u32 = x.get_or_insert_with(|| 5);
    assert_eq!(y, &5);

    *y = 7;
}

assert_eq!(x, Some(7));
```

---

### take()

Move o valor da ```Option``` deixando ```None``` em seu lugar.

```
let mut x = Some(2);
let y = x.take();
assert_eq!(x, None);
assert_eq!(y, Some(2));

let mut x: Option<u32> = None;
let y = x.take();
assert_eq!(x, None);
assert_eq!(y, None);
```

---

### take_if()

Se a closure predicado passada como argumento for ```true```, move o valor da ```Option``` deixando ```None``` em seu lugar.

```
let mut x = Some(42);

let prev = x.take_if(|v| if *v == 42 {
    *v += 1;
    false
} else {
    false
});
assert_eq!(x, Some(43));
assert_eq!(prev, None);

let prev = x.take_if(|v| *v == 43);
assert_eq!(x, None);
assert_eq!(prev, Some(43));
```

---

### replace()

Substitui o valor atual da ```Option``` retornando o valor antigo se presente.

```
let mut x = Some(2);
let old = x.replace(5);
assert_eq!(x, Some(5));
assert_eq!(old, Some(2));

let mut x = None;
let old = x.replace(3);
assert_eq!(x, Some(3));
assert_eq!(old, None);
```

---

### zip()

Combina ```self``` com outra ```Option```.

Se ```self``` é ```Some(s)``` e o argumento passado é ```Some(o)```, este método retorna ```Some((s, o))```. Caso contrário, ```None``` é retornado.

```
let x = Some(1);
let y = Some("hi");
let z = None::<u8>;

assert_eq!(x.zip(y), Some((1, "hi")));
assert_eq!(x.zip(z), None);
```

---

### unzip()

Descompacta uma ```Option``` contendo uma tupla de duas ```Option```.

Se ```self``` for ```Some((a, b))```, este método retorna ```(Some(a), Some(b))```. Caso contrário, ```(None, None)``` é retornado.

```
let x = Some((1, "hi"));
let y = None::<(u8, u32)>;

assert_eq!(x.unzip(), (Some(1), Some("hi")));
assert_eq!(y.unzip(), (None, None));
```

---

### copied()

Mapeia um ```Option<&T>``` para um ```Option<T>``` copiando o conteúdo da opção. 

```
let x = 12;
let opt_x = Some(&x);
assert_eq!(opt_x, Some(&12));
let copied = opt_x.copied();
assert_eq!(copied, Some(12));
```

---

### cloned()

Mapeia um ```Option<&T>``` para um ```Option<T>``` clonando o conteúdo da opção.

```
let x = 12;
let opt_x = Some(&x);
assert_eq!(opt_x, Some(&12));
let cloned = opt_x.cloned();
assert_eq!(cloned, Some(12));
```

---

### copied()

Mapeia um ```Option<&mut T>``` para um ```Option<T>``` copiando o conteúdo da opção.

```
let mut x = 12;
let opt_x = Some(&mut x);
assert_eq!(opt_x, Some(&mut 12));
let copied = opt_x.copied();
assert_eq!(copied, Some(12));
```

---

### cloned()

Mapeia um ```Option<&mut T>``` para um ```Option<T>``` clonando o conteúdo da opção.

```
let mut x = 12;
let opt_x = Some(&mut x);
assert_eq!(opt_x, Some(&mut 12));
let cloned = opt_x.cloned();
assert_eq!(cloned, Some(12));
```

---

### transpose()

Transpõe uma ```Option``` contendo um ```Result``` para um ```Result``` contendo uma ```Option```.

```None``` será mapeado para ```Ok(None)```. ```Some(Ok(_))``` e ```Some(Err(_))``` serão mapeados para ```Ok(Some(_))``` e ```Err(_)```.

```
#[derive(Debug, Eq, PartialEq)]
struct SomeErr;

let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
assert_eq!(x, y.transpose());
```

---

### flatten

Converte de ```Option<Option<T>>``` para ```Option<T>```.

```
let x: Option<Option<u32>> = Some(Some(6));
assert_eq!(Some(6), x.flatten());

let x: Option<Option<u32>> = Some(None);
assert_eq!(None, x.flatten());

let x: Option<Option<u32>> = None;
assert_eq!(None, x.flatten());
```

```flatten()``` remove apenas um nível de aninhamento por vez:

```
let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
assert_eq!(Some(Some(6)), x.flatten());
assert_eq!(Some(6), x.flatten().flatten());
```

---

## Referências

[Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html)

[Módulo Option](https://doc.rust-lang.org/stable/std/option/index.html)

---

arataca89@gmail.com

Última atualização: 20241123
