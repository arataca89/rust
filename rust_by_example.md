# Rust através de exemplos

[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) (RBE ou Rust através de exemplos, numa tradução livre) é uma coleção de exemplos executáveis que ilustram vários conceitos e bibliotecas padrão do Rust. Este artigo aborda alguns tópicos do RBE .

* [match](#match)
* [Option](#option)
* [Result](#result)
* [Operador ?](#operador-)
* [panic!](#panic)
* [HashMap](#hashmap)
* [Chaves em HashMap](#chaves-em-hashmap)
* [HashSet](#hashset)
* [Rc](#rc)
* [Arc](#arc)
* [Manipulação de erro](rbe_erro.md#arataca89)
  
---

## match

Rust tem uma construção chamada ```match``` que pode ser usada como um switch da linguagem C. 

```
fn main() {
    // Teste diferentes valores para 'number'
    let number = 1;

    println!("number: {}", number);

    match number {
        // Compara 'number' com um valor único
        1 => println!("'number' vale um!"),
        // Compara com vários valores
        2 | 3 | 5 | 7 | 11 => println!("'number' é um número primo"),
        // Compara com os valores do intervalo [18,60]
        18..=60 => println!("'number' vale a idade de um adulto"),
        // Trata o resto dos casoss
        _ => println!("'number' tem qualquer outro valor não tratado por match"),
    }

    let boolean = true;

    let binary = match boolean {
        // Os braços de match devem cobrir todos os possíveis valores
        false => 0,
        true => 1,
        // Comente um dos braços e um erro será emitido
    };

    println!("{} -> {}", boolean, binary);
}
```

---

## Option

Às vezes, é desejável capturar a falha de algumas partes de um programa em vez de chamar ```panic!```; isso pode ser feito usando o enum ```Option```.

O enum ```Option<T>``` possui duas variantes:

* ```None```, para indicar falha ou falta de valor; e
* ```Some(valor)```, uma estrutura de tupla que envolve um ```valor``` com tipo ```T```.

```
// rbe_option

// Executa uma divisão e não chama 'panic!' em caso de erro
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // A falha é representada como 'None'
        None
    } else {
        // O resultado é encapsulado em 'Some'
        Some(dividend / divisor)
    }
}

// Esta função trata uma divisão que pode não ser bem sucedida
fn try_division(dividend: i32, divisor: i32) {
    // 'Option', como qualquer enum, pode ser processada via match
    match checked_division(dividend, divisor) {
        None => println!("{} / {} ERRO!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Vincular 'None' a uma variável necessita anotar o tipo da variável
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Usar 'unwrap()' em um 'Some' extrairá o valor dentro do 'Some'
    println!("{:?} é um 'Some' e tem dentro o valor {:?}", optional_float, optional_float.unwrap());

    // Usar 'unwrap' em um 'None' causará 'panic!'
    println!("{:?} é um 'None' e tem dentro o valor {:?}", none, none.unwrap());
}
```

Execução:

```
4 / 2 = 2
1 / 0 ERRO!
Some(0.0) é um 'Some' e tem dentro o valor 0.0
thread 'main' panicked at src/main.rs:39:71:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_option.exe` (exit code: 101)
```

---

## Result

Vimos que o enum ```Option``` pode ser usado como um valor de retorno de funções que podem falhar, onde ```None``` pode ser retornado para indicar falha. No entanto, às vezes é importante expressar por que uma operação falhou. Para fazer isso, temos o enum ```Result```.

O enum ```Result<T, E>``` tem duas variantes: 

* ```Ok(value)```, que indica que a operação foi bem-sucedida e encapsula o valor retornado pela operação. (```value``` tem o tipo ```T```)
* ```Err(why)```, que indica que a operação falhou e encapsula ```why```, que (esperançosamente) explica a causa da falha. (```why``` tem o tipo ```E```)

```
// rbe_result

mod checked {
    // Erros matemáticos que queremos capturar
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // Esta operação falharia, em vez disso, vamos retornar o motivo da
            // falha encapsulada em 'Err'
            Err(MathError::DivisionByZero)
        } else {
            // Esta operação é válida, retorna o resultado dentro de um 'Ok'
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// 'op(x, y)' === 'sqrt(ln(x / y))'
fn op(x: f64, y: f64) -> f64 {
    // Uma pirâmide de 'match' com três níveis
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Isso irá falhar?
   println!("{}", op(1.0, 10.0));
}
```

Execução:

```
thread 'main' panicked at src/main.rs:50:29:
NegativeSquareRoot
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_result.exe` (exit code: 101)
```

---

## Operador ?

Encadear resultados usando ```match``` pode ficar confuso. O operador ```?``` pode ser usado para deixar o código mais fácil de entender. O operador ```?``` é usado no final de uma expressão que retorna um ```Result```, e é equivalente a uma expressão ```match```, onde o ramo ```Err(err)``` se expande para ```return Err(From::from(err))```, e o ramo ```Ok(ok)``` se expande para uma expressão ```ok```.

```
// rbe_opearador_interrogacao

mod checked {
    // Erros matemáticos que queremos capturar
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // Esta operação falharia, em vez disso, vamos retornar o motivo da
            // falha encapsulada em 'Err'
            Err(MathError::DivisionByZero)
        } else {
            // Esta operação é válida, retorna o resultado dentro de um 'Ok'
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Função intermediária
    fn op_(x: f64, y: f64) -> MathResult {
        // se 'div()' falhar, 'DivisionByZero' será retornado.
        let ratio = div(x, y)?;

        // se 'ln()' falhar, 'NonPositiveLogarithm' será retornado.
        let ln = ln(ratio)?;

        sqrt(ln)
    }


    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logarítmo de número negativo",
                MathError::DivisionByZero
                    => "divisão por zero",
                MathError::NegativeSquareRoot
                    => "raiz quadrada de número negativo",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

Execução:

```
thread 'main' panicked at src/main.rs:55:25:
raiz quadrada de número negativo
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rbe_result.exe` (exit code: 101)
```

---

## panic!
 
A macro ```panic!``` pode ser usada para gerar um pânico e iniciar a desmontagem da pilha. Durante essa desmontagem todos os recursos de propriedade da thread serão liberados chamando o destrutor de todos os seus objetos. 

Como estamos lidando com programas com apenas uma thread, ```panic!``` fará com que o programa reporte a mensagem de pânico e saia. 

```
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Divisão por zero irá gerar pânico
        panic!("divisão por zero");
    } else {
        dividend / divisor
    }
}

fn main() {
    // inteiro alocado na memória heap
    let _x = Box::new(0i32);

    // Esta operação irá gerar um erro
    division(3, 0);

    println!("O controle não chega neste ponto!");

    // `_x` será destruído neste ponto
}
```

Vamos verificar se ```panic!``` não causa vazamento de memória. 

```
$ rustc panic.rs && valgrind ./panic
==4401== Memcheck, a memory error detector
==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==4401== Command: ./panic
==4401== 
thread '<main>' panicked at 'division by zero', panic.rs:5
==4401== 
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401== 
==4401== All heap blocks were freed -- no leaks are possible
==4401== 
==4401== For counts of detected and suppressed errors, rerun with: -v
==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

---

## HashMap

Enquanto vetores armazenam valores por um índice inteiro, ```HashMaps``` armazenam valores por chave. As chaves de um ```HashMap``` podem ser booleanos, inteiros, strings ou qualquer outro tipo que implemente as traits ```Eq``` e ```Hash```. 

Assim como vetores, HashMaps são expansíveis, mas HashMaps também podem diminuir de tamanho quando têm espaço em excesso. Você pode criar um ```HashMap``` com uma determinada capacidade inicial usando ```HashMap::with_capacity(uint)```, ou usar ```HashMap::new()``` para obter um ```HashMap``` com uma capacidade inicial padrão(recomendado). 

```
// rbe_hashmap

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "Lamentamos, mas a chamada não pode ser completada.
        Desligue e tente novamente.",
        "645-7689" => "Olá, aqui é o Mr. Awesome's Pizza. Meu nome é Fred.
        O que posso fazer por você hoje?",
        _ => "Olá! Quem é de novo?"
    }
}

fn main() { 
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Recebe uma referência e retorna Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Chamando Daniel: {}", call(number)),
        _ => println!("Não tenho o número do Daniel."),
    }

    // 'HashMap::insert()' retorna 'None' se o valor inserido é novo;
    // senão retorna 'Some(value)'
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Chamando Ashley: {}", call(number)),
        _ => println!("Não tenho o número de Ashley."),
    }

    contacts.remove(&"Ashley"); 

    // `HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.
    // 'HashMap::iter()' retorna um iterador que produz pares
    // (&'a key, &'a value) pem ordem arbitrária.
    for (contact, &number) in contacts.iter() {
        println!("Chamando {}: {}", contact, call(number)); 
    }
}
```

Para mais informações sobre o tipo HashMap consulte [https://doc.rust-lang.org/std/collections/struct.HashMap.html](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

---

## Chaves em HashMap

Qualquer tipo que implemente as traits ```Eq``` e ```Hash``` pode ser chave de um ```HashMap```. Isso inclui:

* ```bool``` (embora não seja muito útil, pois existem apenas duas chaves possíveis);
* ```int```, ```uint``` e todas as suas variações;
* ```String``` e ```&str``` (dica: você pode ter um ```HashMap``` com chave ```String``` e chamar ```get()``` com um ```&str```). 

Observe que ```f32``` e ```f64``` não implementam ```Hash```, provavelmente porque erros de precisão de ponto flutuante tornariam o uso deles como chaves de hashmap terrivelmente propenso a erros.

Os tipos que são coleção implementam ```Eq``` e ```Hash``` se seu tipo contido também implementar ```Eq``` e ```Hash```, respectivamente. Por exemplo, ```Vec<T>``` implementará ```Hash``` se ```T``` implementar ```Hash```. 
 
Você pode facilmente implementar ```Eq``` e ```Hash``` para um tipo personalizado com apenas uma linha:

```
#[derive(PartialEq, Eq, Hash)]
```

O compilador fará o resto. Se você quiser mais controle sobre os detalhes, pode implementar ```Eq``` e/ou ```Hash``` você mesmo. Este guia não abordará os detalhes da implementação de ```Hash```.

Para demonstrar o uso de uma ```struct``` em um ```HashMap```, vamos criar um sistema de login de usuário muito simples: 
 
```
// rbe_hashmap2

use std::collections::HashMap;

// 'Eq' requer que se derive 'PartialEq' também.
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
                 username: &'a str,
                 password: &'a str){
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Tentando fazer login...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Login bem sucedido!");
            println!("Nome: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Falha ao tentar fazer Login!"),
    }
}

fn main(){
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
```

Execução:

```
Username: j.everyman
Password: psasword123
Tentando fazer login...
Falha ao tentar fazer Login!
Username: j.everyman
Password: password123
Tentando fazer login...
Login bem sucedido!
Nome: John Everyman
Email: j.everyman@email.com
``` 

---

## HashSet

Considere um ```HashSet``` como um ```HashMap``` onde nos importamos apenas com as chaves (```HashSet<T>``` é, na verdade, apenas uma camada em torno de ```HashMap<T, ()>```). 

"Qual o sentido disso?", você pergunta. "Eu poderia simplesmente armazenar as chaves em um ```Vec```."

A característica única de um ```HashSet``` é que ele garante não ter elementos duplicados. Esse é o contrato que qualquer coleção de conjuntos cumpre. ```HashSet``` é apenas uma implementação. (veja também: [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)). 

Se você inserir um valor que já está presente no ```HashSet```, (ou seja, o novo valor é igual ao existente e ambos têm o mesmo hash), então o novo valor substituirá o antigo. 

Isso é ótimo para quando você nunca quer mais do que uma unidade de alguma coisa, ou quando você quer saber se já tem alguma coisa. 

Mas conjuntos podem fazer mais do que isso. 

Conjuntos(sets) possuem 4 operações primárias (todas retornam um iterador):

* ```union```(união): obter todos os elementos exclusivos em ambos os conjuntos.
* ```difference```(diferença): obter todos os elementos que estão no primeiro conjunto, mas não no segundo.
* ```intersection```(interseção): obter todos os elementos que estão apenas em ambos os conjuntos.
* ```symmetric_difference```(diferença simétrica): obter todos os elementos que estão em um conjunto ou no outro, mas não em ambos.

```
// rbe_hashset

use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 'HashSet::insert()' retorna 'false' se o valor já existe
    //assert!(b.insert(4), "O valor 4 já existe em 'b'!");
    // CORREÇÃO ^ Comente a linha acima

    b.insert(5);

    // Se o tipo de dado do elemento da coleção implementa 'Debug',
    // então a coleção implementa 'Debug'.
    // Normalmente os elementos são exibidos no formato '[elem1, elem2, ...]'
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Exibe [1, 2, 3, 4, 5] em uma ordem arbitrária
    println!("union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // Deve exibir [1]
    println!("difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Exibe [ 2, 3, 4] em uma ordem arbitrária
    println!("intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // Exibe [1, 5]
    println!("symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
```

---

## Rc

Quando múltiplas propriedades são necessárias, ```Rc``` ('Reference Counting', ou 'Contagem de Referências' numa tradução livre) pode ser usado. ```Rc``` acompanha o número de referências, o que significa o número de proprietários do valor encapsulado em um ```Rc```.

A contagem de referência de um ```Rc``` aumenta em 1 sempre que um ```Rc``` é clonado e diminui em 1 sempre que um ```Rc``` clonado é destruído (sai do escopo). Quando a contagem de referência de um ```Rc``` se torna zero (o que significa que não há mais proprietários), tanto o ```Rc``` quanto o valor são destruídos.

Clonar um ```Rc``` nunca realiza uma cópia profunda (deep copy). A clonagem cria apenas outro ponteiro para o valor encapsulado e incrementa a contagem. 

```
 // rbe_rc

use std::rc::Rc;

fn main() {
    let rc_examples = "Exemplos do uso de Rc".to_string();
    {
        println!("--- rc_a é criado ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Contagem das referências de rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a é clonado para rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Contagem das referências de rc_b: {}", Rc::strong_count(&rc_b));
            println!("Contagem das referências de rc_a: {}", Rc::strong_count(&rc_a));
            
            // Duas 'Rc' são iguais se o valor apontado por elas for igual
            println!("rc_a e rc_b são iguais: {}", rc_a.eq(&rc_b));
            
            // Podemos usar métodos do valor apontado diretamente
            println!("Tamanho do valor apontado por rc_a: {}", rc_a.len());
            println!("Valor apontado por rc_b: {}", rc_b);
            
            println!("--- rc_b sai do escopo e é destruído ---");
        }
        
        println!("Contagem das referências de rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a sai do escopo e é destruído ---");
    }
    
    // ERRO! 'rc_examples' foi movido para 'rc_a'
    // e quanto 'rc_a' foi destruído, 'rc_examples' foi destruído também
    // println!("rc_examples: {}", rc_examples);
    // PARA VER O ERRO DESCOMENTE A LINHA ACIMA
}
```

Veja também:

[std::rc](https://doc.rust-lang.org/std/rc/index.html)
[std::sync::arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

---

## Arc

Quando a propriedade compartilhada entre threads é necessária, ```Arc``` (Atomically Reference Counted) pode ser usado. Esta estrutura, por meio da implementação ```Clone```, pode criar um ponteiro de referência para a localização de um valor na memória heap enquanto aumenta o contador de referência. Como compartilha a propriedade entre threads, quando o último ponteiro de referência para um valor sair do escopo, a variável é descartada.

```
// rbe_arc

use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    // Aqui o valor é especificado
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Aqui o valor não é especificado porque é um ponteiro para
        // uma referência na memória heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // Como 'Arc' foi usado, threads podem ser criadas usando o valor
            // usado como variável 'Arc'
            println!("{:?}", apple);
        });
    }

    // Certifique-se de que todas as instâncias do Arc sejam 
    // impressas a partir das threads gerados.
    thread::sleep(Duration::from_secs(1));
}
```

---

## asd

asd

---

## Referências

[Rust by Example (RBE)](https://doc.rust-lang.org/rust-by-example/index.html)

[RBE - Option](https://doc.rust-lang.org/rust-by-example/std/option.html)

[RBE - Result](https://doc.rust-lang.org/rust-by-example/std/result.html)

[RBE - Operador ?](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)

[RBE - panic!](https://doc.rust-lang.org/rust-by-example/std/panic.html)

[RBE - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)

[RBE - Chaves em HashMap](https://doc.rust-lang.org/rust-by-example/std/hash/alt_key_types.html)

[RBE - HashSet](https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html)

[RBE - RC](https://doc.rust-lang.org/rust-by-example/std/rc.html)

[RBE - Arc](https://doc.rust-lang.org/rust-by-example/std/arc.html)

---

arataca89@gmail.com

Última atualização: 20241222
