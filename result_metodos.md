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

[is_ok()](#is_ok())

---

### is_ok()



 


---

## O tipo Option

O tipo [Option](https://doc.rust-lang.org/std/option/enum.Option.html) é definido da biblioteca padrão.

```
enum Option<T> {
    None,
    Some(T),
}
```

---

## Referências

[Result](https://doc.rust-lang.org/std/result/enum.Result.html#)

---

arataca89@gmail.com

Última atualização: 20241106