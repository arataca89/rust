# Métodos de Result

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

[>>>](#expect) expect(&str) - retorna o valor embutido no ```Ok```


---

### is_ok( )

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

### is_ok_and( )

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

### is_err( ) 

Retorna ```true``` se o ```Result``` é ```Err```.

```
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_err(), false);

let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_err(), true);
```

---

### is_err_and( )

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

### ok( ) 

Converte um ```Result<T,E>``` em um ```Option<T>```.

Converte ```self``` em um ```Option<T>```, consumindo ```self``` e descartando o erro, se houver.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.ok(), Some(2));

let x: Result<u32, &str> = Err("Nothing here");
assert_eq!(x.ok(), None);
```

---

### err( ) 

Converte um ```Result<T, E>``` em um ```Option<E>```.

Converte ```self``` para um ```Option<E>```, consumindo ```self``` e descartando o valor de sucesso, se houver. 

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.err(), None);

let x: Result<u32, &str> = Err("Nothing here");
assert_eq!(x.err(), Some("Nothing here"));
```

---

### as_ref( )

Converte de ```&Result<T, E>``` para ```Result<&T, &E>```.

Produz um novo ```Result```, contendo uma referência para o original, deixando o original no lugar.

```
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.as_ref(), Ok(&2));

let x: Result<u32, &str> = Err("Error");
assert_eq!(x.as_ref(), Err(&"Error"));
```

---

### as_mut( )

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

### map( )

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

### map_or( )

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

### map_or_else( )

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

### map_err( )

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

### inspect( )

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

### inspect_err( )

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

### as_deref( )

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

### as_deref_mut( ) 

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

### iter( )

Retorna um iterador sobre o valor possivelmente contido.

O iterador produz um valor se o ```Result``` for um ```Ok```, caso contrário, produz ```None```.

```
let x: Result<u32, &str> = Ok(7);
assert_eq!(x.iter().next(), Some(&7));

let x: Result<u32, &str> = Err("nothing!");
assert_eq!(x.iter().next(), None);
```

---

### iter_mut( ) 

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

### expect( )


asd


---

## Referências

[Result](https://doc.rust-lang.org/std/result/enum.Result.html#)

---

arataca89@gmail.com

Última atualização: 20241109
