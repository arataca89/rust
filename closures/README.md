# Linguagem Rust - closures

Closures em Rust são funções anônimas que você pode salvar numa variável ou passar como argumento para outras funções. Você pode criar uma closure em um lugar e chamá-la em outro para avaliá-la em outro contexto. Diferente das funções tradicionais, closures podem capturar valores do escopo onde são definidas. Os recursos das closures permitem reutilização de código e customização de comportamento.

[1. Capturando o ambiente com closures](#1-Capturando-o-ambiente-com-closures)


---

## 1. Capturando o ambiente com closures

Closures podem capturar valores do ambiente onde foram definidas para uso posterior.

Para exemplificar o uso deste recurso foi criado um cenário onde uma certa empresa que comercializa camisetas faz a seguinte promoção: será sorteada uma camiseta exclusiva entre as pessoas que se inscreverem na lista para receber emails promocionais da empresa. No momento da inscrição a pessoa pode também declarar sua cor favorita. Se a pessoa sorteada tiver declarado sua cor favorita, ganhará a camiseta dessa cor. Senão ganhará uma camiseta da cor que a empresa mais tenha no momento.

Há muitas maneiras de implementar isso. Para este exemplo, usaremos uma enumeração chamada ```ShirtColor``` que tem as variantes ```Red``` e ```Blue```. Representamos o estoque da empresa com uma estrutura chamada ```Inventory``` que tem um campo chamado ```shirts``` que contém um ```Vec\<ShirtColor\>``` representando as cores das camisas atualmente em estoque. O método ```giveaway()``` definido em ```Inventory``` obtém a preferência de cor de camisa do ganhador da camisa grátis e retorna a cor de camisa que a pessoa receberá.

```
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```


asd

---
## Referências

[capítulo 13 do "Livro"](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

arataca89@gmail.com

Última atualização: 20240928
