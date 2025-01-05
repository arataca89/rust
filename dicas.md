#### arataca89

## Rust - dicas

[Desabilitar as mensagens de warning](#desabilitar-as-mensagens-de-warning)

[Converter um Vec&lt;char&gt; em uma String](#converter-um-vecchar-em-uma-string)

[Ler um arquivo texto para uma String](#ler-um-arquivo-texto-para-uma-string)

[Implementar Display para imprimir com ```{}```](#implementar-display-para-imprimir-com-)


---

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

# Ler um arquivo texto para uma String

```
use std::fs;
use std::io;
/********************************************************************
fs::read_to_string()
    Lê todo o conteúdo de um arquivo para uma String.
********************************************************************/
fn main(){
    let texto =  fs::read_to_string("lorem.txt");
    match texto {
        Ok(s)  => println!("{s}"),
        Err(e) => println!("Erro: {}", e),
    }
}
/********************************************************************
Se o arquivo 'lorem.txt' existir, seu conteudo será exibido,
senão a saída será:

Erro: O sistema não pode encontrar o arquivo especificado. (os error 2)
********************************************************************/
```

[std::read_to_string()](https://doc.rust-lang.org/beta/std/fs/fn.read_to_string.html)

---

# Implementar Display para imprimir com ```{}```


```
// Importa o módulo 'fmt'
use std::fmt;

// Estrutura que implementará 'fmt::Dispaly'.
struct Structure(i32);

// Para imprimir usando '{}', o tipo tem que implementar 'fmt::Display'
impl fmt::Display for Structure {
    // Esta trait requer o método 'fmt' com esta assinatura.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

---

arataca89@gmail.com

Última atualização: 20250105
