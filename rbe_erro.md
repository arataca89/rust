# RBE - manipulação de erros

O tratamento de erros é o processo de lidar com a possibilidade de falha. Por exemplo, falhar em ler um arquivo e continuar usando essa entrada ruim seria claramente problemático. Perceber e gerenciar explicitamente esses erros salva o resto do programa de várias armadilhas. 

Existem várias maneiras de lidar com erros em Rust, que são descritas a seguir. Todos eles têm diferenças mais ou menos sutis e casos de uso diferentes. Como regra geral: 

Um pânico explícito é principalmente útil para testes e lidar com erros irrecuperáveis. Para prototipagem, pode ser útil, por exemplo, ao lidar com funções que ainda não foram implementadas, mas nesses casos, o ```unimplemented``` mais descritivo é melhor. Em testes, o pânico é uma maneira razoável de falhar explicitamente. 

O tipo ```Option``` é para quando um valor é opcional ou quando a falta de um valor não é uma condição de erro. Por exemplo, o pai de um diretório - / e C: não têm um. Ao lidar com Options, ```unwrap``` é bom para prototipagem e casos em que é absolutamente certo que haverá um valor. No entanto, ```expect``` é mais útil, pois permite especificar uma mensagem de erro caso algo dê errado de qualquer maneira.

Quando há uma chance de que as coisas dêem errado e o chamador tenha que lidar com o problema, use ```Result```. Você pode usar ```unwrap``` e ```expect``` também (por favor, não faça isso a menos que seja um teste ou protótipo rápido).

