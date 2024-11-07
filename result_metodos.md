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

[is_ok()](#is_ok)

[is_ok_and()](#is_ok_and)

[is_err()](#is_err)

[is_err_and()](#is_err_and)

[ok()](#ok)

[err()](#err)

[as_ref()](#as_ref)

[as_mut()](#as_mut)

[map()](#map)

[map_or()](#map_or)

[map_or_else()](#map_or_else)


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

Se o ```Result``` é um ```Ok```, aplica a closure passada como argumento ao valor de ```Ok```.

Se o ```Result``` é um ```Err```, retorna o valor default passado como argumento.

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

### asd

asd

---

## Referências

[Result](https://doc.rust-lang.org/std/result/enum.Result.html#)

---

arataca89@gmail.com

Última atualização: 20241107
