#### arataca89

# Linguagem Rust - assert_eq!


```assert_eq!``` é uma macro usada em testes para afirmar que duas expressões são iguais. Ela usa a trait ```PartialEq``` para executar seu trabalho. Para saber mais sobre traits veja: [Traits](https://github.com/arataca89/rust/tree/main/traits) e [Traits deriváveis](derivable_traits.md#arataca89).

```
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => { ... };
    ($left:expr, $right:expr, $($arg:tt)+) => { ... };
}
``` 

Em caso de pânico, esta macro imprimirá os valores das duas expressões , o que ajuda a identificar o erro no código sendo testado.

Assim como o ```assert!```, esta macro tem uma segunda forma, onde uma mensagem de pânico personalizada pode ser fornecida como o terceiro argumento.

```
fn adicionar_um(x: i32) -> i32 {
    x + 1
}

#[test]
fn adicionar_um_test(){
    assert_eq!(adicionar_um(3), 4);
    assert_eq!(adicionar_um(2), 4, "Valor esperado em left: 4; Valor calculado: {}.", adicionar_um(2));
}

fn main() {
    println!("asdfg");
}
```

Execução do teste:

```
C:\Users\arataca89\Documents\rust\packages\estudo_assert_eq>cargo test
   Compiling estudo_assert_eq v0.1.0 (C:\Users\arataca89\Documents\rust\packages\estudo_assert_eq)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running unittests src/main.rs (target\debug\deps\estudo_assert_eq-37eb7377b75c78b7.exe)

running 1 test
test adicionar_um_test ... FAILED

failures:

---- adicionar_um_test stdout ----
thread 'adicionar_um_test' panicked at src/main.rs:8:5:
assertion `left == right` failed: Valor esperado em left: 4; Valor calculado: 3.
  left: 3
 right: 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adicionar_um_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `--bin estudo_assert_eq`
```

Basta comentar o segundo ```assert_eq!``` ...

```
.....
#[test]
fn adicionar_um_test(){
    assert_eq!(adicionar_um(3), 4);
    //assert_eq!(adicionar_um(2), 4, "Valor esperado em left: 4; Valor calculado: {}.", adicionar_um(2));
}
.....
```

... e o teste terá sucesso.

```
C:\Users\arataca89\Documents\rust\packages\estudo_assert_eq>cargo test
   Compiling estudo_assert_eq v0.1.0 (C:\Users\arataca89\Documents\rust\packages\estudo_assert_eq)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/main.rs (target\debug\deps\estudo_assert_eq-37eb7377b75c78b7.exe)

running 1 test
test adicionar_um_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


## Referências

[std::assert_eq](https://doc.rust-lang.org/std/macro.assert_eq.html)

---

arataca89@gmail.com

Última atualização: 20241228
