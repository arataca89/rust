#### arataca89

# Linguagem Rust - Concorrência

[Introdução](concorrencia.md#introdu%C3%A7%C3%A3o)

[Usando Threads para executar código simultaneamente](#usando-threads-para-executar-c%C3%B3digo-simultaneamente)

- [Criando uma nova thread com `spawn()`](#criando-uma-nova-thread-com-spawn)
- [Aguardando que todos os threads terminem usando `join()`](#aguardando-que-todos-os-threads-terminem-usando-join)
- [Usando closures `move` com threads](#usando-closures-move-com-threads)

[Usando passagem de mensagens para transferir dados entre threads](#usando-passagem-de-mensagens-para-transferir-dados-entre-threads)

- [Canais e transferência de propriedade](concorrencia.md#canais-e-transfer%C3%AAncia-de-propriedade)

- [Enviando vários valores e vendo o receptor esperando](concorrencia.md#enviando-v%C3%A1rios-valores-e-vendo-o-receptor-esperando)

- [Criando múltiplos produtores clonando o transmissor](concorrencia.md#criando-m%C3%BAltiplos-produtores-clonando-o-transmissor)

[Concorrência de estado compartilhado](concorrencia.md#concorr%C3%AAncia-de-estado-compartilhado)

- [Usando Mutexes para permitir acesso a dados de um thread por vez](concorrencia.md#usando-mutexes-para-permitir-acesso-a-dados-de-um-thread-por-vez)
- [A API do `Mutex<T>`](concorrencia.md#a-api-do-mutext)
- [Compartilhando um `Mutex<T>` entre vários threads](concorrencia.md#compartilhando-um-mutext-entre-v%C3%A1rios-threads)
- [Propriedade múltipla com múltiplos threads](concorrencia.md#propriedade-m%C3%BAltipla-com-m%C3%BAltiplos-threads)
- [Contagem de referências atômicas com `Arc<T>`](concorrencia.md#contagem-de-refer%C3%AAncias-at%C3%B4micas-com-arct)
- [Semelhanças entre `RefCell<T>`/`Rc<T>` e `Mutex<T>`/`Arc<T>`](concorrencia.md#semelhan%C3%A7as-entre-refcelltrct-e-mutextarct)

[Concorrência extensível com as traits `Sync` e `Send`](concorrencia.md#concorr%C3%AAncia-extens%C3%ADvel-com-as-traits-sync-e-send)

- [Permitindo a transferência de propriedade entre threads com `Send`](concorrencia.md#permitindo-a-transfer%C3%AAncia-de-propriedade-entre-threads-com-send)
- [Permitindo acesso de vários threads com `Sync`](concorrencia.md#permitindo-acesso-de-v%C3%A1rios-threads-com-sync)
- [Implementar `Send` e `Sync` manualmente não é seguro](concorrencia.md#implementar-send-e-sync-manualmente-n%C3%A3o-%C3%A9-seguro)

[Resumo](concorrencia.md#resumo)

---

## Introdução


Gerenciar a programação concorrente de forma segura e eficiente é outro dos principais objetivos do Rust. A programação concorrente, onde diferentes partes de um programa são executadas independentemente, e a programação paralela, onde diferentes partes de um programa são executadas ao mesmo tempo, estão se tornando cada vez mais importantes à medida que mais computadores aproveitam seus múltiplos processadores. Historicamente, a programação nesses contextos tem sido difícil e propensa a erros: o Rust espera mudar isso.

Inicialmente, a equipe do Rust pensava que garantir a segurança da memória e prevenir problemas de concorrência eram dois desafios distintos a serem resolvidos com métodos diferentes. Com o tempo, a equipe descobriu que os sistemas de propriedade e tipos são um conjunto poderoso de ferramentas para ajudar a gerenciar a segurança da memória e problemas de concorrência! Ao aproveitar a propriedade e a verificação de tipos, muitos erros de concorrência são erros de tempo de compilação em Rust, em vez de erros de tempo de execução. Portanto, em vez de fazer você gastar muito tempo tentando reproduzir as circunstâncias exatas em que um bug de concorrência de tempo de execução ocorre, o código incorreto se recusará a compilar e apresentará um erro explicando o problema. Como resultado, você pode corrigir seu código enquanto está trabalhando nele, em vez de potencialmente depois que ele foi enviado para produção. Apelidamos esse aspecto do Rust de "concorrência sem medo". A "concorrência sem medo" permite que você escreva código livre de bugs sutis e fácil de refatorar sem introduzir novos bugs.

Nota: Por simplicidade, nos referiremos a muitos dos problemas como <i>concorrentes</i> em vez de sermos mais precisos dizendo <i>concorrentes e/ou paralelos</i>. Se este livro fosse sobre concorrência e/ou paralelismo, seríamos mais específicos. Para este capítulo, por favor, substitua mentalmente <i>concorrente e/ou paralelo</i> sempre que usarmos <i>concorrente</i>. 

Muitas linguagens são dogmáticas sobre as soluções que oferecem para lidar com problemas concorrentes. Por exemplo, Erlang tem funcionalidade elegante para concorrência de passagem de mensagens, mas tem apenas maneiras obscuras de compartilhar estado entre threads. Suporte apenas a um subconjunto de soluções possíveis é uma estratégia razoável para linguagens de alto nível, porque uma linguagem de alto nível promete benefícios de abrir mão de algum controle para ganhar abstrações. No entanto, espera-se que linguagens de baixo nível forneçam a solução com o melhor desempenho em qualquer situação e tenham menos abstrações sobre o hardware. Portanto, Rust oferece uma variedade de ferramentas para modelar problemas de qualquer maneira que seja apropriada para sua situação e requisitos.

Aqui estão os tópicos que abordaremos neste capítulo: 

* Como criar threads para executar vários pedaços de código ao mesmo tempo. 
* Concorrência de passagem de mensagens, onde canais enviam mensagens entre threads.
* Concorrência de estado compartilhado, onde múltiplas threads têm acesso a um pedaço de dados.
* As traits `Sync` e `Send`, que estendem as garantias de concorrência do Rust para tipos definidos pelo usuário, bem como tipos fornecidos pela biblioteca padrão. 


---

## Usando Threads para executar código simultaneamente

Na maioria dos sistemas operacionais atuais, o código de um programa executado é executado em um processo, e o sistema operacional gerencia vários processos ao mesmo tempo. Dentro de um programa, você também pode ter partes independentes que são executadas simultaneamente. Os recursos que executam essas partes independentes são chamados de `threads`. Por exemplo, um servidor web pode ter várias threads para que possa responder a mais de uma solicitação ao mesmo tempo.

Dividir a computação no seu programa em vários threads para executar várias tarefas ao mesmo tempo pode melhorar o desempenho, mas também adiciona complexidade. Como os threads podem ser executados simultaneamente, não há garantia inerente sobre a ordem em que partes do seu código em diferentes threads serão executadas. Isso pode levar a problemas, como:

* Condições de corrida, onde os threads acessam dados ou recursos em uma ordem inconsistente.
* Deadlocks, onde dois threads estão esperando um pelo outro, impedindo que ambos os threads continuem.
* Bugs que ocorrem apenas em certas situações e são difíceis de reproduzir e corrigir de forma confiável.
 
Rust tenta mitigar os efeitos negativos do uso de threads, mas programar em um contexto multithread ainda exige reflexão cuidadosa e requer uma estrutura de código diferente daquela dos programas executados em um único thread.

Linguagens de programação implementam threads de algumas maneiras diferentes, e muitos sistemas operacionais fornecem uma API que a linguagem pode chamar para criar novos threads. A biblioteca padrão Rust usa um modelo 1:1 de implementação de thread, em que um programa usa um thread do sistema operacional por um thread da linguagem. Existem crates que implementam outros modelos de threading que fazem diferentes compensações para o modelo 1:1.

### Criando uma nova thread com `spawn()` 

Para criar um novo thread, chamamos a função `thread::spawn()` e passamos a ela uma closure contendo o código que queremos executar no novo thread. O exemplo abaixo imprime algum texto de um thread principal e outro texto de um novo thread:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Número {i} a partir da thread criada com spawn()!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Número {i} a partir da thread principal!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Note que quando o thread principal de um programa Rust é concluído, todos os threads gerados são desligados, independentemente de terem ou não terminado de executar. A saída deste programa pode ser um pouco diferente a cada vez, mas será semelhante ao seguinte:

```
Número 1 a partir da thread principal!
Número 1 a partir da thread criada com spawned()!
Número 2 a partir da thread principal!
Número 2 a partir da thread criada com spawned()!
Número 3 a partir da thread principal!
Número 3 a partir da thread criada com spawned()!
Número 4 a partir da thread principal!
Número 4 a partir da thread criada com spawned()!
```

As chamadas a `thread::sleep()` forçam um thread a parar sua execução por um curto período, permitindo que um thread diferente seja executado. Os threads provavelmente se revezarão, mas isso não é garantido: depende de como seu sistema operacional agenda os threads. Nesta execução, o thread principal imprimiu primeiro, embora a declaração print do thread gerado apareça primeiro no código. E embora tenhamos dito ao thread gerado para imprimir até que i seja 9, ele só chegou a 4 antes do thread principal desligar.

Se você executar este código e só ver a saída do thread principal, ou não observar nenhuma sobreposição, tente aumentar os números nos intervalos para criar mais oportunidades para o sistema operacional alternar entre os threads.

### Aguardando que todos os threads terminem usando `join()`

O código acima não apenas interrompe o thread gerado prematuramente na maioria das vezes devido ao término do thread principal, mas também não dá garantia sobre a ordem em que os threads são executados, nem podemos garantir que o thread gerado será executado!

Podemos corrigir o problema do thread gerado não rodar ou terminar prematuramente salvando o valor de retorno de `thread::spawn()` em uma variável. O tipo de retorno de `thread::spawn()` é `JoinHandle`. Um `JoinHandle` é um valor proprietário que, quando chamamos o método `join()` nele, esperará que seu thread termine. O código abaixo mostra como usar o `JoinHandle` do thread que criamos e chamar `join()` para garantir que o thread gerado termine antes que `main()` saia:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Número {i} a partir da thread criada com spawned()!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Número {i} a partir da thread principal!");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
```

Chamar `join()` no handle bloqueia o thread em execução até que o thread representado pelo handle termine. Bloquear um thread significa que o thread é impedido de executar trabalho ou sair. Como colocamos a chamada para `join()` após o loop for do thread principal, a execução deste código deve produzir uma saída semelhante a:

```
Número 1 a partir da thread principal!
Número 1 a partir da thread criada com spawned()!
Número 2 a partir da thread principal!
Número 2 a partir da thread criada com spawned()!
Número 3 a partir da thread principal!
Número 3 a partir da thread criada com spawned()!
Número 4 a partir da thread principal!
Número 4 a partir da thread criada com spawned()!
Número 5 a partir da thread criada com spawned()!
Número 6 a partir da thread criada com spawned()!
Número 7 a partir da thread criada com spawned()!
Número 8 a partir da thread criada com spawned()!
Número 9 a partir da thread criada com spawned()!
```

Os dois threads continuam se alternando, mas o thread principal espera por causa da chamada a `handle.join()` e não termina até que o thread gerado termine.

Mas vamos ver o que acontece quando movemos `handle.join()` antes do loop for em `main()`, assim:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Número {i} a partir da thread criada com spawned()!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    
    for i in 1..5 {
        println!("Número {i} a partir da thread principal!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

O thread principal aguardará que o thread gerado termine e então executará seu loop for, então a saída não será mais intercalada, como mostrado aqui:

```
Número 1 a partir da thread criada com spawned()!
Número 2 a partir da thread criada com spawned()!
Número 3 a partir da thread criada com spawned()!
Número 4 a partir da thread criada com spawned()!
Número 5 a partir da thread criada com spawned()!
Número 6 a partir da thread criada com spawned()!
Número 7 a partir da thread criada com spawned()!
Número 8 a partir da thread criada com spawned()!
Número 9 a partir da thread criada com spawned()!
Número 1 a partir da thread principal!
Número 2 a partir da thread principal!
Número 3 a partir da thread principal!
Número 4 a partir da thread principal!
```

Pequenos detalhes, como onde `join()` é chamado, podem afetar se seus threads serão executados ou não ao mesmo tempo.

### Usando closures `move` com threads

Frequentemente usaremos a palavra-chave `move` com closures passadas ​​para `thread::spawn()` porque a closure então tomará posse dos valores que ela usa do ambiente, transferindo assim a propriedade desses valores de um thread para outro. Na seção [Capturando referências ou movendo propriedade](closures.md#3-capturando-referências-ou-movendo-propriedade) do Capítulo [Closures](closures.md#arataca89), discutimos `move` no contexto de closures. Agora, nos concentraremos mais na interação entre `move` e `thread::spawn()`.

No exemplo anterior a closure que passamos para `thread::spawn()` não recebe argumentos: não estamos usando nenhum dado do thread principal no código do thread gerado. Para usar dados do thread principal no thread gerado, a closure do thread gerado deve capturar os valores de que precisa. Abaixo temos uma tentativa de criar um vetor no thread principal e usá-lo no thread gerado. No entanto, isso ainda não funcionará, como você verá em um momento.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Vetor v: {v:?}");
    });

    handle.join().unwrap();
}
```

A closure usa `v`, então ela irá capturar `v` e torná-lo parte do ambiente da closure. Como `thread::spawn()` executa essa closure em um novo thread, devemos ser capazes de acessar `v` dentro desse novo thread. Mas quando compilamos esse exemplo, obtemos o seguinte erro:

```
$ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {v:?}");
  |                                     - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("Here's a vector: {v:?}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++

For more information about this error, try `rustc --explain E0373`.
error: could not compile `threads` (bin "threads") due to 1 previous error
```

Rust infere como capturar `v`, e como `println!` só precisa de uma referência a `v`, a closure tenta pegar emprestado `v`. No entanto, há um problema: Rust não consegue dizer por quanto tempo o thread gerado será executado, então ele não sabe se a referência a `v` será sempre válida.

Abaixo temos um cenário que tem mais probabilidade de ter uma referência a `v` que não será válida:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Vetor v: {v:?}");
    });

    drop(v); // ERRO

    handle.join().unwrap();
}
```

Se Rust nos permitisse executar esse código, há uma possibilidade de que o thread gerado fosse imediatamente colocado em segundo plano sem ser executado. O thread gerado tem uma referência a `v` dentro, mas o thread principal imediatamente descarta `v`, usando a função `drop()`. Então, quando o thread gerado começa a executar, `v` não é mais válido, então uma referência a ele também é inválida, o que é um erro.

Para corrigir este erro, podemos usar o conselho das mensagens de erro:

```
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Ao adicionar a palavra-chave `move` antes da closure, forçamos a closure a assumir a propriedade dos valores que está usando em vez de permitir que Rust infira que ela deve pegar emprestado os valores. A modificação feita abaixo será compilada e executada conforme pretendemos:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vetor v: {v:?}");
    });

    handle.join().unwrap();
}
```

Podemos ficar tentados a fazer a mesma coisa para consertar o código onde o thread principal chamou `drop()` usando uma closure `move`. No entanto, essa correção não funcionará porque o que aquele código está tentando fazer não é permitido por um motivo diferente. Se adicionássemos `move` a closure, moveríamos `v` para o ambiente da closure e não poderíamos mais chamar `drop()` nele no thread principal. Em vez disso, obteríamos este erro do compilador:

```
$ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
4  |     let v = vec![1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
5  |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved into closure here
7  |         println!("Here's a vector: {v:?}");
   |                                     - variable moved due to use in closure
...
10 |     drop(v); // oh no!
   |          ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `threads` (bin "threads") due to 1 previous error
```

As regras de propriedade do Rust nos salvaram novamente! Inicialmente recebemos um erro do código porque o Rust estava sendo conservador e apenas pegando emprestado `v` para o thread, o que significava que o thread principal poderia teoricamente invalidar a referência do thread gerado. Ao dizer ao Rust para mover a propriedade de `v` para o thread gerado, estamos garantindo ao Rust que o thread principal não usará mais `v`. Se alterarmos a listagem com `drop()` da mesma forma, estaremos violando as regras de propriedade quando tentarmos usar `v` no thread principal. A palavra-chave `move` substitui o padrão conservador de empréstimo do Rust; ela não nos deixa violar as regras de propriedade.

Com um entendimento básico de threads e da API de threads, vamos ver o que podemos fazer com threads.

---

## Usando passagem de mensagens para transferir dados entre threads

Uma abordagem cada vez mais popular para garantir a simultaneidade segura é a passagem de mensagens, onde threads ou atores se comunicam enviando uns aos outros mensagens contendo dados. Aqui está a ideia em um slogan da documentação da linguagem Go: "Não se comunique compartilhando memória; em vez disso, compartilhe memória comunicando."

Para realizar a simultaneidade de envio de mensagens, a biblioteca padrão do Rust fornece uma implementação de canais. Um canal é um conceito geral de programação pelo qual os dados são enviados de uma thread para outra.

Você pode imaginar um canal na programação como sendo um canal direcional de água, como um riacho ou um rio. Se você colocar algo como um pato de borracha em um rio, ele viajará rio abaixo até o fim do curso d'água.

Um canal tem duas metades: um transmissor e um receptor. A metade do transmissor é o local a montante onde você coloca os patos de borracha no rio, e a metade do receptor é onde o pato de borracha termina rio abaixo. Uma parte do seu código chama métodos no transmissor com os dados que você deseja enviar, e outra parte verifica a extremidade receptora para mensagens que chegam. Diz-se que um canal está fechado se a metade transmissora ou receptora for descartada.

Aqui, trabalharemos em um programa que tem um thread para gerar valores e enviá-los por um canal, e outro thread que receberá os valores e os imprimirá. Enviaremos valores simples entre threads usando um canal para ilustrar o recurso. Depois que estiver familiarizado com a técnica, você pode usar canais para quaisquer threads que precisem se comunicar entre si, como um sistema de bate-papo ou um sistema em que muitos threads realizam partes de um cálculo e enviam as partes para um thread que agrega os resultados.

Primeiro, no código abaixo, criaremos um canal, mas não faremos nada com ele. Observe que isso não será compilado ainda porque o Rust não consegue dizer que tipo de valores queremos enviar pelo canal.

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

Criamos um novo canal usando a função `mpsc::channel()`; **mpsc** significa <i> multiple producer, single consumer</i> (produtor múltiplo, consumidor único). Em resumo, a maneira como a biblioteca padrão do Rust implementa canais significa que um canal pode ter várias extremidades de envio que produzem valores, mas apenas uma extremidade de recebimento que consome esses valores. Imagine vários fluxos fluindo juntos em um grande rio: tudo enviado por qualquer um dos fluxos terminará em um rio no final. Começaremos com um único produtor por enquanto, mas adicionaremos vários produtores quando fizermos este exemplo funcionar.

A função `mpsc::channel()` retorna uma tupla, o primeiro elemento da qual é a extremidade de envio — o transmissor — e o segundo elemento é a extremidade de recebimento — o receptor. As abreviações `tx` e `rx` são tradicionalmente usadas em muitos campos para transmissor e receptor, respectivamente, então nomeamos nossas variáveis ​​como tal para indicar cada extremidade. Estamos usando uma instrução `let` com um padrão que desestrutura as tuplas; Para mais informações sobre padrões consulte [Padrões e correspondência de padrões](patterns.md#arataca89). Por enquanto, saiba que usar uma instrução `let` dessa forma é uma abordagem conveniente para extrair as partes da tupla retornada por `mpsc::channel()`.

Vamos mover a extremidade transmissora para um thread gerado e fazer com que ele envie uma string para que o thread gerado esteja se comunicando com o thread principal, conforme mostrado abaixo. Isso é como colocar um pato de borracha no rio rio acima ou enviar uma mensagem de bate-papo de um thread para outro.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Mensagem");
        tx.send(val).unwrap();
    });
}
```

Novamente, estamos usando `thread::spawn()` para criar um novo thread e então usando `move` para mover `tx` para a closure de modo que o thread gerado tenha a propriedade de `tx`. O thread gerado precisa ter a propriedade do transmissor para poder enviar mensagens pelo canal. O transmissor tem um método `send()` que pega o valor que queremos enviar. O método `send()` retorna um tipo `Result<T, E>`, então se o receptor já foi descartado e não há para onde enviar um valor, a operação **send** retornará um erro. Neste exemplo, estamos chamando `unwrap()` para entrar em pânico em caso de erro. Mas em uma aplicação real, nós lidaríamos com isso corretamente. Para mais informações sobre tratamento de erros consulte [Tratamento de erro](erro.md#arataca89).

No código abaixo, obteremos o valor do receptor no thread principal. Isso é como recuperar o patinho de borracha da água no fim do rio ou receber uma mensagem de bate-papo.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Mensagem");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Recebido: {received}");
}
```

O receptor tem dois métodos úteis: `recv()` e `try_recv()`. Estamos usando `recv()`, abreviação de <i>receive</i> (receber), que bloqueará a execução do thread principal e esperará até que um valor seja enviado pelo canal. Depois que um valor for enviado, `recv()` o retornará em um `Result<T, E>`. Quando o transmissor fechar, `recv()` retornará um erro para sinalizar que nenhum outro valor será recebido.

O método `try_recv()` não bloqueia, mas retornará um `Result<T, E>` imediatamente: um valor `Ok` contendo uma mensagem se houver uma disponível e um valor `Err` se não houver nenhuma mensagem desta vez. Usar `try_recv()` é útil se este thread tiver outro trabalho a fazer enquanto espera por mensagens: poderíamos escrever um loop que chama `try_recv()` de vez em quando, manipula uma mensagem se houver uma disponível e, de outra forma, faz outro trabalho por um tempo até verificar novamente.

Usamos `recv()` neste exemplo para simplificar; não temos nenhum outro trabalho para o thread principal fazer além de esperar por mensagens, então bloquear o thread principal é apropriado.

Quando executarmos o código acima, veremos o valor impresso do thread principal:

```
Recebido: Mensagem
```


### Canais e transferência de propriedade

As regras de propriedade desempenham um papel vital no envio de mensagens porque ajudam você a escrever código seguro e concorrente. Prevenir erros na programação concorrente é a vantagem de pensar sobre propriedade em todos os seus programas Rust. Vamos fazer um experimento para mostrar como canais e propriedade trabalham juntos para evitar problemas: tentaremos usar um valor `val` no thread gerado depois de enviá-lo pelo canal. Tente compilar o código abaixo para ver por que esse código não é permitido:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Mensagem");
        tx.send(val).unwrap();
        println!("val: {val}");
    });

    let received = rx.recv().unwrap();
    println!("Recebido: {received}");
}
```

Aqui, tentamos imprimir `val` depois de enviá-lo pelo canal via `tx.send()`. Permitir isso seria uma má ideia: uma vez que o valor foi enviado para outro thread, esse thread poderia modificá-lo ou descartá-lo antes de tentarmos usar o valor novamente. Potencialmente, as modificações do outro thread poderiam causar erros ou resultados inesperados devido a dados inconsistentes ou inexistentes. No entanto, Rust nos dá um erro se tentarmos compilar o código:

```
Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:24
   |
8  |         let val = String::from("Mensagem");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val: {val}");
   |                        ^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground` (bin "playground") due to 1 previous error
Standard Output
```

Nosso erro de simultaneidade causou um erro de tempo de compilação. A função `send()` assume a propriedade de seu parâmetro e, quando o valor é movido, o receptor assume a propriedade dele. Isso nos impede de usar acidentalmente o valor novamente após enviá-lo; o sistema de propriedade verifica se está tudo bem.


### Enviando vários valores e vendo o receptor esperando

O código da [introdução](concorrencia.md#introdu%C3%A7%C3%A3o) foi compilado e executado, mas não nos mostrou claramente que dois threads separados estavam se falando pelo canal. No código abaixo, fizemos algumas modificações que provarão que o código da introdução está sendo executado simultaneamente: o thread gerado agora enviará várias mensagens e pausará por um segundo entre cada mensagem.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Mensagem"),
            String::from("enviada"),
            String::from("a partir da"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Recebido: {received}");
    }
}
```

Desta vez, o thread gerado tem um vetor de strings que queremos enviar para o thread principal. Nós iteramos sobre elas, enviando cada uma individualmente, e pausamos entre cada uma chamando a função `thread::sleep()` com um valor `Duration` de 1 segundo.

No thread principal, não estamos mais chamando a função `recv()` explicitamente: em vez disso, estamos tratando `rx` como um iterador. Para cada valor recebido, estamos imprimindo-o. Quando o canal for fechado, a iteração terminará.

Ao executar este código, você deve ver a seguinte saída com uma pausa de 1 segundo entre cada linha:

```
Recebido: Mensagem
Recebido: enviada
Recebido: a partir da
Recebido: thread
```

Como não temos nenhum código que pause ou atrase o loop for no thread principal, podemos dizer que o thread principal está esperando para receber valores do thread gerado.

### Criando múltiplos produtores clonando o transmissor

Anteriormente, mencionamos que `mpsc` era um acrônimo para <i>multiple producers, single consumer</i>. Vamos usar `mpsc` e expandir o código na listagem da seção anterior para criar múltiplos threads que enviam valores para o mesmo receptor. Podemos fazer isso clonando o transmissor, como mostrado abaixo:

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --snip--
```

Desta vez, antes de criarmos o primeiro thread gerado, chamamos clone no transmissor. Isso nos dará um novo transmissor que podemos passar para o primeiro thread gerado. Passamos o transmissor original para um segundo thread gerado. Isso nos dá dois threads, cada um enviando mensagens diferentes para o único receptor.

Quando você executa o código, sua saída deve ser parecida com esta:

```
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

Você pode ver os valores em outra ordem, dependendo do seu sistema. É isso que torna a simultaneidade interessante e difícil. Se você experimentar `thread::sleep()`, dando a ele vários valores em diferentes threads, cada execução será mais não determinística e criará uma saída diferente a cada vez.

Agora que vimos como os canais funcionam, vamos ver um método diferente de simultaneidade.

---

## Concorrência de estado compartilhado

A passagem de mensagens é uma ótima maneira de lidar com a simultaneidade, mas não é a única. Outro método seria que vários threads acessassem os mesmos dados compartilhados. Considere esta parte do slogan da documentação da linguagem Go novamente: "não se comunique compartilhando memória".

Como seria a comunicação por compartilhamento de memória? Além disso, por que os entusiastas da passagem de mensagens alertariam para não usar o compartilhamento de memória?

De certa forma, os canais em qualquer linguagem de programação são semelhantes à propriedade única, porque uma vez que você transfere um valor por um canal, você não deve mais usar esse valor. A simultaneidade de memória compartilhada é como propriedade múltipla: vários threads podem acessar o mesmo local de memória ao mesmo tempo. Como você pode verificar em [Smart Pointers](smart_pointer.md#arataca89), onde ponteiros inteligentes tornam a propriedade múltipla possível, a propriedade múltipla pode adicionar complexidade porque esses diferentes proprietários precisam ser gerenciados. O sistema de tipos e as regras de propriedade do Rust ajudam muito a obter esse gerenciamento correto. Por exemplo, vamos dar uma olhada nos mutexes, um dos primitivos de simultaneidade mais comuns para memória compartilhada.


### Usando Mutexes para permitir acesso a dados de um thread por vez

Mutex é uma abreviação para exclusão mútua, assim, um mutex permite que apenas um thread acesse alguns dados em um dado momento. Para acessar os dados em um mutex, um thread deve primeiro sinalizar que deseja acesso pedindo para adquirir o bloqueio do mutex. O bloqueio é uma estrutura de dados que faz parte do mutex que mantém o controle de quem atualmente tem acesso exclusivo aos dados. Portanto, o mutex é descrito como guardando os dados que ele contém por meio do sistema de bloqueio.

Os mutexes têm a reputação de serem difíceis de usar porque você tem que se lembrar de duas regras:

- Você deve tentar adquirir a trava (lock) antes de usar os dados. 
- Quando terminar de usar os dados protegidos pelo mutex, você deve liberar o bloqueio para que outras threads possam adquiri-lo.

Para uma metáfora do mundo real para um mutex, imagine um painel de discussão em uma conferência com apenas um microfone. Antes que um painelista possa falar, ele tem que pedir ou sinalizar que quer usar o microfone. Quando ele pega o microfone, ele pode falar o quanto quiser e então passar o microfone para o próximo painelista que pedir para falar. Se um painelista esquecer de entregar o microfone quando terminar de usá-lo, ninguém mais poderá falar. Se o gerenciamento do microfone compartilhado der errado, o painel não funcionará como planejado!

O gerenciamento de mutexes pode ser incrivelmente complicado de acertar, e é por isso que tantas pessoas são entusiasmadas com os canais. No entanto, graças ao sistema de tipos e às regras de propriedade do Rust, você não pode errar no bloqueio e desbloqueio.

### A API do `Mutex<T>`

Como um exemplo de como usar um mutex, vamos começar usando um mutex em um contexto single-threaded, conforme mostrado abaixo: 

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

Assim como em muitos tipos, criamos um `Mutex<T>` usando a função associada `new()`. Para acessar os dados dentro do mutex, usamos o método `lock()` para adquirir o bloqueio. Esta chamada bloqueará o thread atual para que ele não possa fazer nenhum trabalho até que seja nossa vez de ter o bloqueio.

A chamada para `lock()` falharia se outro thread que segura o bloqueio entrasse em pânico. Nesse caso, ninguém jamais conseguiria obter o bloqueio, então escolhemos desembrulhar e fazer com que esse thread entre em pânico se estivermos nessa situação.

Depois de adquirir o bloqueio, podemos tratar o valor de retorno, chamado `num` neste caso, como uma referência mutável aos dados internos. O sistema de tipos garante que adquirimos um bloqueio antes de usar o valor em `m`. O tipo de `m` é `Mutex<i32>`, não `i32`, então devemos chamar `lock()` para poder usar o valor `i32`. Não podemos esquecer; o sistema de tipos não nos deixará acessar o `i32` interno de outra forma.

Como você pode suspeitar, `Mutex<T>` é um ponteiro inteligente. Mais precisamente, a chamada para `lock()` retorna um ponteiro inteligente chamado `MutexGuard`, encapsulado em um `LockResult` que manipulamos com a chamada para `unwrap()`. O ponteiro inteligente `MutexGuard` implementa `Deref` para apontar para nossos dados internos; o ponteiro inteligente também tem uma implementação `Drop` que libera o bloqueio automaticamente quando um `MutexGuard` sai do escopo, o que acontece no final do escopo interno. Como resultado, não corremos o risco de esquecer de liberar o bloqueio e bloquear o mutex de ser usado por outras threads, porque a liberação do bloqueio acontece automaticamente.

Após descartar o bloqueio, podemos imprimir o valor do mutex e ver que conseguimos alterar o `i32` interno para 6.

### Compartilhando um `Mutex<T>` entre vários threads

Agora, vamos tentar compartilhar um valor entre vários threads usando `Mutex<T>`. Vamos criar 10 threads e fazer com que cada um deles incremente um valor de contador em 1, para que o contador vá de 0 a 10. O código abaixo terá um erro do compilador, e usaremos esse erro para aprender mais sobre o uso do `Mutex<T>` e como o Rust nos ajuda a usá-lo corretamente.

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *counter.lock().unwrap());
}
```

Criamos a variável `counter` para manter um `i32` dentro de um `Mutex<T>`, como fizemos anteriormente. Em seguida, criamos 10 threads iterando em um intervalo de números. Usamos `thread::spawn()` e damos a todas as threads a mesma closure: uma que move o contador para a thread, adquire um bloqueio no `Mutex<T>` chamando o método `lock()` e, em seguida, adiciona 1 ao valor no mutex. Quando um thread termina de executar sua closure, `num` sairá do escopo e liberará o bloqueio para que outra thread possa adquiri-lo.

No thread principal, coletamos todos os join handles. Então, como fizemos no código anterior, chamamos `join()` em cada handle para garantir que todos os threads terminem. Nesse ponto, o thread principal adquirirá o bloqueio e imprimirá o resultado deste programa.

Demos a entender que este exemplo não compilaria. Agora vamos descobrir o porquê!

```
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:21:29
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
8  |     for _ in 0..10 {
   |     -------------- inside of this loop
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved into closure here, in previous iteration of loop
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move
   |
help: consider moving the expression out of the loop so it is only moved once
   |
8  ~     let mut value = counter.lock();
9  ~     for _ in 0..10 {
10 |         let handle = thread::spawn(move || {
11 ~             let mut num = value.unwrap();
   |

For more information about this error, try `rustc --explain E0382`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error
```

A mensagem de erro afirma que o valor do contador foi movido na iteração anterior do loop. Rust está nos dizendo que não podemos mover a propriedade do contador para vários threads. Vamos corrigir o erro do compilador com um método de propriedade múltipla que discutimos no [capítulo sobre smart pointers](smart_pointer.md#arataca89).

### Propriedade múltipla com múltiplos threads

No [capítulo sobre smart pointers](smart_pointer.md#arataca89), demos a um valor múltiplos proprietários usando o ponteiro inteligente `Rc<T>` para criar um valor de referência contado. Vamos fazer o mesmo aqui e ver o que acontece. Vamos encapsular o `Mutex<T>` em `Rc<T>` e clonar o `Rc<T>` antes de mover a propriedade para o thread.

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *counter.lock().unwrap());
}
```

Mais uma vez, compilamos e obtemos… erros diferentes! O compilador está nos ensinando muito.

```
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:11:36
    |
11  |           let handle = thread::spawn(move || {
    |                        ------------- ^------
    |                        |             |
    |  ______________________|_____________within this `{closure@src/main.rs:11:36: 11:43}`
    | |                      |
    | |                      required by a bound introduced by this call
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    |
    = help: within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`, which is required by `{closure@src/main.rs:11:36: 11:43}: Send`
note: required because it's used within this closure
   --> src/main.rs:11:36
    |
11  |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^
note: required by a bound in `spawn`
   --> file:///home/.rustup/toolchains/1.82/lib/rustlib/src/rust/library/std/src/thread/mod.rs:675:8
    |
672 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    |        ----- required by a bound in this function
...
675 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error
```

Uau, essa mensagem de erro é muito prolixa! Aqui está a parte importante para focar: `Rc<Mutex<i32>>` não pode ser enviado entre threads com segurança. O compilador também está nos dizendo o motivo: a trait `Send` não é implementada para `Rc<Mutex<i32>>`. Falaremos sobre `Send` na próxima seção: é uma das traits que garante que os tipos que usamos com threads sejam destinados ao uso em situações simultâneas.

Infelizmente, `Rc<T>` não é seguro para compartilhar entre threads. Quando `Rc<T>` gerencia a contagem de referência, ele adiciona à contagem para cada chamada a `clone()` e subtrai da contagem quando cada clone é descartado. Mas ele não usa nenhuma primitiva de simultaneidade para garantir que as alterações na contagem não possam ser interrompidas por outro thread. Isso pode levar a contagens erradas — bugs sutis que podem, por sua vez, levar a vazamentos de memória ou um valor sendo descartado antes de terminarmos com ele. O que precisamos é de um tipo exatamente como `Rc<T>`, mas que faça alterações na contagem de referências de forma segura para threads.

### Contagem de referências atômicas com `Arc<T>`

Felizmente, `Arc<T>` é um tipo como `Rc<T>` que é seguro para uso em situações concorrentes. O `a` significa atomic, o que significa que é um tipo contado por referência atômica. `Atomics` são um tipo adicional de primitiva de simultaneidade que não abordaremos em detalhes aqui: veja a documentação da biblioteca padrão para [std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic/index.html) para mais detalhes. Neste ponto, você só precisa saber que atomics funcionam como tipos primitivos, mas são seguros para compartilhar entre threads.

Você pode então se perguntar por que todos os tipos primitivos não são atômicos e por que os tipos de biblioteca padrão não são implementados para usar `Arc<T>` por padrão. O motivo é que a segurança de thread vem com uma penalidade de desempenho que você só quer pagar quando realmente precisa. Se você estiver apenas executando operações em valores dentro de uma única thread, seu código pode ser executado mais rápido se não tiver que impor as garantias que os atômicos fornecem.

Vamos retornar ao nosso exemplo: `Arc<T>` e `Rc<T>` têm a mesma API, então corrigimos nosso programa alterando a linha `use`, a chamada para `new()` e a chamada para `clone()`. O código, então, finalmente será compilado e executado:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *counter.lock().unwrap());
}
```

Este código imprimirá o seguinte: 

```
Resultado: 10
```

Conseguimos! Contamos de 0 a 10, o que pode não parecer muito impressionante, mas nos ensinou muito sobre `Mutex<T>` e segurança de threads. Você também pode usar a estrutura deste programa para fazer operações mais complicadas do que apenas incrementar um contador. Usando esta estratégia, você pode dividir um cálculo em partes independentes, dividir essas partes entre threads e, em seguida, usar um `Mutex<T>` para que cada thread atualize o resultado final com sua parte.

Observe que, se você estiver fazendo operações numéricas simples, há tipos mais simples do que os tipos `Mutex<T>` fornecidos pelo módulo [std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic/index.html) da biblioteca padrão. Esses tipos fornecem acesso atômico, concorrente e seguro a tipos primitivos. Escolhemos usar `Mutex<T>` com um tipo primitivo para este exemplo, para que pudéssemos nos concentrar em como o `Mutex<T>` funciona.

### Semelhanças entre `RefCell<T>`/`Rc<T>` e `Mutex<T>`/`Arc<T>`

Você pode ter notado que o contador é imutável, mas poderíamos obter uma referência mutável para o valor dentro dele; isso significa que `Mutex<T>` fornece mutabilidade interna, como a família `Cell` faz. Da mesma forma que usamos `RefCell<T>` no [capítulo sobre smart pointers](smart_pointer.md#arataca89) para nos permitir alterar conteúdos dentro de um `Rc<T>`, usamos `Mutex<T>` para alterar conteúdos dentro de um `Arc<T>`.

Outro detalhe a ser observado é que Rust não pode protegê-lo de todos os tipos de erros de lógica quando você usa `Mutex<T>`. Lembre-se no [capítulo sobre smart pointers](smart_pointer.md#arataca89) que usar `Rc<T>` veio com o risco de criar ciclos de referência, onde dois valores `Rc<T>` se referem um ao outro, causando vazamentos de memória. Da mesma forma, `Mutex<T>` vem com o risco de criar deadlocks. Eles ocorrem quando uma operação precisa bloquear dois recursos e dois threads adquiriram, cada um, um dos bloqueios, fazendo com que eles esperem um pelo outro para sempre. Se você estiver interessado em deadlocks, tente criar um programa Rust que tenha um deadlock; então pesquise estratégias de mitigação de deadlock para mutexes em qualquer linguagem e tente implementá-las em Rust. A documentação da API da biblioteca padrão para `Mutex<T>` e `MutexGuard` oferece informações úteis.

Concluiremos este capítulo falando sobre as traits `Send` e `Sync` e como podemos usá-los com tipos personalizados.

---

## Concorrência extensível com as traits `Sync` e `Send`

Curiosamente, a linguagem Rust tem muito poucos recursos de simultaneidade. Quase todos os recursos de simultaneidade sobre os quais falamos até agora neste capítulo fazem parte da biblioteca padrão, não da linguagem. Suas opções para lidar com a simultaneidade não se limitam à linguagem ou à biblioteca padrão; você pode escrever seus próprios recursos de simultaneidade ou usar aqueles escritos por outros.

No entanto, dois conceitos de simultaneidade são incorporados na linguagem: as traits `std::marker` `Sync` e `Send`.

### Permitindo a transferência de propriedade entre threads com `Send`

A trait `Send` indica que a propriedade de valores do tipo que implementa `Send` pode ser transferida entre threads. Quase todo tipo Rust é `Send`, mas há algumas exceções, incluindo `Rc<T>`: que não pode ser `Send` porque se você clonou um valor `Rc<T>` e tentou transferir a propriedade do clone para outro thread, ambos os threads podem atualizar a contagem de referência ao mesmo tempo. Por esse motivo, `Rc<T>` é implementado para uso em situações de thread única em que você não quer pagar a penalidade de desempenho de thread-safe.

Portanto, o sistema de tipos e os limites de trait do Rust garantem que você nunca pode enviar acidentalmente um valor `Rc<T>` entre threads de forma insegura. Quando tentamos fazer isso no código em [Propriedade múltipla com múltiplos threads](concorrencia.md#propriedade-m%C3%BAltipla-com-m%C3%BAltiplos-threads), obtivemos o erro `the trait Send is not implemented for Rc<Mutex<i32>>`. Quando mudamos para `Arc<T>`, que é `Send`, o código foi compilado.

Qualquer tipo composto inteiramente de tipos `Send` é automaticamente marcado como `Send` também. Quase todos os tipos primitivos são `Send`, além dos ponteiros brutos, que discutiremos no capítulo [Recursos avançados](recursos_avancados.md#arataca89).

### Permitindo acesso de vários threads com `Sync`

A trait `Sync` indica que é seguro para o tipo que implementa `Sync` ser referenciado de vários threads. Em outras palavras, qualquer tipo `T` é `Sync` se `&T` (uma referência imutável a `T`) for `Send`, o que significa que a referência pode ser enviada com segurança para outro thread. Semelhante a `Send`, tipos primitivos são `Sync`, e tipos compostos inteiramente de tipos que são `Sync` também são `Sync`.

O ponteiro inteligente `Rc<T>` também não é `Sync` pelos mesmos motivos que não é `Send`. O tipo `RefCell<T>` (sobre o qual falamos no [capítulo sobre smart pointers](smart_pointer.md#arataca89)) e a família de tipos `Cell<T>` relacionados não são `Sync`. A implementação da verificação de empréstimo que `RefCell<T>` faz em tempo de execução não é segura para threads. O ponteiro inteligente `Mutex<T>` é `Sync` e pode ser usado para compartilhar acesso com vários threads, como você viu na seção [Compartilhando um `Mutex<T>` entre vários threads](concorrencia.md#compartilhando-um-mutext-entre-v%C3%A1rios-threads).

### Implementar `Send` e `Sync` manualmente não é seguro

Como os tipos compostos que são `Send` e `Sync` são automaticamente também `Send` e `Sync`, não precisamos implementar essas traits manualmente. Como traits de marcadores, elas nem mesmo têm métodos para implementar. Elas são úteis apenas para impor invariantes relacionadas à simultaneidade.

A implementação manual dessas traits envolve a implementação de código Rust inseguro. Falaremos sobre o uso de código Rust inseguro no capítulo [Recursos avançados](recursos_avancados.md#arataca89); por enquanto, a informação importante é que construir novos tipos concorrentes não compostos de partes `Send` e `Sync` requer um pensamento cuidadoso para manter as garantias de segurança. [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) tem mais informações sobre essas garantias e como mantê-las.

---

## Resumo

Como mencionado anteriormente, como muito pouco de como Rust lida com simultaneidade faz parte da linguagem, muitas soluções de simultaneidade são implementadas como crates. Elas evoluem mais rapidamente do que a biblioteca padrão, então certifique-se de pesquisar on-line pelos crates atuais e de última geração para usar em situações multithread.

A biblioteca padrão Rust fornece canais para passagem de mensagens e tipos de ponteiros inteligentes, como `Mutex<T>` e `Arc<T>`, que são seguros para uso em contextos simultâneos. O sistema de tipos e o verificador de empréstimos garantem que o código que usa essas soluções não acabará com corridas de dados (data races) ou referências inválidas. Depois que seu código for compilado, você pode ter certeza de que ele será executado em vários threads sem os tipos de bugs difíceis de rastrear comuns em outras linguagens. A programação simultânea não é mais um conceito a ser temido: vá em frente e torne seus programas simultâneos, sem medo!

---

## Referências

[Capítulo 16 do Livro - Concorrência](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

arataca89@gmail.com

Última atualização: 20250226