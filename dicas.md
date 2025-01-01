#### arataca89

## Rust - dicas

# Desabilitar as mensagens de warning
Insira a seguinte atributo no início do seu arquivo fonte
```
#![allow(warnings)]
```

Um atributo é um metadado geral e de forma livre que é interpretado de acordo com o nome, convenção, idioma e versão do compilador. Atributos internos, escritos com um ponto de exclamação (!) após a cerquilha (#), aplicam-se ao item dentro do qual o atributo é declarado. Atributos externos, escritos sem o ponto de exclamação após a cerquilha, aplicam-se à coisa que segue o atributo. 

[Atributos](https://doc.rust-lang.org/reference/attributes.html)

---

# Converter um Vec&lt;char&gt; em uma String

Se não for precisar mais do vetor pode usar ```into_inter()``` e ```collect()```.

```
fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let s: String = v.into_iter().collect();
    println!("{}", s);
    //println!("{:?}", v);// ERRO. v foi movido devido a into_iter()
}
```

Se for usar o vetor depois pode usar ```iter()``` e ```collect()```.

```
fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let s: String = v.iter().collect();
    println!("{}", s);
    println!("{:?}", v);
}
```


Fonte: [https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string](https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string)

---

arataca89@gmail.com

Última atualização: 20250101
