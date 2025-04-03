#### arataca89

# Convertendo um tipo em outro com `into()`

Em Rust, a função `into()` é usada para conversões de tipo quando um tipo pode ser convertido em outro de maneira idiomática. Ela é parte da trait `Into<T>`, que é o inverso da trait `From<T>`.

### Para que serve `into()`?
- **Conversão de tipos**: Transforma um valor de um tipo em outro, desde que a implementação de `Into<T>` ou `From<T>` exista para esses tipos.
- **Conveniência**: Muitas bibliotecas do Rust implementam `Into` para facilitar conversões implícitas seguras.
- **Redução de boilerplate**: Evita a necessidade de chamar explicitamente construtores ou funções de conversão.

### Como funciona?
Se um tipo `A` implementa `Into<B>`, então você pode chamar `.into()` em uma instância de `A` para convertê-la em `B`. Isso geralmente depende da implementação da trait `From` (já que `Into` é automaticamente implementado quando `From` está presente).

### Exemplo:
```rust
// Exemplo 1: String -> Vec<u8>
let s = String::from("hello");
let bytes: Vec<u8> = s.into(); // Converte String em Vec<u8>

// Exemplo 2: &str -> String (usando Into)
let texto = "olá";
let string: String = texto.into(); // Converte &str em String

// Exemplo 3: Conversão personalizada com From/Into
struct Numero(i32);

impl From<i32> for Numero {
    fn from(valor: i32) -> Self {
        Numero(valor)
    }
}

let x = 42;
let num: Numero = x.into(); // Usa Into<Numero> (implementado automaticamente via From)
```

### Quando usar `into()`?
- Quando você quer uma conversão clara e segura entre tipos.
- Quando a legibilidade do código melhora em comparação com outras formas de conversão.
- Em funções que aceitam tipos genéricos com restrição `Into<T>` (muito comum em APIs de bibliotecas).

### Observações:
1. O compilador pode exigir anotação de tipo (como em `let x: TipoAlvo = valor.into()`) para inferir corretamente o destino da conversão.
2. `Into` é frequentemente usado em parâmetros de funções para aceitar múltiplos tipos (ex.: `fn foo<T: Into<String>>(s: T)`).

---

arataca89@gmail.com

Última atualização: 20250403