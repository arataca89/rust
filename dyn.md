#### arataca89

# Linguagem Rust - o que significa a palavra-chave `dyn`

Em Rust, a palavra-chave `dyn` é usada para indicar que um tipo é um **trait object**, ou seja, um tipo dinâmico que implementa um determinado trait. Traits em Rust são semelhantes a interfaces em outras linguagens, definindo um conjunto de métodos que um tipo deve implementar.

Quando você usa `dyn`, está criando um tipo que pode ser qualquer tipo que implemente o trait especificado, permitindo **despacho dinâmico** (dynamic dispatch). Isso é útil quando você precisa de flexibilidade para trabalhar com diferentes tipos em tempo de execução, mas ainda quer garantir que esses tipos implementem um determinado comportamento (trait).

### Exemplo de uso:

```rust
trait Animal {
    fn fazer_som(&self);
}

struct Cachorro;
struct Gato;

impl Animal for Cachorro {
    fn fazer_som(&self) {
        println!("Au Au!");
    }
}

impl Animal for Gato {
    fn fazer_som(&self) {
        println!("Miau!");
    }
}

fn main() {
    let animais: Vec<Box<dyn Animal>> = vec![
        Box::new(Cachorro),
        Box::new(Gato),
    ];

    for animal in animais {
        animal.fazer_som();
    }
}
```

### Explicação:
1. **`dyn Animal`**: Indica que o tipo é um trait object que implementa o trait `Animal`.
2. **`Box<dyn Animal>`**: Usamos `Box` para alocar o trait object no heap, já que o tamanho do trait object não é conhecido em tempo de compilação.
3. **Despacho dinâmico**: Em tempo de execução, o Rust determina qual método `fazer_som` chamar, dependendo do tipo real (`Cachorro` ou `Gato`).

### Quando usar `dyn`:
- Quando você precisa de flexibilidade para trabalhar com diferentes tipos em tempo de execução.
- Quando o tamanho do tipo não é conhecido em tempo de compilação (por exemplo, em coleções heterogêneas).

### Alternativa:
Se você sabe os tipos em tempo de compilação, pode usar **genéricos** com traits, que têm despacho estático e são geralmente mais eficientes.

```rust
fn fazer_som<T: Animal>(animal: &T) {
    animal.fazer_som();
}
```

Em resumo, `dyn` é uma ferramenta poderosa para trabalhar com polimorfismo dinâmico em Rust.

---

## Referências

[DeepSeek](https://www.deepseek.com) - IA

---

arataca89@gmail.com

Última atualização: 20250223