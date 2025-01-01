#### arataca89

# Linguagem Rust - Tratamento de erro

Erros são um fato da vida em software, então Rust possui uma série de recursos para lidar com situações em que algo dá errado. Em muitos casos, Rust exige que você reconheça a possibilidade de um erro e tome alguma ação antes que seu código compile. Esse requisito torna seu programa mais robusto, garantindo que você descubra erros e os trate adequadamente antes de implantar seu código na produção! 

Rust agrupa erros em duas categorias principais: **erros recuperáveis** e **erros irrecuperáveis**. Para um erro recuperável, como um erro de arquivo não encontrado, provavelmente queremos apenas relatar o problema ao usuário e tentar novamente a operação. Erros irrecuperáveis são sempre sintomas de bugs, como tentar acessar um local além do final de um array, e por isso queremos parar o programa imediatamente.

A maioria das linguagens não distingue entre esses dois tipos de erros e lida com ambos da mesma forma, usando mecanismos como exceções. Rust não tem exceções. Em vez disso, ele tem o tipo ```Result<T, E>```para erros recuperáveis ​​e a macro ```panic!``` que interrompe a execução quando o programa encontra um erro irrecuperável.

[panic!](#panic)

[Result](#result)

[Fazendo match em diferentes erros](#fazendo-match-em-diferentes-erros)

[Alternativas ao uso de match](#alternativas-ao-uso-de-match)

[unwrap e expect](#unwrap-e-expect)

[propagando erros](#propagando-erros)

---

## panic!

Às vezes, coisas ruins acontecem no seu código e não há nada que você possa fazer sobre isso. Nesses casos, o Rust tem a macro ```panic!```. Existem duas maneiras de causar um pânico na prática: realizando uma ação que faz com que nosso código entre em pânico (como acessar um array após o fim) ou chamando explicitamente a macro ```panic!```. Em ambos os casos, causamos um pânico em nosso programa. Por padrão, esses pânicos imprimirão uma mensagem de erro, a pilha será percorrida e limpa, e o programa será encerrado. Por meio de uma variável de ambiente, você também pode fazer com que o Rust exiba a pilha de chamadas quando ocorrer um pânico para facilitar o rastreamento da fonte do pânico.

#### Desmontando a pilha ou abortando em resposta a um Pânico

Por padrão, quando um pânico ocorre, o programa começa a desmontar a pilha, o que significa que o Rust volta para cima da pilha e limpa os dados de cada função que encontra. No entanto, voltar e limpar é muito trabalhoso. Portanto, o Rust permite que você escolha a alternativa de abortar imediatamente, o que encerra o programa sem limpar a pilha. 

A memória que o programa estava usando precisará ser limpa pelo sistema operacional. Se no seu projeto você precisa tornar o binário resultante o menor possível, você pode mudar do comportamento padrão de desmontar a pilha para abortar imediatamente em caso de pânico adicionando ```panic = 'abort'``` às seções ```[profile]``` apropriadas no seu arquivo ```Cargo.toml```. Por exemplo, se você quiser abortar em caso de pânico no modo release, adicione isto:

```
[profile.release]
panic = 'abort'
```

#### Emitindo um pânico

Vamos tentar chamar ```panic!``` em um programa simples:

```
fn main() {
    panic!("emitindo um panic!");
}
```

Quando você executar o programa, verá algo como isto: 

```
Compiling playground v0.0.1 (/playground)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/playground`
thread 'main' panicked at src/main.rs:2:5:
emitindo um panic!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

A chamada a ```panic!``` emite a mensagem de erro contida nas duas últimas linhas. A primeira linha mostra nossa mensagem de pânico e o local em nosso código-fonte onde o pânico ocorreu: **src/main.rs:2:5** indica que é a segunda linha, quinto caractere do nosso arquivo **src/main.rs**.

Neste caso, a linha indicada faz parte do nosso código, e se formos para essa linha, vemos a chamada da macro ```panic!```. Em outros casos, a chamada a ```panic!``` pode estar em código que nosso código chama, e o nome do arquivo e o número da linha relatados pela mensagem de erro serão de código de outra pessoa onde a macro ```panic!``` é chamada, não a linha do nosso código que eventualmente levou à chamada ```panic!```.

Podemos usar o rastreamento da pilha de funções de onde a chamada ```panic!``` veio para descobrir a parte do nosso código que está causando o problema. Para entender como usar um rastreamento de pilha com ```panic!```, vamos olhar para outro exemplo e ver como é quando uma chamada a ```panic!``` vem de uma biblioteca por causa de um bug no nosso código em vez de do nosso código chamando a macro diretamente. O trecho de código abaixo tenta acessar um índice em um vetor além do intervalo de índices válidos.

```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Aqui, estamos tentando acessar o 100º elemento do nosso vetor (que está no índice 99 porque a indexação começa em zero), mas o vetor tem apenas três elementos. Nesta situação, o Rust entrará em pânico. O uso de ```[]``` deve retornar um elemento, mas se você passar um índice inválido, não há elemento que o Rust possa retornar aqui que esteja correto.

Em C, tentar ler além do final de uma estrutura de dados provoca um comportamento indefinido. Você pode obter o que estiver no local da memória que corresponderia a esse elemento na estrutura de dados, mesmo que a memória não pertença a essa estrutura. Isso é chamado de estouro de buffer e pode levar a vulnerabilidades de segurança se um atacante conseguir manipular o índice de forma a ler dados que não deveriam ser permitidos e que são armazenados após a estrutura de dados.

Para proteger seu programa desse tipo de vulnerabilidade, se você tentar ler um elemento em um índice que não existe, Rust interromperá a execução e se recusará a continuar. Vamos tentar e ver:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Este erro aponta para a linha 4 do nosso **main.rs**, onde tentamos acessar o índice **99** do vetor em **v**.

A linha **note:** nos diz que podemos definir a variável de ambiente **RUST_BACKTRACE** para obter um backtrace do que exatamente aconteceu para causar o erro. Um backtrace é uma lista de todas as funções que foram chamadas para chegar a esse ponto. Backtraces em Rust funcionam como em outras linguagens: a chave para ler o backtrace é começar do topo e ler até ver os arquivos que você escreveu. Esse é o ponto onde o problema se originou. As linhas acima desse ponto são o código que seu código chamou; as linhas abaixo são o código que chamou seu código. Essas linhas antes e depois podem incluir o código Rust principal, o código da biblioteca padrão ou crates que você está usando. Vamos tentar obter um backtrace definindo a variável de ambiente **RUST_BACKTRACE** para qualquer valor, exceto **0**.

```
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_bounds_check
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:208:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:255:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/alloc/src/vec/mod.rs:2770:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

A saída exata pode ser diferente dependendo do seu sistema operacional e versão do Rust. Para obter backtraces com essa informação, os símbolos de depuração devem ser habilitados. Os símbolos de depuração são habilitados por padrão ao usar **cargo build** ou **cargo run** sem a flag **--release**, como fizemos aqui.

A linha 6 do backtrace aponta para a linha em nosso projeto que está causando o problema: **linha 4 de src/main.rs**. Se não quisermos que nosso programa entre em pânico, devemos começar nossa investigação no local apontado pela primeira linha mencionando um arquivo que escrevemos. Neste caso, a maneira de consertar o pânico é não solicitar um elemento além do intervalo dos índices do vetor. Quando seu código entrar em pânico no futuro, você precisará descobrir qual ação o código está tomando com quais valores para causar o pânico e o que o código deve fazer em vez disso.

---

## Result

A maioria dos erros não é séria o suficiente para exigir que o programa pare completamente. Às vezes, quando uma função falha, é por um motivo que você pode interpretar e responder facilmente. Por exemplo, se você tentar abrir um arquivo e essa operação falhar porque o arquivo não existe, você pode querer criar o arquivo em vez de encerrar o processo.

```Result``` é uma enumeração da biblioteca Rust usada quando temos uma situação onde pode haver sucesso ou erro. 

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


Em caso de sucesso usa-se a variante ```Ok``` e em caso de erro usa-se a variante ```Err```.

**T** e **E** são parâmetros de tipo genérico. **T** representa o tipo do valor que será retornado em um caso de sucesso dentro da variante **Ok**, e **E** representa o tipo do erro que será retornado em um caso de falha dentro da variante **Err**. Como ```Result``` tem esses parâmetros de tipo genérico, podemos usar o tipo ```Result``` e as funções definidas nele em muitas situações diferentes em que o valor de sucesso e o valor de erro que queremos retornar podem ser diferentes.

Vamos chamar uma função que retorna um valor ```Result``´ porque a função pode falhar. No código abaixo tentamos abrir um arquivo.

```
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

O tipo de retorno de ```File::open()``` é um ```Result<T, E>```. O parâmetro genérico **T** foi preenchido pela implementação de ```File::open()``` com o tipo do valor de sucesso, ```std::fs::File```, que é um identificador de arquivo. O tipo de **E** usado no valor de erro é ```std::io::Error```. Este tipo de retorno significa que a chamada a ```File::open()``` pode ter sucesso e retornar um identificador de arquivo do qual podemos ler ou escrever. A chamada de função também pode falhar: por exemplo, o arquivo pode não existir, ou podemos não ter permissão para acessar o arquivo. A função ```File::open()``` precisa ter uma maneira de nos dizer se teve sucesso ou falhou e, ao mesmo tempo, nos dar o identificador de arquivo ou informações de erro. Essas informações são exatamente o que a enumeração ```Result``` transmite.

No caso em que ```File::open()``` for bem-sucedido, o valor na variável **greeting_file_result** será uma instância de **Ok** que contém um identificador de arquivo. No caso em que falhar, o valor em **greeting_file_result** será uma instância de **Err** que contém mais informações sobre o tipo de erro que ocorreu. 

Precisamos adicionar mais código para executar ações diferentes dependendo do valor retornado por ```File::open()```. O código abaixo  mostra uma maneira de tratar ```Result``` usando uma ferramenta básica, a expressão **match**.

```
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problema ao abrir o arquivo: {error:?}"),
    };
}
```

Observe que, assim como a enumeração ```Option```, a enumeração ```Result``` e suas variantes foram trazidas ao escopo pelo prelúdio, então não precisamos especificar ```Result::``` antes das variantes **Ok** e **Err** nos ramos de **match**.

Quando o resultado for **Ok**, esse código retornará o valor do arquivo contido na variante **Ok**, e então atribuiremos esse valor de identificador de arquivo à variável **greeting_file**. Após o **match**, podemos usar o identificador de arquivo para leitura ou escrita.

O outro braço do match lida com o caso em que obtemos um valor **Err** de ```File::open()```. Neste exemplo, escolhemos chamar a macro ```panic!```. Se não houver um arquivo chamado **hello.txt** no nosso diretório atual e executarmos este código, veremos a seguinte saída da macro ```panic!```:

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Como de costume, esta saída nos diz exatamente o que deu errado. 

---

## Fazendo match em diferentes erros

O código acima emitirá pânico independente do motivo pelo qual ```File::open()``` falhou. No entanto, queremos tomar ações diferentes para diferentes razões de falha. Se ```File::open()``` falhou porque o arquivo não existe, queremos criar o arquivo e retornar o identificador para o novo arquivo. Se ```File::open()``` falhou por qualquer outro motivo — por exemplo, porque não tínhamos permissão para abrir o arquivo — ainda queremos que o código entre em pânico.

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema ao criar o arquivo: {e:?}"),
            },
            other_error => {
                panic!("Problema ao abrir o arquivo: {other_error:?}");
            }
        },
    };
}
```

O tipo do valor que ```File::open()``` retorna dentro da variante **Err** é ```io::Error```, que é uma ```struct``` fornecida pela biblioteca padrão. Esta ```struct``` tem um método ```kind()``` que podemos chamar para obter um valor ```io::ErrorKind```. A enumeração  ```io::ErrorKind``` é fornecida pela biblioteca padrão e tem variantes que representam os diferentes tipos de erros que podem resultar de uma operação de entrada/saída (I/O). A variante que queremos usar é ```ErrorKind::NotFound```, que indica que o arquivo que estamos tentando abrir ainda não existe. Então, fazemos o match para  **greeting_file_result**, mas também temos um match interno em ```error.kind()```.

A condição que queremos verificar na correspondência interna é se o valor retornado por ```error.kind()``` é a variante ```NotFound``` da enumeração ```ErrorKind```. Se for, tentamos criar o arquivo com ```File::create()```. No entanto, como ```File::create()``` também pode falhar, precisamos de um segundo braço na expressão de correspondência interna. Quando o arquivo não pode ser criado, uma mensagem de erro diferente é impressa. O segundo braço da correspondência externa permanece o mesmo, então o programa entra em pânico em qualquer erro além do erro de arquivo ausente.

---

## Alternativas ao uso de match

**match** é uma ferramenta muito útil, mas algumas vezes torna o código muito verboso e difícil de entender. Nestas situações pode-se usar closures.

Por exemplo, aqui está outra maneira de escrever a mesma lógica mostrada no código anterior, desta vez usando closures e o método ```unwrap_or_else()```:

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problema ao criar o arquivo: {error:?}");
            })
        } else {
            panic!("Problema ao abrir o arquivo: {error:?}");
        }
    });
} 
```

Embora este código tenha o mesmo comportamento do anterior, ele não contém nenhum **match** e é mais limpo de ler. Além de ```unwrap_or_else()``` existem outros métodos da biblioteca padrão que ajudam neste tipo de trabalho. Estes métodos podem limpar grandes expressões **match** aninhadas quando você está lidando com erros.

---

## unwrap e expect 

Usar **match** funciona bem o suficiente, mas pode ser um pouco prolixo e nem sempre comunica bem a intenção. O tipo ```Result<T, E>``` tem muitos métodos auxiliares definidos para fazer várias tarefas mais específicas. O método ```unwrap()``` é um método de atalho implementado assim como a expressão **match** que usamos mais acima nos códigos. Se o valor ```Result``` for a variante ```Ok```, ```unwrap()``` retornará o valor dentro de ```Ok```. Se o ```Result``` for a variante ```Err```, ```unwrap()``` chamará a macro ```panic!``` para nós.

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

Se executarmos este código sem um arquivo hello.txt, veremos uma mensagem de erro da chamada ```panic!``` que o método ```unwrap()``` faz:

```
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Da mesma forma, o método ```expect()``` também nos permite escolher a mensagem de erro de ```panic!```. Usar ```expect()``` em vez de ```unwrap()``` e fornecer boas mensagens de erro pode transmitir sua intenção e facilitar o rastreamento da origem de um pânico.

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt deve existir neste projeto");
}
```

Usamos ```expect()``` da mesma forma que ```unwrap()```: para retornar o manipulador de arquivo ou chamar a macro ```panic!```. A mensagem de erro usada por ```expect()``` em sua chamada a ```panic!``` será o parâmetro que passamos para ```expect()```, em vez da mensagem ```panic!``` padrão que ```unwrap()``` usa. Aqui está como fica:

```
thread 'main' panicked at src/main.rs:5:10:
hello.txt deve existir neste projeto: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Em código de qualidade de produção, a maioria dos Rustaceans escolhe ```expect()``` em vez de ```unwrap()``` e fornece mais contexto sobre o que a operação precisa para sempre ter sucesso. Dessa forma, o programador terá mais informações para usar na depuração.

---

## propagando erros

asd

---

<img src="images/em_construcao.png" width="250" alt="EM CONSTRUCAO">

---




## Referências

[The Book - Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

arataca89@gmail.com

Última atualização: 20250101
