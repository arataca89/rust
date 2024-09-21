# Linguagem Rust - ownership

Ownership (propriedade) é o recurso do Rust que tem mais implicações profundas no resto da linguagem. A propriedade permite que o Rust possa garantir segurança de memória sem precisar de um coletor de lixo (garbage collection), então é importante entender como ownership funciona.

[1. Entendendo o que é ownership](#1-Entendendo-o-que-é-ownership)

[2. Movendo dados na memória](#2-Movendo-dados-na-memória)


---

## 1. Entendendo o que é ownership

Ownership (propriedade) consiste em um conjunto de regras que governam como um programa Rust gerencia a memória. Todos os programas precisam gerenciar a maneira como usam a memória de um computador durante a execução. Algumas linguagens têm garbage collection (coleta de lixo) que procura regularmente por memória que não é mais usada enquanto o programa é executado; em outras linguagens, o programador deve alocar e liberar explicitamente a memória. Rust usa uma terceira abordagem: a memória é gerenciada por meio de um sistema de propriedade com um conjunto de regras que o compilador verifica. Se alguma das regras for violada, o programa não será compilado. Nenhum dos recursos de propriedade deixará seu programa lento enquanto ele estiver em execução.

Regras da propriedade(ownership) em Rust:

* Cada valor tem um proprietário;
* Só pode haver um proprietário de cada vez;
* Quando o proprietário sai do escopo o valor é destruído.

Os tipos de dados mais simples (inteiros, decimais, booleanos, caracteres) têm seu tamanho conhecido e podem ser armazenados na região de memória da pilha. Estes dados podem ser facilmente  inseridos e removidos da pilha e podem ser copiados rapidamente quando necessário.

Tipos de dados mais complexos (strings, vetores, objetos criados dinamicamente, etc...) não têm seu tamanho conhecido em tempo de compilação e são armazenados no heap, o tipo String é um exemplo. O armazenamento de dados no heap exige que a região de memória alocada para o programa seja devolvida ao sistema quando não estiver mais sendo usada. Além disso, estes tipos de dados têm um custo maior para serem copiados e muitas vezes é preferível que sejam movidos para outras posições de memória e não copiados.

Literais string, valores envoltos em aspas duplas, como por exemplo "Calango", são usados quando normalmente já conhecemos a string a ser processada. Este tipo de dado é conveniente, mas não é adequado para todas as situações onde queremos usar texto. Um dos motivos é porque eles são imutáveis. Outro motivo é que nem todo valor de string pode ser conhecido quando escrevemos nosso código: por exemplo, e se quisermos receber entrada do usuário e armazená-la? Para essas situações, Rust tem o tipo String. O tipo de dados String é armazenado na memória heap e, assim, é capaz de armazenar uma quantidade de texto que é desconhecida para nós em tempo de compilação. 

Um objeto String pode ser criado a partir de um literal string pelo uso da função ```from()```:

```
let s = String::from("hello");
```

O tipo String pode ser mutável:

```
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() adiciona um literal a String

    println!("{s}"); // hello, world!
```


Quando usamos literais string, conhecemos o valor do texto em tempo de compilação, então o texto é codificado diretamente no executável final. É por isso que as literais string são rápidas e eficientes. Mas essa rapidez e eficiência vêm da imutabilidade da literal string. Mas ao criar programas reais precisamos de texto imutável e texto mutável, ou melhor, precisamos de dados imutáveis e mutáveis, sejam eles de qualquer tipo.

Quando usamos o tipo String, ou qualquer outro tipo de dados mutável,  precisamos colocar dados na memória heap pois não sabemos a quantidade de memória que será necessária em tempo de compilação; e para usar a memória heap precisamos:

* Solicitar esta memória em tempo de execução;
* Uma maneira de retornar essa memória ao sistema quando não precisarmos mais dela.

A solicitação de memória ao sistema normalmente é feita através de funções fornecidas pela linguagem de programação. No caso de usarmos String em Rust podemos usar ```String::from()```. Essa abordagem é praticamente  universal nas linguagens de programação. 

No entanto, o retorno da memória não mais necessária não é tão simples assim. Em linguagens com um coletor de lixo, o famoso garbage collector (GC), o GC rastreia e limpa a memória que não está mais sendo usada, e não precisamos pensar sobre isso. Nas linguagens que não têm GC, é responsabilidade do programador identificar quando a memória não está mais sendo usada e chamar o código para liberá-la explicitamente, assim como fizemos para solicitá-la. Fazer isso corretamente tem sido historicamente um problema de programação difícil. Se esquecermos, desperdiçaremos memória. Se fizermos isso muito cedo, teremos uma variável inválida. Se fizermos isso duas vezes, isso também é um bug. Precisamos parear exatamente uma alocação de memória com sua liberação. Ou seja, para cada malloc() um free(); para cada new() um delete().

Rust adota uma abordagem diferente: a memória é devolvida automaticamente quando a variável que a possui sai do escopo. 

Quand a variável sai do escopo Rust chama um método especial da biblioteca chamado [drop()](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop). Este método executa o destrutor do objeto implicitamente.

Esta forma de gerenciar a memória heap tem um impacto profundo na forma como o código Rust é escrito. Pode parecer simples agora, mas o comportamento do código pode ser inesperado em situações mais complicadas quando queremos que várias variáveis usem os dados alocados no heap. 

## 2. Movendo dados na memória

Em Rust, múltiplas variáveis podem interagir com os mesmos dados de maneiras diferentes. Vamos ver um exemplo usando um inteiro. 

```
    let x = 5;
    let y = x;
```

Provavelmente podemos imaginar o que este trecho de código faz: "vincule o valor 5 a x; então faça uma cópia do valor em x e vincule-o a y." Agora temos duas variáveis, x e y, e ambas iguais a 5. Isso é realmente o que está acontecendo, porque inteiros são valores simples com um tamanho fixo conhecido, e esses dois valores 5 são armazenados na região de memória da pilha.

Vamos dar uma olhada no mesmto tipo de código, mas usando um tipo complexo, por exemplo, String.

```
    let s1 = String::from("hello");
    let s2 = s1;
```

Isso parece muito semelhante, então poderíamos assumir que a forma como funciona seria a mesma: ou seja, a segunda linha faria uma cópia do valor em s1 e o vincularia a s2. Mas isso não é bem o que acontece. 

Observe na figura abaixo como um objeto String é armazenado. Um objeto String é composto por três partes: um ponteiro para a memória que contém o conteúdo da string, um comprimento e uma capacidade. Este grupo de dados é armazenado na pilha. À direita está a memória no heap que contém o conteúdo. 

 ![Campos de um objeto String](./images/ownership1.svg)
 
<img alt="Campos de um objeto String" src="images/ownership1.svg" class="center" style="width: 50%;">
 
 

asd


---
## Referências

[Capítulo 4 do Livro](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)



---

arataca89@gmail.com

Última atualização: 20240921
