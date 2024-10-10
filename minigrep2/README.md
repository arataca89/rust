# minigrep2 - melhorando o código do minigrep com iterators

Baseado no projeto descrito no [capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) e melhorado no [capítulo 13](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)


Com o conhecimento sobre iteradores, podemos melhorar o projeto do programa minigrep usando iteradores e tornando o código mais claro e conciso. Vamos ver como os iteradores podem melhorar nossa implementação das funções ```Config::build()``` e ```search()```. 


[1. Removendo a chamada a clone usando iteradores](#1-Removendo-a-chamada-a-clone-usando-iteradores)

[2. Usando o iterador retornado diretamente](#2-Usando-o-iterador-retornado-diretamente)

[3. Usando métodos de iteradores em vez de indexar](#3-Usando-métodos-de-iteradores-em-vez-de-indexar)

[4. Tornando o código mais legível com adaptadores de iteradores](#4-Tornando-o-código-mais-legível-com-adaptadores-de-iteradores)

[5. Escolhendo entre usar loops ou usar iteradores](#5-Escolhendo-entre-usar-loops-ou-usar-iteradores)

[6. Comparando a performance de loop versus iteradores](#6-Comparando-a-performance-de-loops-versus-iteradores)

## 1. Removendo a chamada a clone usando iteradores

Na implementação anterior, o código pegava uma slice de ```String``` e criava uma instância da estrutura ```Config``` indexando a slice e clonando os valores, permitindo que a estrutura ```Config``` assumisse a propriedade desses valores. 

```
impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Erro:Poucos argumentos.");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config{string, filepath, ignore_case})
    }
}
```

Foi necessário clonar aqui porque temos uma slice com elementos ```String``` no parâmetro ```args```, mas a função ```build()``` não possui a propriedade de ```args``` porque uma slice é um tipo de referência, portanto não possui propriedade. Para retornar a propriedade de uma instância ```Config```, tivemos que clonar os valores dos campos ```string``` e ```file_path``` de ```Config``` para que a instância de ```Config``` pudesse ser proprietária dos seus valores. 

Com os conhecimentos sobre iteradores, podemos mudar a função ```build()``` para assumir a propriedade de um iterador como seu argumento em vez de pegar uma slice emprestada. Usaremos a funcionalidade do iterador em vez do código que verifica o tamanho da slice e a indexa. Isso tornará mais claro o que a função ```Config::build()``` está fazendo porque o iterador acessará os valores. 

Com ```Config::build()``` assumindo a propriedade do iterador e não usando operações de indexação que emprestam, podemos mover os valores ```String``` do iterador para ```Config``` em vez de chamar ```clone()``` e fazer uma nova alocação. Isso tornará nosso código melhor em performance pois alocação tem um custo maior que a movimentação de valores. 

## 2. Usando o iterador retornado diretamente

A função ```main()```, no arquivo ```main.rs``` tinha a seguinte implementação:

```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        eprintln!("{err}");
        eprintln!("Uso: minigrep string arquivo");
        process::exit(1);
    });

	// código omitido
}
```

Note que a função ```main()``` no código acima não está completa, há código omitido. 

Vamos alterar para que agora ```main()``` receba um iterador.

```
fn main() {
    let config =  Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("{err}");
        eprintln!("Uso: minigrep string arquivo");
        process::exit(1);
    });

	// código omitido
}
```

```env::args()``` retorna uma estrutura ```env::Args``` que é um iterador sobre os argumentos passados na linha de comando.

Em vez de coletar os valores do iterador em um vetor e então passar uma slice para ```Config::build()```, agora estamos passando a propriedade do iterador retornado de ```env::args()``` para ```Config::build()``` diretamente. 

Depois disso temos que atualizar ```Config::build()``` no arquivo ```lib.rs```. Vamos mudar a assinatura de ```Config::build()```. Isso ainda não vai compilar porque precisamos atualizar o corpo da função. 

```
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // código omitido
```

A documentação da biblioteca padrão para a função ```env::args()``` mostra que o tipo do iterador que ela retorna é ```std::env::Args```, e que esse tipo implementa a trait ```Iterator``` e retorna valores ```String```.

Atualizamos a assinatura da função ```Config::build()``` para que o parâmetro ```args``` tenha um tipo genérico com a trait bound ```impl Iterator<Item = String>``` em vez de ```&[String]```. Este uso da sintaxe ```impl Trait``` significa que ```args``` pode ser qualquer tipo que implemente a trait ```Iterator``` e retorne itens ```String```.

Como estamos assumindo a propriedade de ```args``` e vamos alterar ```args``` iterando sobre ele, devemos adicionar a palavra-chave ```mut``` na especificação do parâmetro ```args``` para torná-lo mutável. 

## 3. Usando métodos de iteradores em vez de indexar
 
Em seguida, vamos corrigir o corpo de ```Config::build()```. Como ```args``` implementa a trait ```Iterator```, sabemos que podemos chamar o método ```next()```.

```
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        args.next();
        
        let string = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi passada string a ser pesquisada"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi passado o path do arquivo a ser pesquisado"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{string, filepath, ignore_case})
    }
}
```
 
Lembre-se que o primeiro valor retornado por ```env::args()``` é o nome do programa. Queremos ignorar isso e ir para o próximo valor, então primeiro chamamos ```next()``` e não fazemos nada com o valor de retorno. Depois chamamos ```next()``` para obter o valor que queremos colocar no campo ```string``` de ```Config```. Usamos uma estrutura ```match```. Se ```next()``` retornar um ```Some```, armazenamos o valor dentro do ```Some``` em ```string```. Se ```next()```retornar ```None```, significa que não foram fornecidos argumentos suficientes e retornamos antecipadamente com um valor ```Err```. Fazemos a mesma coisa para o valor ```file_path```. 

## 4. Tornando o código mais legível com adaptadores de iteradores

Também podemos tirar proveito de iteradores na função ```search()```. a implementação atual é. 

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}
```

Podemos escrever este código de forma mais concisa usando métodos adaptadores de iterador. Fazer isso também nos permite evitar ter um vetor de resultados intermediários mutável. O estilo de programação funcional prefere minimizar a quantidade de estado mutável para tornar o código mais claro. Remover o estado mutável pode permitir uma melhoria futura para tornar a pesquisa uma ação paralela, porque não precisaríamos gerenciar o acesso concorrente ao vetor de resultados. Lembrando que métodos adaptadores de iterador (Iterator adaptors) são métodos definidos na trait Iterator que não consomem o iterador. Em vez disso, eles produzem iteradores diferentes alterando algum aspecto do iterador original.

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Lembre-se de que o objetivo da função ```search()``` é retornar todas as linhas de ```contents``` que contenham ```query```. Este código usa o adaptador ```filter()``` para manter apenas as linhas para as quais ```line.contains(query)``` retorna ```true```. Em seguida, coletamos as linhas correspondentes em outro vetor com ```collect()```. Muito mais simples.

Podemos fazer o mesmo tipo de alteração na função ```search_case_insensitive()```. 

```
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
```

Observe que após usar ```to_lowercase()``` em ```query``` o valor retornado será do tipo ```String```. Porém, o argumento de ```contains()``` deve ser um tipo ```&str``` e não um tipo ```String```. Por isso tivemos que inserir o ```&``` antes de ```query.to_lowercase()```.

## 5. Escolhendo entre usar loops ou usar iteradores

A próxima pergunta lógica é qual estilo você deve escolher em seu próprio código e por quê: a implementação antiga ou a versão usando iteradores apresentada aqui. A maioria dos programadores Rust prefere usar o estilo iterador. É um pouco mais difícil de pegar o jeito no começo, mas depois que você pega o jeito dos vários adaptadores de iterador e o que eles fazem, os iteradores podem ser mais fáceis de entender. Em vez de mexer com vários loops e construir novos vetores, o código se concentra no objetivo de alto nível. Isso abstrai parte do código comum para que seja mais fácil ver os conceitos que são exclusivos desse código, como a condição de filtragem que cada elemento no iterador deve passar.

Mas as duas implementações são realmente equivalentes? A suposição intuitiva pode ser que o loop de nível mais baixo será mais rápido. Vamos falar sobre desempenho. 

## 6. Comparando a performance de loops versus iteradores

Para determinar se deve usar loops ou iteradores, você precisa saber qual implementação é mais rápida: a versão da função ```search()``` com um loop for explícito ou a versão com iteradores.

Executamos um benchmark carregando todo o conteúdo do livro "As Aventuras de Sherlock Holmes" de Sir Arthur Conan Doyle em uma String e procurando pela palavra "the" no conteúdo. Aqui estão os resultados do benchmark na versão de ```search()``` usando o laço for e a versão usando iteradores:

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

A versão usando iterador foi ligeiramente mais rápida! Não vamos explicar o código de benchmark aqui, porque o objetivo não é provar que as duas versões são equivalentes, mas sim ter uma ideia geral de como essas duas implementações se comparam em termos de desempenho. 

Para um benchmark mais abrangente, você deve usar vários textos de vários tamanhos como conteúdo, palavras diferentes e palavras de diferentes comprimentos como consulta, e todos os tipos de outras variações. O ponto é este: iteradores, embora sejam uma abstração de alto nível, são compilados para aproximadamente o mesmo código como se você tivesse escrito código de baixo nível. Iteradores são uma das abstrações de custo zero do Rust, com o que queremos dizer que usar a abstração não impõe nenhuma sobrecarga de tempo de execução adicional. Isso é análogo a como Bjarne Stroustrup, o designer e implementador original do C++, define sobrecarga zero em “Foundations of C++” (2012):

Em geral, as implementações de C++ obedecem ao princípio de zero sobrecarga: O que você não usa, você não paga. E ainda: O que você usa, você não poderia codificar manualmente melhor. 

Closures e iteradores são recursos do Rust inspirados em ideias de linguagens de programação funcional. Eles contribuem para a capacidade do Rust de expressar ideias de alto nível com desempenho de baixo nível. As implementações de closures e iteradores são tais que o desempenho em tempo de execução não é afetado. Isso faz parte do objetivo do Rust de buscar fornecer abstrações de custo zero. 

---
## Referências

[capítulo 12 do "Livro"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

[capítulo 13](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)

[std::env::args()](https://doc.rust-lang.org/beta/std/env/fn.args.html)

[std::env::Args](https://doc.rust-lang.org/beta/std/env/struct.Args.html)

[to_lowercase()](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)

[contains()](https://doc.rust-lang.org/std/primitive.str.html#method.contains)

---
arataca89@gmail.com

Última atualização: 20241010
