# Result

```std::result::Result``` é uma enumeração definida na biblioteca padrão da linguagem Rust.

```
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Note que ela trabalha com os tipos genéricos ```T``` e ```E``` o que permite grande flexibilidade de uso com diversos tipos.

```Result``` é um tipo que representa sucesso (```Ok```) ou falha (```Err```).

A variante ```Ok``` contém o valor do sucesso ```T```, enquanto a varável ```Err``` contém o valor em caso de falha ```E```.

### Métodos

[>>>](#is_ok) is_ok( ) - retorna ```true``` se é um ```Ok```

[>>>](#is_ok_and) is_ok_and(closure) - retorna ```true``` se é um ```Ok``` e o valor embutido no ```Ok``` atende a closure

[>>>](#is_err) is_err( ) - retorna ```true``` se é um ```Err```

[>>>](#is_err_and) is_err_and(closure) - retorna ```true``` se é um ```Err``` e o valor embutido no ```Err``` atende a closure

[>>>](#ok) ok( ) - converte um ```Result<T, E>``` em um ```Option<T>```

[>>>](#err) err( ) - converte um ```Result<T, E>``` em um ```Option<E>```.

[>>>](#as_ref) as_ref( ) - converte de ```&Result<T, E>``` para ```Result<&T, &E>```

[>>>](#as_mut) as_mut( ) - converte de ```&mut Result<T, E>``` para ```Result<&mut T, &mut E>```

[>>>)](#map) map(closure) - mapeia de ```Result<T, E>``` para ```Result<U, E>``` aplicando a closure ao valor do ```Ok```

[>>>](#map_or) map_or(default, closure) - se é um ```Ok```, aplica a closure ao valor do ```Ok```; se é um ```Err```, retorna o valor default 

[>>>](#map_or_else) map_or_else(closure_err, closure_ok) - se  é um ```Ok```, aplica closure_ok ao valor de ```Ok```. Se é um ```Err```, aplica closure_err ao valor de ```Err```

[>>>](#map_err) map_err(closure) - mapeia de ```Result<T, E>``` para ```Result<T, F>``` aplicando a closure ao valor de ```Err```


[>>>](#inspect) inspect(closure) - aplica a closure ao valor contido no ```Ok``` sem alterar o valor original.

[>>>](#inspect_err) inspect_err(closure) - aplica a closure ao valor contido no ```Err``` sem alterar o valor original.

[>>>](#as_deref) as_deref() - retorna outro ```Result``` com referências aos valores do ```Result``` original

[>>>](#as_deref_mut) as_deref_mut() - retorna outro ```Result``` com referências mutáveis aos valores do ```Result``` original

[>>>](#iter) iter() - retorna um iterador para o valor embutido no ```Result```

[>>>](#expect) expect(&str) - retorna o valor embutido no ```Ok```. Em caso de erro chama ```panic!``` com a ```&str``` passada como argumento como mensagem de erro.

[>>>](#unwrap) unwrap() - retorna o valor embutido no ```Ok```. Em caso de erro chama ```panic!``` com as mensagens padrão.

[>>>](#unwrap_or_default) unwrap_or_default() - retorna o valor embutido no ```Ok`` ou o valor default para o tipo de dados em questão.