Para uma discussão mais rigorosa sobre o tratamento de erros, consulte a seção de tratamento de erros no [livro oficial](https://doc.rust-lang.org/book/ch09-00-error-handling.html). 
 

* [panic](#panic)
* abort e unwind
* [Option e unwrap](#Option-e-unwrap)
* [Operador interrogação](#Operador-interrogação)
* [Combinador map](#Combinador-map)
* [Combindor and_then](#Combindor-and-then)
* [Diferença entre map e and_then](#Diferença-entre-map-e-and_then)
* [Opções de desempacotamento de Option e padrões](#Opções-de-desempacotamento-de-Option-e-padrões)

---

## panic

O mecanismo de tratamento de erros mais simples que veremos é o pânico. Ele imprime uma mensagem de erro, inicia a desmontagem da pilha e geralmente encerra o programa. Aqui, chamamos explicitamente o pânico em nossa condição de erro: 

```
fn drink(beverage: &str) {
    // Você não deve beber bebidas com muito açucar
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
```

A primeira chamada a ```drink()```funciona. A segunda entra em pânico e, portanto, a terceira nunca é chamada. 

---

## Option e unwrap

No último exemplo, mostramos que podemos induzir falhas no programa à vontade. Dissemos ao nosso programa para entrar em pânico se bebermos uma limonada açucarada. Mas e se esperarmos alguma bebida, mas não recebermos nenhuma? Esse caso seria tão ruim quanto, então precisa ser tratado! 

Poderíamos testar se o argumento passado para a função foi uma string vazia mas, como estamos usando Rust, vamos em vez disso fazer o compilador apontar os casos em que não há bebida. 

Rust tem uma enumeração chamada ```Option<T>``` na biblioteca padrão que pode ser usada quando a ausência é uma possibilidade. Ela se manifesta como uma de duas "opções":

* ```Some(T)```: um valor do tipo ```T``` foi encontrado;
* ```None```: nenhum valor foi encontrado. 

Esses casos podem ser tratados explicitamente via ```match``` ou implicitamente com ```unwrap```. O tratamento implícito retornará o elemento interno ou causará um ```panic```. 

Observe que é possível personalizar manualmente o pânico com ```expect```, mas ```unwrap```, ao contrário, nos deixa com uma saída menos significativa do que o tratamento explícito. No exemplo a seguir, o tratamento explícito produz um resultado mais controlado, mantendo a opção de entrar em pânico se desejado.


```
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Muito açucar."),
        Some(inner)   => println!("{}? Beleza.", inner),
        None          => println!("Não foi passada nenhuma bebida."),
    }
}

fn drink(drink: Option<&str>) {
    // 'unwrap' retorna um 'panic' quando recebe um 'None'
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }
    println!("Eu amo {}!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
```

---

## Operador interrogação

Você pode descompactar ```Option``` usando ```match```, mas geralmente é mais fácil usar o operador ```?``` (interrogação). Se ```x``` for uma ```Option```, então avaliar ```x?``` retornará o valor subjacente se ```x``` for ```Some```, caso contrário, terminará qualquer função que está sendo executada e retornará ```None```.

```
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // Se 'current_age' for 'None', retornará 'None'.
    // Se 'current_age' for 'Some', retornará o valor interno 'u8' + 1,
    // o qual será atribuído a 'next_age'.
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))	
}
```

Você pode encadear vários ```?``` juntos para tornar seu código muito mais legível. 

```
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Obtém o código de área do número de telefone do trabalho da pessoa, se existir.
    fn work_phone_area_code(&self) -> Option<u8> {
        // Isso precisaria de muitas instruções 'match' aninhadas sem o operador '?'.
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```


---

## Combinador map

```match``` é um método válido para lidar com ```Option```. No entanto, você pode acabar achando o uso excessivo tedioso, especialmente com operações válidas apenas com uma entrada. Nesses casos, combinadores podem ser usados para gerenciar o fluxo de controle de forma modular. 

```Option``` possui um método embutido chamado ```map()```, um combinador para o mapeamento simples de ```Some -> Some``` e ```None -> None```. Múltiplas chamadas a ```map()``` podem ser encadeadas para maior flexibilidade. 

No exemplo a seguir, ```process()``` substitui todas as funções anteriores a ele, mantendo-se compacto. 

```
#![allow(dead_code)]

#[derive(Debug)] enum Comida {Banana, Cenoura, Batata }

#[derive(Debug)] struct Descascada(Comida);
#[derive(Debug)] struct Picada(Comida);
#[derive(Debug)] struct Cozida(Comida);

// Descascando comida. Se não houver nenhuma, então retorne 'None'.
// Caso contrário, retorne a comida descascada.
fn descascar(comida: Option<Comida>) -> Option<Descascada> {
    match comida {
        Some(comida) => Some(Descascada(comida)),
        None         => None,
    }
}

// Cortando comida. Se não houver nenhuma, então retorne 'None'.
// Caso contrário, retorne a comida picada.
fn picar(descascada: Option<Descascada>) -> Option<Picada> {
    match descascada {
        Some(Descascada(comida)) => Some(Picada(comida)),
        None                     => None,
    }
}

// Cozinhando comida. Aqui, usamos 'map()' em vez de 'match' para tratamento de casos.
fn cozinhar(picada: Option<Picada>) -> Option<Cozida> {
    picada.map(|Picada(comida)| Cozida(comida))
}

// Uma função para descascar, cortar e cozinhar comida, tudo em sequência.
// Encadeamos múltiplos 'map()' para simplificar o código.
fn processar(comida: Option<Comida>) -> Option<Cozida> {
    comida.map(|f| Descascada(f))
        .map(|Descascada(f)| Picada(f))
        .map(|Picada(f)| Cozida(f))
}

// Verifique se tem comida ou não antes de tentar comê-la!
fn comer(comida: Option<Cozida>) {
    match comida {
        Some(comida) => println!("Ummm. Eu gosto de {:?}", comida),
        None       => println!("Oh não! Não há o que comer."),
    }
}

fn main() {
    let banana = Some(Comida::Banana);
    let cenoura = Some(Comida::Cenoura);
    let batata = None;

    let cozida_banana = cozinhar(picar(descascar(banana)));
    let cozida_cenoura = cozinhar(picar(descascar(cenoura)));
    // usar 'processar()' deixa o código bem mais simples
    let cozida_batata = processar(batata);

    comer(cozida_banana);
    comer(cozida_cenoura);
    comer(cozida_batata);
}
```

Veja também:

[closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)

[Options](https://doc.rust-lang.org/std/option/enum.Option.html)

[Option::map()](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)

---

## Combindor and_then

```map()``` foi descrito como uma forma encadeável de simplificar instruções ```match```. No entanto, usar ```map()``` em uma função que retorna um ```Option<T>``` resulta em ```Option<Option<T>>```. Encadear várias chamadas juntas pode então se tornar confuso. É aí que entra outro combinador chamado ```and_then()```, conhecido em algumas linguagens como flatmap.

```and_then()``` chama sua função de entrada com o valor encapsulado e retorna o resultado. Se a Opção for ```None```, então ele retorna ```None```.

No exemplo a seguir, ```cookable_v3()``` resulta em um ```Option<Food>``` . Usar ```map()``` em vez de ```and_then()``` teria dado um ```Option<Option<Food>>```, que é um tipo inválido para ```eat()```.

```
#![allow(dead_code)]

#[derive(Debug)] enum Comida { Cuzcuz, Tapioca, Sopa }
#[derive(Debug)] enum Dia { Segunda, Quarta, Sexta }

// Não temos ingredientes para fazer uma Sopa.
fn ingredientes(comida: Comida) -> Option<Comida> {
    match comida {
        Comida::Sopa => None,
        _            => Some(comida),
    }
}

// Temos receita pra tudo menos para Cuzcuz
fn receita(comida: Comida) -> Option<Comida> {
    match comida {
        Comida::Cuzcuz => None,
        _                => Some(comida),
    }
}

// Para fazer a refeição precisamos de receita e dos ingredientes.
// Podemos representar a lógica usando 'match'
fn cozinhar_v1(comida: Comida) -> Option<Comida> {
    match receita(comida) {
        None       => None,
        Some(comida) => ingredientes(comida),
    }
}

// Isto pode ser refatorado de forma mais compacta usando 'and_then':
fn cozinhar_v3(comida: Comida) -> Option<Comida> {
    receita(comida).and_then(ingredientes)
}

// ou pode-se usar 'flatten()' em 'Option<Option<comida>>'
// para extrair um 'Option<Comida>':
fn cozinhar_v2(comida: Comida) -> Option<Comida> {
    receita(comida).map(ingredientes).flatten()
}

fn comer(comida: Comida, dia: Dia) {
    match cozinhar_v3(comida) {
        Some(comida) => println!("OK! Na {:?} vamos comer {:?}.", dia, comida),
        None       => println!("Oh não. Não temos nada para comer na {:?}", dia),
    }
}

fn main() {
    let (cuzcuz, tapioca, sopa) = (Comida::Cuzcuz, Comida::Tapioca, Comida::Sopa);

    comer(cuzcuz, Dia::Segunda);
    comer(tapioca, Dia::Quarta);
    comer(sopa, Dia::Sexta);
}
```

---

## Diferença entre map e and_then


Basicamente a diferença está no argumento que você fornece para a closure (ou Fn).

 ```map()``` recebe um ```FnOnce(T) -> U``` enquanto ```and_then()``` recebe um ```FnOnce(T) -> Option<U>```. 

Com ```and_then()``` você pode combinar várias operações que podem falhar individualmente, enquanto ```map()``` aplica uma transformação infalível. 


Em Rust, ```and_then()``` e ```map()``` são métodos usados ​​para transformar tipos ```Option``` ou ```Result```, mas eles têm padrões de uso diferentes.

O método ```map``` mapeia um ```Option``` ou ```Result<T, E>``` para um novo ```Option``` ou ```Result<U, E>```, onde a operação descrita na closure é aplicada ao valor contido no ```Option``` ou ```Result```. Se o valor original for ```None``` ou ```Err```, a função de mapeamento não será executada e, em vez disso, um novo ```None``` ou ```Err``` será retornado.

Por exemplo, aqui está um exemplo usando o método ```map``` para dobrar o valor em um ```Option```:

```
let some_number = Some(5);
let doubled = some_number.map(|x| x * 2);
assert_eq!(doubled, Some(10));
```

O método ```and_then``` é semelhante ao ```map``` em uso, mas seu tipo de retorno é ```Option``` ou ```Result<U, E>``` em vez de `U`. Na closure de ```and_then```, devemos retornar um novo ```Option``` ou ```Result``` em vez de retornar diretamente um valor. Isso significa que ```and_then``` pode ser usado para transformar um ```Option``` ou ```Result``` em outro, ao mesmo tempo em que realiza alguns testes lógicos. 

Por exemplo, aqui está um exemplo usando o método ```and_then``` para multiplicar o valor em um ```Option``` por 3, retornando ```None``` se o valor for menor que 10:

```
let some_number = Some(5);
let result = some_number.and_then(|x| {
    if x < 10 {
        None
    } else {
        Some(x * 3)
    }
});
assert_eq!(result, None);

let some_number = Some(11);
let result = some_number.and_then(|x| {
    if x < 10 {
        None
    } else {
        Some(x * 3)
    }
});
assert_eq!(result, Some(33));
```
 
Portanto, em uso, ```map``` é usado para transformações de valor simples, enquanto ```and_then``` é usado para operações mais complexas e testes lógicos. 

---

## Opções de desempacotamento de Option e padrões

asd

---

## asd

asd

---

## Referências

[RBE - Error handling](https://doc.rust-lang.org/rust-by-example/error.html)

[difference between map and and-then](https://users.rust-lang.org/t/what-is-the-difference-between-map-and-and-then/29108)

[https://davirain.xlog.app/and_then-he-map-zai-shi-yong-shang-you-shen-me-qu-bie?locale=en](https://davirain.xlog.app/and_then-he-map-zai-shi-yong-shang-you-shen-me-qu-bie?locale=en)


---

arataca89@gmail.com

Última atualização: 20241031
