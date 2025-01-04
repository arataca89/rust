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

[Propagando erros](#propagando-erros)

[Operador ?: um atalho para a propagação de erros](#operador--um-atalho-para-a-propaga%C3%A7%C3%A3o-de-erros)

[Onde o operador ? pode ser usado](#onde-o-operador--pode-ser-usado)

[Quando entrar em pânico e quando não](#quando-entrar-em-pânico-e-quando-não)

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

## Propagando erros

Quando a implementação de uma função chama algo que pode falhar, em vez de manipular o erro dentro da própria função, você pode retornar o erro para o código de chamada para que ele possa decidir o que fazer. Isso é conhecido como propagação do erro e dá mais controle ao código de chamada, onde pode haver mais informações ou lógica que ditam como o erro deve ser manipulado do que o que você tem disponível no contexto do seu código.

Por exemplo, o código abaixo mostra uma função que lê um nome de usuário de um arquivo. Se o arquivo não existir ou não puder ser lido, essa função retornará esses erros para o código que chamou a função.

```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Essa função pode ser escrita de uma forma muito mais curta, mas vamos começar fazendo muito dela manualmente para explorar o tratamento de erros; no final, mostraremos a maneira mais curta. Vamos olhar para o tipo de retorno da função primeiro: ```Result<String, io::Error>```. Isso significa que a função está retornando um valor do tipo ```Result<T, E>```, onde o parâmetro genérico **T** foi preenchido com o tipo concreto **String** e o tipo genérico **E** foi preenchido com o tipo concreto **io::Error**.

Se essa função for bem-sucedida sem problemas, o código que chama essa função receberá um valor **Ok** que contém uma **String** — o nome de usuário que essa função leu do arquivo. Se essa função encontrar algum problema, o código de chamada receberá um valor **Err** que contém uma instância de **io::Error** que contém mais informações sobre quais foram os problemas. Escolhemos **io::Error** como o tipo de retorno dessa função porque esse é o tipo do valor de erro retornado de ambas as operações que estamos chamando no corpo dessa função que podem falhar: a função ```File::open()``` e o método ```read_to_string()```.

O corpo da função começa chamando a função ```File::open()```. Então, manipulamos o valor **Result** com um **match**. Se ```File::open()``` for bem-sucedido, o identificador de arquivo na variável **file** se torna o valor da variável mutável **username_file** e a função continua. No caso de **Err**, em vez de chamar ```panic!```, usamos a palavra-chave **return** para retornar antecipadamente da função e passar o valor de erro de ```File::open()```, agora na variável **e**, de volta ao código de chamada como o valor de erro desta função.

Então, se tivermos um identificador de arquivo em **username_file**, a função cria uma nova **String** na variável **username** e chama o método ```read_to_string()``` no identificador de arquivo em **username_file** para ler o conteúdo do arquivo em **username**. O método ```read_to_string()``` também retorna um **Result** porque pode falhar, mesmo que ```File::open()``` tenha sido bem-sucedido. Então, precisamos de outro **match** para lidar com esse **Result**: se ```read_to_string()``` for bem-sucedido, então nossa função foi bem-sucedida, e retornamos o **username** do arquivo encapsulado em um **Ok**. Se ```read_to_string()``` falhar, retornamos o valor de erro da mesma forma que retornamos o valor de erro no **match** que lidou com o valor de retorno de ```File::open()```. No entanto, não precisamos dizer explicitamente **return**, porque esta é a última expressão na função.

O código que chama esse código então manipulará o valor recebido de um **Ok** que contém um nome de usuário ou de um **Err** que contém um **io::Error**. Cabe ao código de chamada decidir o que fazer com esses valores. Se o código de chamada obtiver um valor **Err**, ele poderá chamar ```panic!``` e encerrar o programa, usar um nome de usuário padrão ou procurar o nome de usuário em algum lugar que não seja um arquivo, por exemplo. Não temos informações suficientes sobre o que o código de chamada irá fazer, então propagamos todas as informações de sucesso ou erro para cima para que ele manipule adequadamente.

Como este padrão de propagação de erros é muito comum, Rust fornece o operador de interrogação, **?**, para tornar isso mais fácil.

---

## Operador ?: um atalho para a propagação de erros

O código abaixo mostra uma implementação de ```read_username_from_file()``` que tem a mesma funcionalidade do código anterior, mas esta implementação usa o operador **?**.

```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

O **?** colocado após um valor **Result** é definido para funcionar quase da mesma forma que as expressões **match** que definimos para manipular os valores **Result** no código anterior. Se o valor do **Result** for um **Ok**, o valor dentro do **Ok** será retornado desta expressão, e o programa continuará. Se o valor for um **Err**, o **Err** será retornado de toda a função como se tivéssemos usado a palavra-chave **return**, então o valor do erro é propagado para o código de chamada.

Há uma diferença entre o que a expressão **match** faz e o que o operador **?** faz: valores de erro que têm o operador **?** chamado neles passam pela função ```from()```, definida na trait ```From``` da biblioteca padrão, que é usada para converter valores de um tipo em outro. Quando o operador **?** chama a função ```from()```, o tipo de erro recebido é convertido no tipo de erro definido no tipo de retorno da função atual. Isso é útil quando uma função retorna um tipo de erro para representar todas as maneiras pelas quais uma função pode falhar, mesmo que partes possam falhar por muitos motivos diferentes.

Por exemplo, poderíamos alterar a função ```read_username_from_file()``` para retornar um tipo de erro personalizado chamado **OurError** que definimos. Se também definirmos ```impl From<io::Error>``` para **OurError** para construir uma instância de **OurError** a partir de um **io::Error**, então o operador **?** no corpo de ```read_username_from_file()``` chamará ```from()``` e converterá os tipos de erro sem precisar adicionar mais código à função. 

No contexto deste código atual, o **?** no final da chamada ```File::open()``` retornará o valor dentro de um **Ok** para a variável **username_file**. Se ocorrer um erro, o operador **?** retornará antecipadamente de toda a função e fornecerá qualquer valor **Err** para o código que está chamando. O mesmo se aplica ao **?** no final da chamada ```read_to_string()```.

O operador **?** elimina muita repetição e torna a implementação dessa função mais simples. Poderíamos até encurtar esse código ainda mais concatenando chamadas de método imediatamente após o **?**.

```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

Movemos a criação da nova **String** em **username** para o início da função; essa parte não mudou. Em vez de criar uma variável **username_file**, encadeamos a chamada para ```read_to_string()``` diretamente no resultado de ```File::open("hello.txt")?```. Ainda temos um **?** no final da chamada ```read_to_string()```, e ainda retornamos um valor **Ok** contendo **username** quando ```File::open()``` e ```read_to_string()``` são bem-sucedidos em vez de retornar erros. A funcionalidade é novamente a mesma do código anterior; esta é apenas uma maneira diferente e mais ergonômica de escrevê-la.

Abaixo temos uma maneira de tornar isso ainda mais curto usando ```fs::read_to_string()```.

```
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Ler um arquivo em uma string é uma operação bastante comum, então a biblioteca padrão fornece a função conveniente ```fs::read_to_string()``` que abre o arquivo, cria uma nova **String**, lê o conteúdo do arquivo, coloca o conteúdo nessa **String** e o retorna. Claro, usar ```fs::read_to_string()``` não nos dá a oportunidade de explicar todo o tratamento de erros, então fizemos isso da maneira mais longa primeiro. 

---

## Onde o operador ? pode ser usado

O operador **?** só pode ser usado em funções cujo tipo de retorno seja compatível com o valor em que o **?** é usado. Isso ocorre porque o operador **?** é definido para realizar um retorno antecipado de um valor da função, da mesma forma que a expressão **match**. No código anterior que usou **match**, **match** estava usando um valor ```Result```, e o braço de retorno antecipado retornou um valor ```Err(e)```. O tipo de retorno da função tem que ser um ```Result``` para que seja compatível com esse retorno.

No código abaixo, vamos analisar o erro que obteremos se usarmos o operador **?** em uma função principal com um tipo de retorno incompatível com o tipo do valor em que usamos com **?**.

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

Este código abre um arquivo, o que pode falhar. O operador **?** segue o valor ```Result``` retornado por ```File::open()```, mas esta função principal tem o tipo de retorno ```()```, não ```Result```. Quando compilamos este código, obtemos a seguinte mensagem de erro:

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
help: consider adding return type
  |
3 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
4 |     let greeting_file = File::open("hello.txt")?;
5 + 
6 +     Ok(())
7 + }
  |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` (bin "error-handling") due to 1 previous error
```

Este erro indica que só podemos usar o operador **?** em uma função que retorna ```Result```, ```Option``` ou outro tipo que implementa ```FromResidual```.

Para corrigir o erro, você tem duas opções. Uma opção é alterar o tipo de retorno da sua função para ser compatível com o valor em que você está usando o operador **?** desde que você não tenha nenhuma restrição impedindo isso. A outra opção é usar um **match** ou um dos métodos de ```Result<T, E>``` para lidar com o ```Result<T, E>``` da maneira que for apropriada.

A mensagem de erro também mencionou que **?** pode ser usado com valores ```Option<T>``` também. Assim como ao usar **?** em ```Result```, você só pode usar **?** em ```Option``` em uma função que retorna uma ```Option```. O comportamento do operador **?** quando chamado em um ```Option<T>``` é semelhante ao seu comportamento quando chamado em um ```Result<T, E>```: se o valor for ```None```, o ```None``` será retornado antecipadamente da função naquele ponto. Se o valor for ```Some```, o valor dentro de ```Some``` é o valor resultante da expressão, e a função continua. Abaixo temos um exemplo de uma função que encontra o último caractere da primeira linha no texto fornecido.

```
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

Esta função retorna ```Option<char>``` porque é possível que haja um caractere ali, mas também é possível que não haja. Este código pega o argumento da slice de string de texto e chama o método ```lines()``` nele, que retorna um iterador sobre as linhas na string. Como esta função quer examinar a primeira linha, ela chama ```next()``` no iterador para obter o primeiro valor do iterador. Se **text** for uma string vazia, esta chamada para ```next()``` retornará ```None```, nesse caso usamos **?** para parar e retornar ```None``` de ```last_char_of_first_line()```. Se **text** não for uma string vazia, ```next()``` retornará um valor ```Some``` contendo uma slice de string da primeira linha em **text**.
 
O **?** extrai a slice da string, e podemos chamar ```chars()``` nessa slice de string para obter um iterador de seus caracteres. Estamos interessados ​​no último caractere nesta primeira linha, então chamamos ```last()``` para retornar o último item no iterador. Esta é uma ```Option``` porque é possível que a primeira linha seja a string vazia; por exemplo, se **text** começa com uma linha em branco, mas tem caracteres em outras linhas, como em "\nhi". No entanto, se houver um último caractere na primeira linha, ele será retornado na variante ```Some```. O operador **?** no meio nos dá uma maneira concisa de expressar essa lógica, permitindo-nos implementar a função em uma linha. Se não pudéssemos usar o operador **?** em ```Option```, teríamos que implementar essa lógica usando mais chamadas de método ou uma expressão **match**. 
 
Observe que você pode usar o operador **?** em um ```Result``` em uma função que retorna ```Result```, e você pode usar o operador **?** em um ```Option``` em uma função que retorna ```Option```, mas você não pode misturar e combinar. O operador **?** não converterá automaticamente um ```Result``` em um ```Option``` ou vice-versa; nesses casos, você pode usar métodos como o método ```ok()``` em ```Result``` ou o método ```ok_or()``` em ```Option``` para fazer a conversão explicitamente. 
 
Até agora, todas as funções ```main()``` que usamos retornam ```()```. A função ```main()``` é especial porque é o ponto de entrada e saída de um programa executável, e existem restrições sobre qual pode ser seu tipo de retorno para que o programa se comporte como esperado. 
 
Felizmente, ```main()``` também pode retornar um ```Result<(), E>```. Abaixo temos um código usado anteriormente, mas mudamos o tipo de retorno de ```main()``` para ser ```Result<(), Box<dyn Error>>``` e adicionamos um valor de retorno ```Ok(())``` no final. Este código agora irá compilar.

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

O tipo ```Box<dyn Error>``` é um objeto trait. Você pode ler ```Box<dyn Error>``` como “qualquer tipo de erro”. Usar **?** em um valor ```Result``` em uma função ```main()``` com o tipo error ```Box<dyn Error>``` é permitido porque permite que qualquer valor ```Err``` seja retornado antecipadamente. Embora o corpo desta função ```main()``` só retorne erros do tipo ```std::io::Error```, ao especificar ```Box<dyn Error>```, esta assinatura continuará correta mesmo se mais código que retorna outros erros for adicionado ao corpo de ```main()```. 
 
Quando uma função ```main()``` retorna um ```Result<(), E>```, o executável sairá com um valor de 0 se ```main()``` retornar ```Ok(())``` e sairá com um valor diferente de zero se ```main()``` retornar um valor ```Err```. Executáveis escritos em C retornam inteiros quando saem: programas que saem com sucesso retornam o inteiro 0, e programas que geram erros retornam algum inteiro diferente de 0. O Rust também retorna inteiros de executáveis para ser compatível com essa convenção. 

A função ```main()``` pode retornar qualquer tipo que implemente a trait [std::process::Termination](https://doc.rust-lang.org/std/process/trait.Termination.html), que contém uma função ```report()``` que retorna um ```ExitCode```. Consulte a documentação da biblioteca padrão para obter mais informações sobre a implementação da trait ```Termination``` para seus próprios tipos.

Agora que discutimos os detalhes de chamar ```panic!``` ou retornar ```Result```, vamos voltar ao tópico de como decidir qual é apropriado para usar em quais casos.

---

## Quando entrar em pânico e quando não

Então, como você decide quando deve chamar ```panic!``` e quando deve retornar ```Result```? Quando o código entra em pânico, não há como se recuperar. Você pode chamar ```panic!``` para qualquer situação de erro, haja ou não uma maneira possível de se recuperar, mas então você está tomando a decisão de que uma situação é irrecuperável em nome do código chamador. Quando você escolhe retornar um valor ```Result```, você dá opções ao código chamador. Ele pode escolher tentar se recuperar de uma forma que seja apropriada para sua situação, ou pode decidir que um valor ```Err``` neste caso é irrecuperável, então ele pode chamar ```panic!``` e transformar seu erro recuperável em um irrecuperável. Portanto, retornar ```Result``` é uma boa escolha padrão quando você está definindo uma função que pode falhar.

Em situações como exemplos, código de protótipo e testes, é mais apropriado escrever código que causa pânico em vez de retornar um ```Result```. Vamos explorar porquê, e depois discutir situações em que o compilador não consegue dizer que a falha é impossível, mas você, como humano, pode. Concluiremos com algumas diretrizes gerais sobre como decidir se deve causar pânico no código da biblioteca.

#### Exemplos, códigos de protótipo e testes

Quando você está escrevendo um exemplo para ilustrar algum conceito, incluir código de tratamento de erros robusto também pode tornar o exemplo menos claro. Em exemplos, é entendido que uma chamada a um método como ```unwrap()```, que pode causar pânico, é um local de possível futura substituição do código para a maneira como você deseja que seu aplicativo trate o erro, o que pode variar com base no que o resto do seu código está fazendo. 
 
Da mesma forma, os métodos ```unwrap()``` e ```expect()``` são muito úteis durante a prototipagem, antes de você estar pronto para decidir como lidar com erros. Eles deixam marcadores claros em seu código para quando você estiver pronto para tornar seu programa mais robusto. 

Se uma chamada de método falhar em um teste, você deseja que todo o teste falhe, mesmo que esse método não seja a funcionalidade em teste. Como o ```panic!``` é como um teste é marcado como falha, chamar ```unwrap()``` ou ```expect()``` é exatamente o que deve acontecer.

<font color="blue"><b>Conclusão: em exemplos, protótipos e testes normalmente você deve usar ```panic!``` explicitamente, ```unwrap()``` ou ```expect()```.</b></font>

#### Quando você tem mais informações que o compilador

Também seria apropriado chamar ```unwrap()``` ou ```expect()``` quando você tem alguma outra lógica que garante que o ```Result``` terá um valor ```Ok```, mas a lógica não é algo que o compilador entende. Você ainda terá um valor ```Result``` que precisa manipular: qualquer operação que você esteja chamando ainda tem a possibilidade de falhar em geral, mesmo que seja logicamente impossível em sua situação particular. Se você puder garantir, inspecionando o código, que nunca terá uma variante ```Err```, é perfeitamente aceitável chamar ```unwrap()```, e é ainda melhor usar ```expect()``` passando como argumento o motivo pelo qual você acha que nunca terá uma variante ```Err```. Aqui está um exemplo:

```
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("O endereço IP foi codificado hardcode e deve ser válido");
```

Estamos criando uma instância ```IpAddr``` analisando uma string codificada. Podemos ver que **127.0.0.1** é um endereço IP válido, então é aceitável usar ```expect()``` aqui. No entanto, ter uma string codificada hardcode (diretamente no código) e válida não altera o tipo de retorno do método ```parse()```: ainda obtemos um valor ```Result```, e o compilador ainda nos fará lidar com ```Result``` como se a variante ```Err``` fosse uma possibilidade porque o compilador não é inteligente o suficiente para ver que essa string é sempre um endereço IP válido. Se a string do endereço IP viesse de um usuário em vez de ser codificada diretamente no programa (hardcode) e, portanto, tivesse uma possibilidade de falha, definitivamente gostaríamos de lidar com o ```Result``` de uma forma mais robusta. Mencionar a suposição de que esse endereço IP é codificado hardcode nos levará a mudar ```expect()``` para um código de tratamento de erros melhor se, no futuro, precisarmos obter o endereço IP de alguma outra fonte.

#### Diretrizes para tratamento de erros

É aconselhável fazer com que seu código entre em pânico quando houver a possibilidade de que ele possa acabar em um estado ruim. Nesse contexto, um estado ruim é quando alguma suposição, garantia, contrato ou invariante foi violada, como quando valores inválidos, valores contraditórios ou valores ausentes são passados para seu código - além de um ou mais dos seguintes:

* O estado ruim é algo inesperado, ao contrário de algo que provavelmente acontecerá ocasionalmente, como um usuário inserindo dados no formato errado;
* Seu código após este ponto precisa confiar em não estar nesse estado ruim, em vez de verificar o problema a cada passo;
* Não há uma boa maneira de codificar essa informação nos tipos que você usa.

Se alguém chamar seu código e passar valores que não fazem sentido, é melhor retornar um erro se puder para que o usuário da biblioteca possa decidir o que quer fazer nesse caso. No entanto, em casos em que continuar pode ser inseguro ou prejudicial, a melhor escolha pode ser chamar ```panic!``` e alertar a pessoa que usa sua biblioteca sobre o bug em seu código para que ela possa corrigi-lo durante o desenvolvimento. Da mesma forma, ```panic!``` geralmente é apropriado se você estiver chamando um código externo que está fora de seu controle e ele retorna um estado inválido que você não tem como corrigir.

No entanto, quando o fracasso é esperado, é mais apropriado retornar um ```Result``` do que fazer uma chamada ```panic!``` . Exemplos incluem um parser recebendo dados malformados ou uma solicitação HTTP retornando um status que indica que você atingiu um limite de taxa. Nesses casos, retornar um ```Result``` indica que a falha é uma possibilidade esperada que o código de chamada deve decidir como lidar.

Quando seu código executa uma operação que pode colocar um usuário em risco se for chamada usando valores inválidos, seu código deve verificar se os valores são válidos primeiro e entrar em pânico se os valores não forem válidos. Isso ocorre principalmente por motivos de segurança: tentar operar em dados inválidos pode expor seu código a vulnerabilidades. Esta é a principal razão pela qual a biblioteca padrão chamará ```panic!``` se você tentar um acesso de memória fora dos limites: tentar acessar memória que não pertence à estrutura de dados atual é um problema de segurança comum. Funções geralmente têm contratos: seu comportamento só é garantido se as entradas atenderem a requisitos específicos. Entrar em pânico quando o contrato é violado faz sentido porque uma violação de contrato sempre indica um bug do lado do chamador, e não é um tipo de erro que você deseja que o código de chamada tenha que lidar explicitamente. Na verdade, não há uma maneira razoável para o código de chamada se recuperar; os programadores do código chamador precisam consertar o código. Contratos para uma função, especialmente quando uma violação causará pânico, devem ser explicados na documentação da API para a função.

No entanto, ter muitas verificações de erro em todas as suas funções seria prolixo e irritante. Felizmente, você pode usar o sistema de tipos do Rust (e, portanto, a verificação de tipos feita pelo compilador) para fazer muitas das verificações para você. Se sua função tiver um tipo específico como parâmetro, você pode prosseguir com a lógica do seu código sabendo que o compilador já garantiu que você tem um valor válido. Por exemplo, se você tiver um tipo em vez de uma ```Option```, seu programa espera ter sempre algo em vez de poder ter nada em alguma situação. Seu código então não precisa lidar com dois casos para as variantes ```Some``` e ```None```: ele terá apenas um caso para definitivamente ter um valor. O código que tenta passar nada para sua função nem será compilado, então sua função não precisa verificar esse caso em tempo de execução. Outro exemplo é usar um tipo inteiro sem sinal, como ```u32```, que garante que o parâmetro nunca seja negativo.

#### Criando tipos personalizados para validação 
 
asd

---

<img src="images/em_construcao.png" width="250" alt="EM CONSTRUCAO">

---



## Referências

[The Book - Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

arataca89@gmail.com

Última atualização: 20250104