[>>>](#expect_err) expect_err(&str) - retorna o valor embutido no ```Err```. Se o valor é um ```Ok```, gera pânico com uma mensagem que tem o argumento ```&str``` e o valor do ```Ok```.

[>>>](#unwrap_err) unwrap_err() - retorna o valor embutido no ```Err```. Se o valor é um ```Ok```, gera pânico com uma mensagem de pânico fornecida pelo valor do ```Ok```. 

[>>>](#and) and() - retorna ```res``` se o resultado for ```Ok```, caso contrário retorna o valor ```Err``` de self.)

[>>>](#and_then) and_then(closure) - se for ```Ok```, chama a closure passada como argumento. Caso contrário retorna o valor ```Err``` de ```self```.

[>>>](#or) or() - retorna ```res``` se o resultado for ```Err```, caso contrário retorna o valor ```Ok``` de self.

[>>>](#or_else) or_else(closure) - se for ```Err```, chama a closure passada como argumento com o valoor de ```Err```. Caso contrário retorna o valor ```Ok``` de self.

[>>>](#unwrap_or) unwrap_or(default) - retorna o valor ```Ok``` contido ou o valor default fornecido.

[>>>](#unwrap_or_else) unwrap_or_else(closure) - retorna o valor ```Ok``` contido ou o calcula a partir da closure passada como argumento.

[>>>](#unwrap_unchecked) unwrap_unchecked() - retorna o valor ```Ok``` contido, consumindo ```self```, sem verificar se o valor não é um ```Err```.

[>>>](#unwrap_err_unchecked) unwrap_err_unchecked() - retorna o valor ```Err``` contido, consumindo ```self```, sem verificar se o valor não é um ```Ok```.

[>>>](#copied) copied() - mapeia um ```Result<&T, E>``` para um ```Result<T, E>``` copiando o conteúdo da parte ```Ok```.

[>>>](#cloned) cloned() - mapeia um ```Result<&T, E>``` para um ```Result<T, E>``` clonando o conteúdo da parte ```Ok```.

[>>>](#copied) copied() - mapeia um ```Result<&mut T, E>``` para um ```Result<T, E>``` copiando o conteúdo da parte ```Ok```.

[>>>](#cloned) cloned() - mapeia um ```Result<&mut T, E>``` para um ```Result<T, E>``` clonando o conteúdo da parte ```Ok```.

[>>>](#transpose) transpose() - transpõe de um ```Result``` com um ```Option``` para um ```Option``` com um ```Result```.

---

### is_ok()

Retorna ```true``` se o ```Result``` é um ```Ok```.

```
fn main() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}
```

---

### is_ok_and()

Retorna ```true``` se o ```Result``` for ```Ok``` e o valor dentro dele corresponder ao predicado passado como argumento.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.is_ok_and(|x| x > 1), true);

let x: Result<u32, &str> = Ok(0);
assert_eq!(x.is_ok_and(|x| x > 1), false);

let x: Result<u32, &str> = Err("hey");
assert_eq!(x.is_ok_and(|x| x > 1), false);
```

---

### is_err() 

Retorna ```true``` se o ```Result``` é ```Err```.

```
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_err(), false);

let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_err(), true);
```

---

### is_err_and()

Retorna ```true``` se o ```Result``` é ```Err``` e o valor dentro dele corresponde ao predicado passado como argumento.

```
use std::io::{Error, ErrorKind};

let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);

let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);

let x: Result<u32, Error> = Ok(123);
assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
```

---

### ok() 

Converte um ```Result<T,E>``` em um ```Option<T>```.

Converte ```self``` em um ```Option<T>```, consumindo ```self``` e descartando o erro, se houver.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.ok(), Some(2));

let x: Result<u32, &str> = Err("Nothing here");
assert_eq!(x.ok(), None);
```

---

### err() 

Converte um ```Result<T, E>``` em um ```Option<E>```.

Converte ```self``` para um ```Option<E>```, consumindo ```self``` e descartando o valor de sucesso, se houver. 

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.err(), None);

let x: Result<u32, &str> = Err("Nothing here");
assert_eq!(x.err(), Some("Nothing here"));
```

---

### as_ref()

Converte de ```&Result<T, E>``` para ```Result<&T, &E>```.

Produz um novo ```Result```, contendo uma referência para o original, deixando o original no lugar.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.as_ref(), Ok(&2));

let x: Result<u32, &str> = Err("Error");
assert_eq!(x.as_ref(), Err(&"Error"));
```

---

### as_mut()

Converte de ```&mut Result<T, E>``` para ```Result<&mut T, &mut E>```.

```
fn mutate(r: &mut Result<i32, i32>) {
    match r.as_mut() {
        Ok(v) => *v = 42,
        Err(e) => *e = 0,
    }
}

let mut x: Result<i32, i32> = Ok(2);
mutate(&mut x);
assert_eq!(x.unwrap(), 42);

let mut x: Result<i32, i32> = Err(13);
mutate(&mut x);
assert_eq!(x.unwrap_err(), 0);
```

---

### map()

Mapeia um ```Result<T, E>``` para um ```Result<U, E>``` aplicando a closure passada como argumento ao valor contido em ```Ok```, deixando o valor ```Err``` intocado.

```
let line = "1\n2\n3\n4\n";

for num in line.lines() {
    match num.parse::<i32>().map(|i| i * 2) {
        Ok(n) => println!("{n}"),
        Err(..) => {}
    }
}
```

---

### map_or()

Se o ```Result``` é um ```Ok```, aplica a closure passada como segundo argumento ao valor de ```Ok```.

Se o ```Result``` é um ```Err```, retorna o valor default passado como primeiro argumento.

```
let x: Result<_, &str> = Ok("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);

let x: Result<&str, _> = Err("bar");
assert_eq!(x.map_or(42, |v| v.len()), 42);
```

OBSERVAÇÃO: Os argumentos passados para ```map_or()``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```map_or_else()```. 

---

### map_or_else()

Se o ```Result``` é um ```Ok```, aplica a closure passada como segundo argumento ao valor de ```Ok```.

Se o ```Result``` é um ```Err```, aplica a closure passada como primeiro argumento ao valor de ```Err```.

```
let k = 21;
    
let x : Result<_, &str> = Ok("foo");
assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
    
let x : Result<&str, _> = Err("bar");
assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);
    
let x : Result<&str, _> = Err("barro");
assert_eq!(x.map_or_else(|e| e.len() + 1, |v| v.len()), 6);
```

---

### map_err()

Mapeia um ```Result<T, E>``` para um ```Result<T, F>``` aplicando a closure passada como argumento ao valor contido em ```Err```, deixando o valor ```Ok``` intocado.

Esta função pode ser usada para passar por um ```Result``` bem-sucedido enquanto lida com um erro. 

```
fn stringify(x: u32) -> String { format!("error code: {x}") }

let x: Result<u32, u32> = Ok(2);
assert_eq!(x.map_err(stringify), Ok(2));

let x: Result<u32, u32> = Err(13);
assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
```

---

### inspect()

Executa uma closure com uma referência ao valor contido no ```Ok```.

Retorna o valor original do ```Result```. 

```
    let x: u8 = "2"
        .parse::<u8>()
        .inspect(|x| println!("valor inicial: {x}"))
        .map(|x| x.pow(3))
        .expect("failed to parse number");

    println!("novo valor   : {}", x);
```

Execução:

```
valor inicial: 2
novo valor   : 8
```

---

### inspect_err()

Executa uma closure com uma referência ao valor contido no ```Err```.

Retorna o valor original do ```Result```. 

```
use std::{fs, io};

fn read() -> io::Result<String> {
    fs::read_to_string("address.txt")
        .inspect_err(|e| eprintln!("falha ao tentar ler o arquivo: {e}"))
}

fn main() {
    let _ = read();
}
```

Execução:

```
falha ao tentar ler o arquivo: O sistema não pode encontrar o arquivo especificado. (os error 2)
```

---

### as_deref()

Converte de ```Result<T, E>``` (ou ```&Result<T, E>```) para ```Result<&<T as Deref>::Target, &E>```.

Faz coerção da variante ```Ok``` do ```Result``` original via ```Deref``` e retorna o novo ```Result```.

```
let x: Result<String, u32> = Ok("hello".to_string());
let y: Result<&str, &u32> = Ok("hello");
assert_eq!(x.as_deref(), y);

let x: Result<String, u32> = Err(42);
let y: Result<&str, &u32> = Err(&42);
assert_eq!(x.as_deref(), y);
```

---

### as_deref_mut() 

Converte de ```Result<T, E>``` (ou ```&mut Result<T, E>```) para ```Result<&mut <T as DerefMut>::Target, &mut E>```.

Faz coerção da variante ```Ok``` do ```Result``` original via ```DerefMut``` e retorna o novo ```Result```.

```
let mut s = "HELLO".to_string();
let mut x: Result<String, u32> = Ok("hello".to_string());
let y: Result<&mut str, &mut u32> = Ok(&mut s);
assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);

let mut i = 42;
let mut x: Result<String, u32> = Err(42);
let y: Result<&mut str, &mut u32> = Err(&mut i);
assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
```

---

### iter()

Retorna um iterador sobre o valor possivelmente contido.

O iterador produz um valor se o ```Result``` for um ```Ok```, caso contrário, produz ```None```.

```
let x: Result<u32, &str> = Ok(7);
assert_eq!(x.iter().next(), Some(&7));

let x: Result<u32, &str> = Err("nothing!");
assert_eq!(x.iter().next(), None);
```

---

### iter_mut() 

Retorna um iterador mutável sobre o valor possivelmente contido.

O iterador produz um valor se o ```Result``` for um ```Ok```, caso contrário, produz ```None```.

```
let mut x: Result<u32, &str> = Ok(7);
match x.iter_mut().next() {
    Some(v) => *v = 40,
    None => {},
}
assert_eq!(x, Ok(40));

let mut x: Result<u32, &str> = Err("nothing!");
assert_eq!(x.iter_mut().next(), None);
```

---

### expect()

Retorna o valor embutido no ```Ok```, consumindo ```self```.

Como essa função pode causar pânico, seu uso é geralmente desencorajado. Em vez disso, prefira usar correspondência de padrões (```match```) e lidar com o caso ```Err``` explicitamente, ou chamar ```unwrap_or()```, ```unwrap_or_else()``` ou ```unwrap_or_default()```.

Se o valor for um ```Err```, lança um pânico com uma mensagem incluindo a mensagem passada como argumento e o conteúdo do ```Err```. 

```
let x: Result<u32, &str> = Err("emergency failure");
x.expect("Testing expect"); // panics with `Testing expect: emergency failure`
```

Recomenda-se que a mensagem passada como argumento seja usada para descrever o motivo pelo qual você espera que o ```Result``` seja ```Ok```. 

```
let path = std::env::var("IMPORTANT_PATH")
    .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
```

---

### unwrap()

Retorna o valor embutido no ```Ok```, consumindo ```self```.

Como essa função pode causar pânico, seu uso é geralmente desencorajado. Em vez disso, prefira usar correspondência de padrões (```match```) e lidar com o caso ```Err``` explicitamente, ou chamar ```unwrap_or()```, ```unwrap_or_else()``` ou ```unwrap_or_default()```.

Se o valor for um ```Err```, lança um pânico com uma mensagem fornecida pelo valor do ```Err```.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.unwrap(), 2);
```

```
let x: Result<u32, &str> = Err("emergency failure");
x.unwrap(); // panics with `emergency failure`
```

---

### unwrap_or_default()

Retorna o valor embutido no ```Ok``` ou o valor padrão para o tipo.

Consome ```self``` e, se for um ```Ok```, retorna o valor contido, caso contrário, se for um ```Err```, retorna o valor padrão para esse tipo.

```
let good_year_from_input = "1909";
let bad_year_from_input = "190blarg";
let good_year = good_year_from_input.parse().unwrap_or_default();
let bad_year = bad_year_from_input.parse().unwrap_or_default();

assert_eq!(1909, good_year);
assert_eq!(0, bad_year); // 0 é o valor default para inteiros
```

---

### expect_err()

Retorna o valor embutido no ```Err```, consumindo o valor ```self```.

Se o valor é um ```Ok```, gera pânico com uma mensagem que inclui a ```&str``` passada como argumento e o conteúdo do ```Ok```.

```
let x: Result<u32, &str> = Ok(10);
x.expect_err("Testing expect_err"); // panics with `Testing expect_err: 10`
```

---

### unwrap_err()

Retorna o valor embutido no ```Err```, consumindo o valor ```self```.

Se o valor é um ```Ok```, gera pânico com uma mensagem de pânico fornecida pelo valor do ```Ok```. 

```
let x: Result<u32, &str> = Ok(2);
x.unwrap_err(); // panics with `2`
```

```
let x: Result<u32, &str> = Err("emergency failure");
assert_eq!(x.unwrap_err(), "emergency failure");
```

---

### and()

Retorna ```res``` se o resultado for ```Ok```, caso contrário retorna o valor ```Err``` de self.

Argumentos passados para ```and``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```and_then```.

```
let x: Result<u32, &str> = Ok(2);
let y: Result<&str, &str> = Err("late error");
assert_eq!(x.and(y), Err("late error"));

let x: Result<u32, &str> = Err("early error");
let y: Result<&str, &str> = Ok("foo");
assert_eq!(x.and(y), Err("early error"));

let x: Result<u32, &str> = Err("not a 2");
let y: Result<&str, &str> = Err("late error");
assert_eq!(x.and(y), Err("not a 2"));

let x: Result<u32, &str> = Ok(2);
let y: Result<&str, &str> = Ok("different result type");
assert_eq!(x.and(y), Ok("different result type"));
```

---

### and_then()
 
Se for ```Ok```, chama a closure passada como argumento. Caso contrário retorna o valor ```Err``` de ```self```.

Esta função pode ser usada para controle de fluxo baseado em valores do  ```Result```.

```
fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
    x.checked_mul(x).map(|sq| sq.to_string()).ok_or("overflowed")
}

assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
assert_eq!(Err("not a number").and_then(sq_then_to_string), Err("not a number"));
```

Frequentemente usado para encadear operações falíveis que podem retornar ```Err```.

```
use std::{io::ErrorKind, path::Path};

// Note: on Windows "/" maps to "C:\"
let root_modified_time = Path::new("/").metadata().and_then(|md| md.modified());
assert!(root_modified_time.is_ok());

let should_fail = Path::new("/bad/path").metadata().and_then(|md| md.modified());
assert!(should_fail.is_err());
assert_eq!(should_fail.unwrap_err().kind(), ErrorKind::NotFound);
```

---

### or()

Retorna ```res``` se o resultado for ```Err```, caso contrário retorna o valor ```Ok``` de self.

Argumentos passados para ```or``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```or_else```. 

```
let x: Result<u32, &str> = Ok(2);
let y: Result<u32, &str> = Err("late error");
assert_eq!(x.or(y), Ok(2));

let x: Result<u32, &str> = Err("early error");
let y: Result<u32, &str> = Ok(2);
assert_eq!(x.or(y), Ok(2));

let x: Result<u32, &str> = Err("not a 2");
let y: Result<u32, &str> = Err("late error");
assert_eq!(x.or(y), Err("late error"));

let x: Result<u32, &str> = Ok(2);
let y: Result<u32, &str> = Ok(100);
assert_eq!(x.or(y), Ok(2));
```

---

### or_else()

Se for ```Err```, chama a closure passada como argumento com o valoor de ```Err```. Caso contrário retorna o valor ```Ok``` de self.

Esta função pode ser usada para controle de fluxo baseado em valores do ```Result```.

```
fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
fn err(x: u32) -> Result<u32, u32> { Err(x) }

assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
```

---

### unwrap_or()

Retorna o valor ```Ok``` contido ou o valor default fornecido.

Argumentos passados para ```unwrap_or``` são avaliados ativamente; se você estiver passando o resultado de uma chamada de função, é recomendável usar ```unwrap_or_else```. 

```
let default = 2;
let x: Result<u32, &str> = Ok(9);
assert_eq!(x.unwrap_or(default), 9);

let x: Result<u32, &str> = Err("error");
assert_eq!(x.unwrap_or(default), default);
```

---

### unwrap_or_else()

Retorna o valor ```Ok``` contido ou o calcula a partir da closure passada como argumento.

```
fn count(x: &str) -> usize { x.len() }

assert_eq!(Ok(2).unwrap_or_else(count), 2);
assert_eq!(Err("foo").unwrap_or_else(count), 3);
```

---

### unwrap_unchecked()

Retorna o valor ```Ok``` contido, consumindo ```self```, sem verificar se o valor não é um ```Err```.

Segurança: Chamar este método em um ```Err``` provoca um comportamento indefinido. 

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(unsafe { x.unwrap_unchecked() }, 2);
```

```
let x: Result<u32, &str> = Err("emergency failure");
unsafe { x.unwrap_unchecked(); } // Undefined behavior!
```

---

### unwrap_err_unchecked()

Retorna o valor ```Err``` contido, consumindo ```self```, sem verificar se o valor não é um ```Ok```.

Segurança: Chamar este método em um ```Ok``` provoca um comportamento indefinido. 

```
let x: Result<u32, &str> = Ok(2);
unsafe { x.unwrap_err_unchecked() }; // Undefined behavior!
```

```
let x: Result<u32, &str> = Err("emergency failure");
assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");
```

---

### copied()

Mapeia um ```Result<&T, E>``` para um ```Result<T, E>``` copiando o conteúdo da parte ```Ok```.

```
let val = 12;
let x: Result<&i32, i32> = Ok(&val);
assert_eq!(x, Ok(&12));
let copied = x.copied();
assert_eq!(copied, Ok(12));
```

---

### cloned()

Mapeia um ```Result<&T, E>``` para um ```Result<T, E>``` clonando o conteúdo da parte ```Ok```.

```
let val = 12;
let x: Result<&i32, i32> = Ok(&val);
assert_eq!(x, Ok(&12));
let cloned = x.cloned();
assert_eq!(cloned, Ok(12));
```

---

### copied()

Mapeia um ```Result<&mut T, E>``` para um ```Result<T, E>``` copiando o conteúdo da parte ```Ok```.

```
let mut val = 12;
let x: Result<&mut i32, i32> = Ok(&mut val);
assert_eq!(x, Ok(&mut 12));
let copied = x.copied();
assert_eq!(copied, Ok(12));
```

---

### cloned()

Mapeia um ```Result<&mut T, E>``` para um ```Result<T, E>``` clonando o conteúdo da parte ```Ok```.

```
let mut val = 12;
let x: Result<&mut i32, i32> = Ok(&mut val);
assert_eq!(x, Ok(&mut 12));
let cloned = x.cloned();
assert_eq!(cloned, Ok(12));
```

---

### transpose()

Transpõe de um ```Result``` com um ```Option``` para um ```Option``` com um ```Result```.

```Ok(None)``` será mapeado para ```None```. ```Ok(Some(_))``` e ```Err(_)``` serão mapeados para ```Some(Ok(_))``` e ```Some(Err(_)```).

```
#[derive(Debug, Eq, PartialEq)]
struct SomeErr;

let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
assert_eq!(x.transpose(), y);
```

---

## Referências

[Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html)

---

arataca89@gmail.com

Última atualização: 20241113
