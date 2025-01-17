#### arataca89

# Linguagem Rust - Padrões e correspondência de padrões

Padrões são uma sintaxe especial em Rust para combinar com a estrutura de tipos, tanto complexos quanto simples. Usar padrões em conjunto com expressões `match` e outras construções lhe dá mais controle sobre o fluxo do programa. Um padrão consiste em alguma combinação de: 

* Literais;
* Arrays, enums, structs ou tuplas desestruturadas;
* Variáveis;
* Curingas(wildcards);
* Placeholders. 

Alguns padrões de exemplo incluem **x**, **(a, 3)** e **Some(Color::Red)**. Nos contextos em que os padrões são válidos, esses componentes descrevem a forma dos dados. Nosso programa então compara os valores com os padrões para determinar se eles têm a forma correta de dados para continuar executando um determinado trecho de código. 

[Onde os padrões podem ser usados](#onde-os-padrões-podem-ser-usados)

- [match](#match)
- [if let](#if-let)
- [while let](#while-let)
- [for](#for)
- [let](#let)
- [Parâmetros de função](#parâmetros-de-função)
	
[Refutabilidade: se um padrão pode falhar na correspondência](#refutabilidade-se-um-padr%C3%A3o-pode-falhar-na-correspond%C3%AAncia)

[Sintaxe dos padrões](#sintaxe-dos-padrões)

- [Literais](#literais)
- [Variáveis](#vari%C3%A1veis)
- [Casando com vários padrões em um braço de match](#casando-com-v%C3%A1rios-padr%C3%B5es-em-um-bra%C3%A7o-de-match)
- [Padrão com intervalo de valores](#padr%C3%A3o-com-intervalo-de-valores)


[Desestruturando para separar valores](#desestruturando-para-separar-valores)

- [Desestruturando structs](#desestruturando-structs)
- [Desestruturando enums](#desestruturando-enums)
- [Desestruturando structs e enums aninhadas](#desestruturando-structs-e-enums-aninhadas)
- [Desestruturando structs e tuplas](#desestruturando-structs-e-tuplas)

[Ignorando valores em um padrão](#ignorando-valores-em-um-padr%C3%A3o)

- [Ignorando um valor inteiro com _ (sublinhado)](#ignorando-um-valor-inteiro-com-_)
- [Ignorando partes de um valor com um _ (sublinhado) aninhado](#ignorando-partes-de-um-valor-com-um-_-aninhado)
- [Ignorando uma variável não utilizada iniciando seu nome com _ (sublinhado)](#ignorando-uma-vari%C3%A1vel-n%C3%A3o-utilizada-iniciando-seu-nome-com-_)
- [Ignorando partes restantes de um valor com .. (ponto ponto)](#ignorando-partes-restantes-de-um-valor-com-)

[match guard conditional](#match-guard-conditional)

[Operador @](#operador-)

---

## Onde os padrões podem ser usados

#### `match`

Sintaxe da instrução `match`:

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Por exemplo, aqui está uma expressão `match` que analisa um valor `Option<i32>` na variável **x**:

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Os padrões nesta expressão são o `None` e `Some(i)` à esquerda de cada seta.

Um requisito para expressões `match` é que elas precisam ser exaustivas no sentido de que todas as possibilidades para o valor na expressão de correspondência devem ser verificadas. Uma maneira de garantir que você cobriu todas as possibilidades é ter, no último braço, um padrão que atenda todas as outras possibilidades que não foram atendidas nos braços anteriores.

O padrão `_` (sublinhado) corresponde a qualquer coisa, mas não está vinculado a nenhuma variável, por isso é frequentemente usado no último braço de `match`. O padrão `_` pode ser útil quando você deseja ignorar qualquer valor não especificado, por exemplo. 

#### `if let`
 
A sintaxe `if let` permite combinar `if` e `let` em uma maneira menos verbosa de lidar com valores que correspondem a um padrão, ignorando o restante. Considere o código abaixo que corresponde a um valor `Option<u8>` na variável **config_max**, mas só deseja executar código se o valor for a variante `Some`.

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("O máximo está configurado para ser {max}"),
        _ => (),
    }
``` 

Se o valor for `Some`, imprimimos o valor na variante `Some` vinculando o valor à variável **max**. Não queremos fazer nada com o valor `None`. Para satisfazer a expressão `match`, temos que adicionar `_ => ()` após processar apenas uma variante.

Em vez disso, poderíamos escrever isso de uma forma mais curta usando `if let`. 

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("O máximo está configurado para ser {max}");
    }

```

Então, `if let` é uma maneira mais curta de escrever o equivalente a um `match` com apenas um caso. Opcionalmente, `if let` pode ter um `else` contendo código para executar se o padrão no `if let` não corresponder. 

O código abaixo mostra que também é possível misturar e combinar expressões `if let`, `else if` e `else if let`. Fazer isso nos dá mais flexibilidade do que uma expressão `match` na qual podemos expressar apenas um valor para comparar com os padrões. Além disso, o Rust não exige que as condições em uma série de braços `if let`, `else if`, `else if let` estejam relacionadas entre si. 

Este exemplo determina qual cor usar para o seu plano de fundo com base em uma série de verificações para várias condições. Para este exemplo, criamos variáveis com valores escritos diretamente no código (hardcoded) que um programa real pode receber de entradas do usuário.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Usando sua cor favorita, {color}, como background");
    } else if is_tuesday {
        println!("Terça-feira é o dia verde!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Usando amarelo como cor de background");
        } else {
            println!("Usando laranja como cor de background");
        }
    } else {
        println!("Usando azul como cor de background");
    }
}
```

Se o usuário especificar uma cor favorita, essa cor será usada como fundo. Se nenhuma cor favorita for especificada e hoje for terça-feira, a cor de fundo será verde. Caso contrário, se o usuário especificar sua idade como uma string e pudermos analisá-la como um número com sucesso, a cor será amarela ou laranja, dependendo do valor do número. Se nenhuma dessas condições se aplicar, a cor de fundo será azul. 

Você pode ver que `if let` também pode introduzir variáveis sombreadas da mesma forma que os braços de `match` podem: a linha `if let Ok(age) = age` introduz uma nova variável `age` sombreada que contém o valor dentro da variante `Ok`. Isso significa que precisamos colocar a condição `if age > 30` dentro desse bloco: não podemos combinar essas duas condições em `if let Ok(age) = age && age > 30`. O `age` sombreado que queremos comparar com `30` não é válido até que o novo escopo comece com a chave. 

A desvantagem de usar expressões `if let` é que o compilador não exige a verificação de todas as possibilidades, enquanto que com expressões `match` ele exige. Se omitirmos o último bloco `else` e, portanto, perdermos o tratamento de alguns casos, o compilador não nos alertará para o possível erro lógico. 

#### `while let`

Semelhante à construção `if let`, o loop condicional `while let` permite que um loop `while` seja executado enquanto um padrão continuar a corresponder. No exemplo abaixo, codificamos um loop `while let` que usa um vetor como uma pilha e imprime os valores do vetor na ordem oposta em que foram inseridos.

```rust
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
```

Este exemplo imprime 3, 2 e então 1. O método `pop()` remove o último elemento do vetor e retorna `Some(valor)`. Se o vetor estiver vazio, `pop()` retorna `None`. O loop `while` continua executando o código em seu bloco enquanto `pop()` retornar `Some`. Quando `pop()` retorna `None`, o loop para. Podemos usar `while let` para remover todos os elementos da nossa pilha. 

#### `for`

Em um loop `for`, o valor que segue diretamente a palavra-chave `for` é um padrão. Por exemplo, em `for x in y`, o `x` é o padrão. O exemplo abaixo demonstra como usar um padrão em um loop `for` para desestruturar, ou separar, uma tupla como parte do loop `for`.

```rust
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

```

Este código imprimirá o seguinte:

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/patterns`
a is at index 0
b is at index 1
c is at index 2
```

Adaptamos um iterador usando o método `enumerate()` para que ele produza um valor e o índice desse valor, colocados em uma tupla. O primeiro valor produzido é a tupla `(0, 'a')`. Quando esse valor é correspondido ao padrão `(index, value)`, índice será `0` e valor será `'a'`, imprimindo a primeira linha da saída. 

#### `let`

Padrões podem ser usados com  `match` e `if let` e com somente `let` também. Por exemplo, considere esta atribuição de variável direta com `let`:

```rust
let x = 5;
```

Toda vez que você usa uma declaração `let` como esta, você está usando padrões, embora você possa não perceber. Mais formalmente, uma declaração `let` se parece com isso:

```rust
let PATTERN = EXPRESSION;
``` 
Em declarações como `let x = 5;` com um nome de variável no slot `PATTERN`, o nome da variável é apenas uma forma particularmente simples de um padrão. Rust compara a expressão com o padrão e atribui quaisquer nomes que encontrar. Então, no exemplo `let x = 5;`, `x` é um padrão que significa "vincule o que corresponde aqui à variável x". Como o nome `x` é o padrão inteiro, esse padrão efetivamente significa "vincule tudo à variável x, seja qual for o valor".

Para ver o aspecto de correspondência de padrões do `let` mais claramente, considere o trecho de código abaixo, que usa um padrão com `let` para desmontar uma tupla.

```rust
    let (x, y, z) = (1, 2, 3);
```

Aqui, nós comparamos uma tupla com um padrão. Rust compara o valor `(1, 2, 3)` com o padrão `(x, y, z)` e vê que o valor corresponde ao padrão, então Rust vincula `1` a `x`, `2` a `y` e `3` a `z`.

Se o número de elementos no padrão não corresponder ao número de elementos na tupla, o tipo geral não corresponderá e obteremos um erro do compilador. Por exemplo, abaixo mostra uma tentativa de desestruturar uma tupla com três elementos em duas variáveis, o que não funcionará.

```rust
    let (x, y) = (1, 2, 3);
```

Tentar compilar este código resulta no erro `mismatched types` (tipos incompatíveis):

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
  |         |
  |         expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

Para corrigir o erro, poderíamos ignorar um ou mais dos valores na tupla usando `_` (sublinhado) ou `..` (ponto ponto), como você verá na seção [Ignorando valores em um padrão](#ignorando-valores-em-um-padrão). Se o problema é que temos muitas variáveis no padrão, a solução é fazer os tipos corresponderem removendo variáveis para que o número de variáveis seja igual ao número de elementos na tupla. 

#### Parâmetros de função

Os parâmetros da função também podem ser padrões. O código abaixo, declara uma função chamada `foo()` que recebe um parâmetro chamado `x` do tipo `i32`.

```rust
fn foo(x: i32) {
    // -- codigo omitido --
}
``` 

A parte `x` é um padrão. Como fizemos com `let`, poderíamos combinar uma tupla nos argumentos de uma função com o padrão.

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

Este código imprime `Current location: (3, 5)`. Os valores `&(3, 5)` correspondem ao padrão `&(x, y)`, então `x` é o valor `3` e `y` é o valor `5`.

Também podemos usar padrões em listas de parâmetros de closures da mesma forma que em listas de parâmetros de função, porque closures são semelhantes a funções.

Neste ponto, você viu várias maneiras de usar padrões, mas os padrões não funcionam da mesma forma em todos os lugares em que podemos usá-los. Em alguns lugares, os padrões devem ser irrefutáveis; em outras circunstâncias, eles podem ser refutáveis. Discutiremos esses dois conceitos a seguir.

---

## Refutabilidade: se um padrão pode falhar na correspondência 

Padrões vêm em duas formas: refutáveis e irrefutáveis. Padrões que corresponderão a qualquer valor possível passado são irrefutáveis. Um exemplo seria `x` na declaração `let x = 5`; porque `x` corresponde a qualquer coisa e, portanto, não pode deixar de corresponder. Padrões que podem falhar em corresponder a algum valor possível são refutáveis. Um exemplo seria `Some(x)` na expressão `if let Some(x) = a_value` porque se o valor na variável `a_value` for `None` em vez de `Some`, o padrão `Some(x)` não corresponderá.

Parâmetros de função, instruções `let` e laços `for` só podem aceitar padrões irrefutáveis, porque o programa não pode fazer nada significativo quando os valores não correspondem. As expressões `if let` e `while let` aceitam padrões refutáveis e irrefutáveis, mas o compilador avisa contra padrões irrefutáveis porque, por definição, eles são destinados a lidar com possíveis falhas: a funcionalidade de uma condicional está em sua capacidade de executar de forma diferente dependendo do sucesso ou falha.

Em geral, você não deve se preocupar com a distinção entre padrões refutáveis e irrefutáveis; no entanto, você precisa estar familiarizado com o conceito de refutabilidade para poder responder quando ver isso em uma mensagem de erro. Nesses casos, você precisará alterar o padrão ou a construção que está usando o padrão, dependendo do comportamento pretendido do código. 

Vamos ver um exemplo do que acontece quando tentamos usar um padrão refutável onde Rust exige um padrão irrefutável e vice-versa. O código abaixo mostra uma declaração `let` e o padrão que especificamos, `Some(x)`, é um padrão refutável. Como você pode esperar, este código não compilará.

```rust
    let Some(x) = some_option_value;
``` 

Se `some_option_value` fosse um valor `None`, ele falharia ao combinar com o padrão `Some(x)`, significando que o padrão é refutável. No entanto, a declaração `let` só pode aceitar um padrão irrefutável porque não há nada válido que o código possa fazer com um valor `None`. Em tempo de compilação, Rust reclamará que tentamos usar um padrão refutável onde um padrão irrefutável é necessário:

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
  = note: the matched value is of type `Option<i32>`
help: you might want to use `let else` to handle the variant that isn't matched
  |
3 |     let Some(x) = some_option_value else { todo!() };
  |                                     ++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
``` 

Porque não cobrimos (e não poderíamos cobrir!) todos os valores válidos com o padrão `Some(x)`, Rust gera um erro de compilação com razão.

Se tivermos um padrão refutável onde um padrão irrefutável é necessário, podemos corrigi-lo alterando o código que usa o padrão: em vez de usar `let`, podemos usar `if let`. Então, se o padrão não corresponder, o código simplesmente ignorará o código entre as chaves, dando-lhe uma maneira de continuar validamente. Abaixo temos como corrigir o código anterior.

```rust
    if let Some(x) = some_option_value {
        println!("{x}");
    }
``` 

Damos ao código uma saída! Este código é perfeitamente válido agora. No entanto, se dermos a `if let` um padrão irrefutável (um padrão que sempre corresponderá), como `x`, como mostrado abaixo, o compilador emitirá um aviso.

```rust
    if let x = 5 {
        println!("{x}");
    };
```

Rust reclama que não faz sentido usar `if let` com um padrão irrefutável:

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
warning: irrefutable `if let` pattern
 --> src/main.rs:2:8
  |
2 |     if let x = 5 {
  |        ^^^^^^^^^
  |
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`
  = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: `patterns` (bin "patterns") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/patterns`
5
```

Por esse motivo, as opções de um `match` devem usar padrões refutáveis, exceto pela última, que deve corresponder a quaisquer valores restantes com um padrão irrefutável. Rust permite que usemos um padrão irrefutável em um `match` com apenas um braço, mas essa sintaxe não é particularmente útil e pode ser substituída por uma instrução `let` mais simples.

---

## Sintaxe dos padrões

### Literais

Você pode combinar padrões com literais diretamente. O código a seguir fornece alguns exemplos:

```rust
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

Este código imprime `one` porque o valor em `x` é `1`. Esta sintaxe é útil quando você quer que seu código tome uma ação se ele receber um valor concreto particular. 

### Variáveis

Variáveis são padrões irrefutáveis ​​que correspondem a qualquer valor. No entanto, há uma complicação quando você usa variáveis em um `match`. Como um `match` inicia um novo escopo, as variáveis ​​declaradas como parte de um padrão dentro da expressão `match` sombrearão aquelas com o mesmo nome fora da construção `match`, como é o caso com todas as variáveis. No exemplo abaixo, declaramos uma variável chamada `x` com o valor `Some(5)` e uma variável `y` com o valor `10`. Em seguida, criamos uma expressão `match` para um valor `x`. Observe os padrões nos braços do `match` e o `println!` no final.

```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("x vale 50"),
        Some(y) => println!("Combinou com y = {y}"),
        _ => println!("Caso default, x = {x:?}"),
    }

    println!("ao final temos: x = {x:?}, y = {y}");
```

Vamos analisar o que acontece quando este `match` é executado. O padrão no primeiro braço não corresponde ao valor definido de `x`, então o código continua.

O padrão no segundo braço introduz uma nova variável chamada `y` que corresponderá a qualquer valor dentro de um `Some`. Como estamos em um novo escopo dentro da expressão `match`, esta é uma nova variável `y`, não a `y` que declaramos no início com o valor 10. Esta nova variável `y` corresponderá a qualquer valor dentro de um `Some`, que é o que temos em `x`. Portanto, esta nova `y` se liga ao valor interno do `Some` em `x`. Esse valor é 5, então a expressão para esse braço é executada e imprime "Combinou com y = 5".

Se `x` tivesse sido um valor `None` em vez de `Some(5)`, os padrões nos dois primeiros braços não teriam correspondido, então o valor teria correspondido ao sublinhado. Não introduzimos a variável `x` no padrão do braço sublinhado, então o `x` na expressão ainda é o `x` externo que não foi sombreado. Neste caso hipotético, a correspondência imprimiria "Caso default, x = None". 
 
Quando a expressão `match` é concluída, seu escopo termina, assim como o escopo do `y` interno. O último `println!` produz "ao final temos: x = Some(5), y = 10". 

Para criar uma expressão `match` que compare os valores de `x` e `y` externos, em vez de introduzir uma variável sombreada, precisaríamos usar um [match guard conditional](#match-guard-conditional) que veremos mais abaixo.

### Casando com vários padrões em um braço de match

Você pode combinar vários padrões usando a sintaxe `|` (barra vertical), que é o operador `or` (ou) para padrões em um `match`. Por exemplo, no código a seguir combinamos o valor de `x` com os braços de `match`, o primeiro dos quais tem uma opção `or`, o que significa que se o valor de `x` corresponder a qualquer um dos valores naquele braço, o código daquele braço será executado:

```rust
    let x = 1;

    match x {
        1 | 2 => println!("um ou dois"),
        3 => println!("três"),
        _ => println!("qualquer coisa"),
    }
```

Este código imprime "um ou dois".

### Padrão com intervalo de valores

A sintaxe `..=` (ponto, ponto, igual) permite que se faça a correspondência com um intervalo inclusivo de valores. No código a seguir, quando um padrão corresponde a qualquer um dos valores dentro do intervalo fornecido, esse braço será executado:

```rust
    let x = 5;

    match x {
        1..=5 => println!("de um até cinco"),
        _ => println!("qualquer coisa"),
    }
```

Se `x` for 1, 2, 3, 4 ou 5, o primeiro braço corresponderá. Essa sintaxe é mais conveniente para vários valores correspondentes do que usar o operador `|` para expressar a mesma ideia; se usássemos `|`, teríamos que especificar `1 | 2 | 3 | 4 | 5`. Especificar um intervalo é muito mais curto, especialmente se quisermos corresponder, digamos, a qualquer número entre 1 e 1.000! 

O compilador verifica se o intervalo não está vazio em tempo de compilação, e porque os únicos tipos para os quais o Rust pode dizer se um intervalo está vazio ou não são `char` e valores numéricos, os intervalos só são permitidos com valores numéricos ou `char`.

Aqui está um exemplo usando intervalos de valores `char`:

```rust
    let x = 'c';

    match x {
        'a'..='j' => println!("primeiras letras ASCII"),
        'k'..='z' => println!("últimas letras ASCII"),
        _ => println!("qualquer coisa"),
    }
```

---

## Desestruturando para separar valores

Também podemos usar padrões para desestruturar structs, enums e tuplas para usar diferentes partes desses valores. Vamos percorrer cada valor. 

### Desestruturando structs

Abaixo temos uma estrutura `Point` com dois campos, `x` e `y`, que podemos separar usando um padrão com uma declaração `let`.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Este código cria as variáveis `​​a` e `b` que correspondem aos valores dos campos `x` e `y` da estrutura `p`. Este exemplo mostra que os nomes das variáveis ​​no padrão não precisam corresponder aos nomes dos campos da estrutura. No entanto, é comum corresponder os nomes das variáveis ​​aos nomes dos campos para facilitar a lembrança de quais variáveis ​​vieram de quais campos. Devido a esse uso comum, e porque escrever

```
let Point { x: x, y: y } = p;
```

contém muita duplicação, Rust tem uma abreviação para padrões que correspondem aos campos da estrutura: você só precisa listar o nome do campo da estrutura, e as variáveis ​​criadas a partir do padrão terão os mesmos nomes. O código abaixo se comporta da mesma forma que o código anterior, mas as variáveis ​​criadas são `x` e `y` em vez de `a` e `b`.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

Este código cria as variáveis `x` e `y` que correspondem aos campos `x` e `y` da variável `p`. O resultado é que as variáveis `x` e `y` contêm os valores da estrutura `p`. 

Também podemos desestruturar com valores literais como parte do padrão de estrutura, em vez de criar variáveis para todos os campos. Fazer isso nos permite testar alguns dos campos para valores específicos enquanto criamos variáveis para desestruturar os outros campos. 

No código abaixo, temos uma expressão `match` que separa os valores de um `Point` em três casos: pontos que se situam diretamente no eixo x (o que é verdadeiro quando y = 0), no eixo y (x = 0) ou nenhum dos dois.

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("Ponto no eixo x em {x}"),
        Point { x: 0, y } => println!("Ponto no eixo y em {y}"),
        Point { x, y } => {
            println!("Ponto em nenhum dos eixos: ({x}, {y})");
        }
    }
}
```

O primeiro braço corresponderá a qualquer ponto que esteja no eixo x especificando que o campo `y` corresponde se seu valor corresponder ao literal 0. O padrão ainda cria uma variável `x` que podemos usar no código para este braço.

Da mesma forma, o segundo braço corresponde a qualquer ponto no eixo y especificando que o campo `x` corresponde se seu valor for 0 e cria uma variável `y` para o valor do campo `y`. O terceiro braço não especifica nenhum literal, então ele corresponde a qualquer outro ponto e cria variáveis para os campos `x` e `y`.

Neste exemplo, o valor `p` corresponde ao segundo braço devido a `x` conter um 0, então este código imprimirá "Ponto no eixo y em 7". 

Lembre-se que uma expressão de correspondência para de verificar braços assim que encontra o primeiro padrão correspondente, então mesmo que `Point { x: 0, y: 0}` esteja no eixo x e no eixo y, este código só imprimirá "Ponto no eixo x em 0".

### Desestruturando enums

O padrão para desestruturar uma `enum` corresponde à maneira como os dados armazenados dentro da `enum` são definidos. No exemplo abaixo, usamos uma `enum` chamada `Message` e escrevemos um  `match` com padrões que irão identificar cada valor interno.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("A variante Quit não tem nenhum dado encapsulado.");
        }
        Message::Move { x, y } => {
            println!("Mover na direção x:{x} e na direção y:{y}");
        }
        Message::Write(text) => {
            println!("Texto: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Alterar a cor para R:{r}, G:{g}, B:{b}")
        }
    }
}
```

Este código imprimirá "Alterar a cor para R:0, G:160, B:255".

Para variantes de `enum` sem nenhum dado, como `Message::Quit`, não podemos desestruturar valor. Só podemos corresponder ao valor literal `Message::Quit`, e nenhuma variável está nesse padrão.

Para variantes de `enum` do tipo `struct`, como `Message::Move`, podemos usar um padrão semelhante ao padrão que especificamos para corresponder a structs. Após o nome da variante, colocamos chaves e, em seguida, listamos os campos com variáveis ​​para separar as partes a serem usadas no código para este braço. Aqui, usamos a forma abreviada.

Para variantes de `enum` do tipo tupla, como `Message::Write` que contém uma tupla com um elemento e `Message::ChangeColor` que contém uma tupla com três elementos, o padrão é semelhante ao padrão que especificamos para corresponder a tuplas. O número de variáveis ​​no padrão deve corresponder ao número de elementos na variante que estamos correspondendo.

### Desestruturando structs e enums aninhadas

Até agora, nossos exemplos têm sido todos de correspondência de structs ou enums em um nível de profundidade, mas a correspondência também pode funcionar em itens aninhados! Por exemplo, podemos refatorar o código anterior para suportar cores RGB e HSV na variante `ChangeColor`.

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Alterar a cor para R:{r}, G:{g}, B:{b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Alterar a cor para H:{h}, S:{s}, V:{v}")
        }
        _ => (),
    }
}
```

O padrão do primeiro braço na expressão `match` corresponde a uma variante `Message::ChangeColor` que contém uma variante `Color::Rgb`; então o padrão se liga aos três valores internos i32. O padrão do segundo braço também corresponde a uma variante `Message::ChangeColor`, mas a enumeração interna corresponde a `Color::Hsv`. Podemos especificar essas condições complexas em uma única expressão `match`, mesmo que duas enumerações estejam envolvidas.

### Desestruturando structs e tuplas

Podemos misturar, combinar e aninhar padrões de desestruturação de maneiras ainda mais complexas. O exemplo a seguir mostra uma desestruturação complicada onde aninhamos structs e tuplas dentro de uma tupla e desestruturamos todos os valores primitivos:

```rust
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

Este código nos permite dividir tipos complexos em suas partes componentes para que possamos usar os valores em que estamos interessados separadamente.

A desestruturação com padrões é uma maneira conveniente de usar partes de valores, como o valor de cada campo em uma estrutura, separadamente um do outro. 

## Ignorando valores em um padrão

Algumas vezes é útil ignorar valores em um padrão, como no último braço do `match`, para ter uma opção que corresponde a qualquer coisa e não faz nada, tratando todos os valores possíveis restantes. Existem algumas maneiras de ignorar valores inteiros ou partes de valores em um padrão: usando o padrão `_` (sublinhado), usando o padrão `_` dentro de outro padrão, usando um nome que começa com um sublinhado ou usando `..` (ponto ponto) para ignorar as partes restantes de um valor. Vamos explorar como e por que usar cada um desses padrões.

### Ignorando um valor inteiro com `_`

O caractere sublinhado pode ser usado como um padrão curinga que corresponde a qualquer valor, mas não se liga ao valor. Isso é especialmente útil como o último braço em uma expressão `match`, mas também podemos usá-lo em qualquer padrão, incluindo parâmetros de função, como mostrado abaixo.

```rust
fn foo(_: i32, y: i32) {
    println!("Este código usa apenas o parâmetro y: {y}");
}

fn main() {
    foo(3, 4);
}
```

Este código ignorará completamente o valor 3 passado como o primeiro argumento e imprimirá "Este código usa apenas o parâmetro y: 4".

Na maioria dos casos, quando você não precisa mais de um parâmetro de função específico, você alteraria a assinatura para que ela não incluísse o parâmetro não utilizado. Ignorar um parâmetro de função pode ser especialmente útil em casos em que, por exemplo, você está implementando uma trait quando precisa de uma certa assinatura de tipo, mas o corpo da função em sua implementação não precisa de um dos parâmetros. Você então evita receber um aviso do compilador sobre parâmetros de função não utilizados, como aconteceria se usasse um nome.

### Ignorando partes de um valor com um `_` aninhado

Também podemos usar `_` dentro de outro padrão para ignorar apenas parte de um valor, por exemplo, quando queremos testar apenas parte de um valor, mas não temos uso para as outras partes no código correspondente que queremos executar. O exemplo abaixo mostra o código responsável por gerenciar o valor de uma configuração. Os requisitos de negócios são que o usuário não deve ter permissão para sobrescrever uma personalização existente de uma configuração, mas pode desabilitar a configuração e dar a ela um valor se ela estiver atualmente desabilitada.

```rust
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Não é permitido sobrescrever uma configuração existente.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Configurado como: {setting_value:?}");
```

Note que usar `_` dentro de um `Some` indica que não precisamos usar o valor dentro do `Some`. O que interessa aqui é se o `Option` `setting_value` já tem algum valor embutido. Se tiver, não podemos sobrescrever. O valor embutido em `new_setting_value` também não interessa neste caso. 

Este código imprimirá:

```
Não é permitido sobrescrever uma configuração existente.
Configurado como: Some(5)
```

No primeiro braço do `match`, não precisamos combinar ou usar os valores dentro de nenhuma variante `Some`, mas precisamos testar o caso em que `setting_value` e `new_setting_value` são a variante `Some`. Nesse caso, imprimimos o motivo para não alterar `setting_value`, e ele não é alterado.

Em todos os outros casos (se `setting_value` ou `new_setting_value` forem `None`), expressos pelo padrão `_` no segundo braço, queremos permitir que `new_setting_value` se torne `setting_value`. 
 
Também podemos usar sublinhados em vários lugares dentro de um padrão para ignorar valores específicos. O código abaixo mostra um exemplo de como ignorar o segundo e o quarto valores em uma tupla de cinco itens. 

```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Alguns números: {first}, {third}, {fifth}")
        }
    }
```

Este código imprimirá "Alguns números: 2, 8, 32", e os valores 4 e 16 serão ignorados. 

### Ignorando uma variável não utilizada iniciando seu nome com `_` 

Se você criar uma variável, mas não usá-la em lugar nenhum, o Rust geralmente emitirá um aviso porque uma variável não usada pode ser um bug. No entanto, às vezes é útil poder criar uma variável que você ainda não usará, como quando você está prototipando ou apenas iniciando um projeto. Nessa situação, você pode dizer ao Rust para não avisá-lo sobre a variável não usada iniciando o nome da variável com um sublinhado. Abaixo, criamos duas variáveis ​​não usadas, mas quando compilamos esse código, devemos receber apenas um aviso sobre uma delas.

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

Aqui recebemos um aviso sobre não usar a variável `y`, mas não recebemos um aviso sobre não usar `_x`. 

Observe que existe uma diferença sutil entre usar apenas `_` e usar um nome que começa com um sublinhado. A sintaxe `_x` ainda vincula o valor à variável, enquanto `_` não vincula nada. Para mostrar um caso em que essa distinção importa, o exemplo abaixo nos fornecerá um erro.

```rust
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("encontrada uma string");
    }

    println!("{s:?}");
```

Receberemos um erro porque o valor de `s` ainda será movido para `_s`, o que impede que usemos `s` novamente. No entanto, usar o sublinhado sozinho nunca se liga ao valor. O código abaixo será compilada sem erros porque `s` não é movido para `_`.

```rust
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("encontrada uma string");
    }

    println!("{s:?}");
``` 

Este código funciona perfeitamente porque nunca ligamos `s` a nada; ele não é movido. 

### Ignorando partes restantes de um valor com `..`

Com valores que têm muitas partes, podemos usar a sintaxe `..` (ponto ponto) para usar partes específicas e ignorar o resto, evitando a necessidade de listar sublinhados para cada valor ignorado. O padrão `..` ignora quaisquer partes de um valor que não correspondam explicitamente no restante do padrão. No código abaixo, temos uma estrutura `Point` que contém uma coordenada no espaço tridimensional. Na expressão `match`, queremos operar apenas na coordenada `x` e ignorar os valores nos campos `y` e `z`.

```rust
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x: {x}"),
    }
```

Listamos o valor de `x` e então apenas incluímos o padrão `..` . Isso é mais rápido do que ter que listar `y: _` e `z: _`, particularmente quando estamos trabalhando com structs que têm muitos campos em situações onde apenas um ou dois campos são relevantes. 
 
A sintaxe `..` expandirá para quantos valores forem necessários. Abaixo temos como usar `..` com uma tupla.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Alguns números: {first}, {last}");
        }
    }
}
```

Neste código, o primeiro e o último valor são comparados com `first` e `last`. O `..` irá comparar e ignorar tudo no meio.
 
No entanto, usar `..` não permite ambiguidade. Se não estiver claro quais valores são destinados à correspondência e quais devem ser ignorados, o Rust nos dará um erro. Abaixo temos um exemplo de uso de `..` de forma ambígua, portanto, não será compilado.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Alguns números: {second}")
        },
    }
}
```

Quando compilamos este exemplo, obtemos este erro:

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here

error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

É impossível para Rust determinar quantos valores na tupla ignorar antes de corresponder um valor com `second` e então quantos valores adicionais ignorar depois disso. Esse código pode significar que queremos ignorar 2, vincular `second` a 4 e então ignorar 8, 16 e 32; ou que queremos ignorar 2 e 4, vincular `second` a 8 e então ignorar 16 e 32; e assim por diante. O nome da variável `second` não significa nada especial para Rust, então obtemos um erro do compilador porque usar `..` em dois lugares como esse é ambíguo. 

---

## match guard conditional

Um **match guard conditional** é uma condição `if` adicional, especificada após o padrão em um braço de um `match`, que também deve corresponder para que esse braço seja escolhido. Os match guards são úteis para expressar ideias mais complexas do que um padrão sozinho permite.

O match guard pode usar variáveis criadas no padrão. O exemplo abaixo mostra que o primeiro braço tem o padrão `Some(x)` e também tem o match guard `if x % 2 == 0` (que será verdadeira se o número for par).

```rust
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("O número {x} é par"),
        Some(x) => println!("O número {x} é ímpar"),
        None => (),
    }
```

Este exemplo imprimirá "O número 4 é par". Quando `num` é comparado com o padrão no primeiro braço, ele coincide, porque `Some(4)` coincide com `Some(x)`. Então, o match guard verifica se o resto da divisão de `x` por 2 é igual a 0, e como é, o primeiro braço é selecionado.

Se `num` tivesse sido `Some(5)` em vez disso, o match guard no primeiro braço teria sido falso porque o resto da divisão de 5 por 2 é 1, o que não é igual a 0. O Rust então iria para o segundo braço, que corresponderia porque o segundo braço não tem match guard e, portanto, corresponde a qualquer variante `Some`.

Não há como expressar a condição `if x % 2 == 0` dentro de um padrão, então o match guard nos dá a capacidade de expressar essa lógica. A desvantagem dessa expressividade adicional é que o compilador não tenta verificar a exaustividade quando match guards estão envolvidas.

Anteriormente, na seção [Variáveis](#vari%C3%A1veis), mencionamos que poderíamos usar match guard para resolver nosso problema de sombreamento de padrões. Lembre-se de que criamos uma nova variável no braço de `match` em vez de usar a variável externa. Essa nova variável significava que não poderíamos testar o valor da variável externa. O código abaixo mostra como podemos usar um match guard para corrigir esse problema.

```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("x vale 50"),
        Some(n) if n == y => println!("Combinou com y = {y}"),
        _ => println!("Caso default, x = {x:?}"),
    }

    println!("ao final temos: x = {x:?}, y = {y}");
```

Este código agora imprimirá "Caso default, x = Some(5)". O padrão no segundo braço não introduz uma nova variável `y` que sombrearia o `y` externo, o que significa que podemos usar o `y` externo no match guard. Em vez de especificar o padrão como `Some(y)`, o que teria sombreado o `y` externo, especificamos `Some(n)`. Isso cria uma nova variável `n` que não sombreia nada porque não há variável `n` fora do `match`. 

 
O match guard `if n == y` não é um padrão e, portanto, não introduz novas variáveis. Este `y` é o `y` externo, em vez de um novo `y` sombreado, e podemos procurar um valor que tenha o mesmo valor que o `y` externo comparando `n` com `y`.

Você também pode usar o operador ou `|` em um match guard para especificar vários padrões; a condição do match guard se aplicará a todos os padrões. Abaixo temos a precedência ao combinar um padrão que usa `|` com um match guard. A parte importante deste exemplo é que o match guard `if y` se aplica a 4, 5 e 6, mesmo que pareça que `if y` só se aplica a 6.

```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
```

A condição de correspondência afirma que o braço só corresponde se o valor de `x` for igual a 4, 5 ou 6 e se `y` for verdadeiro. Quando esse código é executado, o padrão do primeiro braço corresponde porque `x` é 4, mas a match guard `if y` não é verdadeira, então o primeiro braço não é escolhido. O código passa para o segundo braço, que corresponde, e esse programa imprime "no". O motivo é que a condição `if` se aplica a todo o padrão `4 | 5 | 6`, não apenas ao último valor 6. Em outras palavras, a precedência de um match guard em relação a um padrão se comporta assim:

```rust
(4 | 5 | 6) if y => ...
```

Em vez disso:

```rust
4 | 5 | (6 if y) => ...
```

Após executar o código, o comportamento de precedência é evidente: se o match guard fosse aplicado apenas ao valor final na lista de valores especificados usando o operador `|`, o braço teria correspondido e o programa teria impresso "yes".

---

## Operador @

O operador `@` nos permite criar uma variável que contém um valor ao mesmo tempo em que testamos esse valor para uma correspondência de padrão. No código abaixo, queremos testar se um campo `Message::Hello id` está dentro do intervalo `3..=7`. Também queremos vincular o valor à variável `id_variable` para que possamos usá-lo no código associado ao braço. Poderíamos nomear essa variável `id`, a mesma do campo, mas para este exemplo usaremos um nome diferente.

```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Encontrado id no intervalo: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Encontrado id em outro intervalo")
        }
        Message::Hello { id } => println!("Encontrado outro id: {id}"),
    }
```

Este exemplo imprimirá "Encontrado id no intervalo: 5". Ao especificar `id_variable @` antes do intervalo `3..=7`, estamos capturando qualquer valor que corresponda ao intervalo, ao mesmo tempo em que testamos se o valor corresponde ao padrão do intervalo.

No segundo braço, onde temos apenas um intervalo especificado no padrão, o código associado ao braço não possui uma variável que contém o valor real do campo `id`. O valor do campo `id` poderia ter sido 10, 11 ou 12, mas o código que acompanha esse padrão não sabe qual é. O código do padrão não consegue usar o valor do campo `id`, porque não salvamos o valor do `id` em uma variável.

No último braço, onde especificamos uma variável sem um intervalo, temos o valor disponível para usar no código do braço em uma variável chamada `id`. A razão é que usamos a sintaxe abreviada de campo de estrutura. Mas não aplicamos nenhum teste ao valor no campo `id` neste braço, como fizemos com os dois primeiros braços: qualquer valor corresponderia a este padrão.

Usar `@` permite-nos testar um valor e salvá-lo em uma variável dentro de um padrão. 
 
---

## Referências

[The Book - Chapter 6](https://doc.rust-lang.org/book/ch06-03-if-let.html)

[The Book - Chapter 18](https://doc.rust-lang.org/book/ch18-00-patterns.html)

---

arataca89@gmail.com

Última atualização: 20250117