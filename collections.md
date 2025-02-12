#### arataca89

# Linguagem Rust - módulo `Collections`

A biblioteca `std::collections` do Rust fornece implementações eficientes das estruturas de dados de programação de propósito geral mais comuns. Ao usar as implementações padrão, deve ser possível que duas bibliotecas se comuniquem sem conversão significativa de dados.

Para começar: você provavelmente deveria usar [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) ou [HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html). Essas duas coleções cobrem a maioria dos casos de uso para armazenamento e processamento de dados genéricos. Elas são excepcionalmente boas em fazer o que fazem. Todas as outras coleções na biblioteca padrão têm casos de uso específicos onde são a escolha ideal, mas esses casos são quase de nicho em comparação. Mesmo quando `Vec` e `HashMap` não são tecnicamente a melhor escolha, elas provavelmente são uma boa escolha para começar.

As coleções do Rust podem ser agrupadas em quatro categorias principais: 

- Sequências: [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html), [VecDeque](https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html), [LinkedList](https://doc.rust-lang.org/std/collections/linked_list/struct.LinkedList.html)
- Mapas: [HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html), [BTreeMap](https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html)
- Conjuntos: [HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html), [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)
- Miscelânia: [BinaryHeap](https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html)

[Quando você deve usar qual coleção?](#quando-voc%C3%AA-deve-usar-qual-cole%C3%A7%C3%A3o)

[Performance](#performance)

[Custo das operações](#custo-das-opera%C3%A7%C3%B5es)

[Uso correto e eficiente de coleções](#uso-correto-e-eficiente-de-cole%C3%A7%C3%B5es)

- [Gerenciamento de capacidade](#gerenciamento-de-capacidade)
- [Iteradores](#iteradores)
- [API `Entry`](#api-entry)
- [Contando o número de vezes que cada caractere ocorre em uma string](#contando-o-n%C3%BAmero-de-vezes-que-cada-caractere-ocorre-em-uma-string)
- [Rastreando a embriaguez dos clientes em um bar](#rastreando-a-embriaguez-dos-clientes-em-um-bar)

[`insert` e chaves complexas](#insert-e-chaves-complexas)

---

## Quando você deve usar qual coleção?

Aqui temos um resumo de quando cada coleção deve ser considerada para determinado uso. Para mais detalhes sobre os pontos fortes e fracos de cada coleção consulte a documentação específica.

### Vec

- Você deseja coletar itens para serem processados ou enviados para outro lugar mais tarde, e não se importa com nenhuma propriedade dos valores reais que estão sendo armazenados.
- Você quer uma sequência de elementos em uma ordem específica e só irá inserir dados no final (ou próximo do final).
- Você quer uma Pilha.
- Você quer um array redimensionável.
- Você quer um array com os dados alocados na memória heap.

Documentação específica: [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

### VecDeque

- Você quer um `Vec` que suporte inserção eficiente no início e no final.
- Você quer uma Fila.
- Você quer uma fila de duas extremidades ("double-ended queue", ou "deque").

Documentação específica: [VecDeque](https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html)

### LinkedList

- Você quer um `Vec` ou `VecDeque` de tamanho desconhecido e não pode tolerar amortecimento.
- Você quer dividir e anexar listas de forma eficiente. 
- Você tem **absoluta certeza** de que **realmente**, **verdadeiramente**, quer uma lista duplamente encadeada. 

Documentação específica: [LinkedList](https://doc.rust-lang.org/std/collections/linked_list/struct.LinkedList.html)

### HashMap 

- Você quer uma estrutura do tipo chave-valor ("dicionário").
- Você quer um mapa, sem nenhuma funcionalidade extra. 

Documentação específica: [HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html)

### BTreeMap 

- Você quer um mapa ordenado por suas chaves. 
- Você quer ser capaz de obter uma gama de entradas sob demanda. 
- Você está interessado em qual é o par chave-valor menor ou maior. 
- Você quer encontrar a maior ou menor chave que seja menor ou maior do que algo. 

Documentação específica: [BTreeMap](https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html)

### Use a variante Set de qualquer um desses mapas quando

- Você só quer lembrar quais chaves você já viu.
- Não há nenhum valor significativo a associar às suas chaves. 
- Você só quer um conjunto.

Documentação específica: [HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html), [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

### BinaryHeap

- Você quer armazenar um monte de elementos, mas só quer processar o "maior" ou "mais importante" a qualquer momento. 
- Você quer uma fila de prioridade. 

Documentação específica: [BinaryHeap](https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html)

---

## Performance

Escolher a coleção certa para determinado trabalho exige entender o que cada coleção é boa em fazer. Aqui, resumimos brevemente o desempenho de diferentes coleções para certas operações importantes. Para mais detalhes, consulte a documentação de cada tipo e observe que os nomes dos métodos reais podem diferir das tabelas abaixo em certas coleções.

Ao longo da documentação, seguiremos as seguintes convenções para notação de operação: 

- O tamanho da coleção é denotado por `n`. 
- Se uma segunda coleção estiver envolvida, seu tamanho é denotado por `m`.
- Os índices dos itens são denotados por `i`.
- Operações que têm um custo amortizado são sufixadas com um `*`.
- Operações com um custo esperado são sufixadas com um `~`. 

Chamadas de operações que adicionam a uma coleção ocasionalmente exigirão que uma coleção seja redimensionada - uma operação extra que leva O(n) tempo. 

Os custos amortizados são calculados para contabilizar o custo de tempo dessas operações de redimensionamento ao longo de uma série suficientemente grande de operações. Uma operação individual pode ser mais lenta ou mais rápida devido à natureza esporádica do redimensionamento da coleção, no entanto, o custo médio por operação se aproximará do custo amortizado.

As coleções do Rust nunca diminuem automaticamente, então as operações de remoção não são amortizadas. 

O `HashMap` usa custos esperados. É teoricamente possível, embora muito improvável, que o `HashMap` experimente um desempenho significativamente pior do que o custo esperado. Isso se deve à natureza probabilística do hash - ou seja, é possível gerar um hash duplicado dado alguma chave de entrada que exigirá computação extra para corrigir.

---

## Custo das operações


<img src="images/collections_cost.png" width="100%" alt="CUSTO DAS COLECOES">

Observe que, em caso de empate, `Vec` geralmente será mais rápido que `VecDeque`, e `VecDeque` geralmente será mais rápido que `LinkedList`.

Para conjuntos (set), todas as operações têm o custo da operação de mapa equivalente.

---

## Uso correto e eficiente de coleções 
 
Saber qual coleção é a certa para o trabalho não permite instantaneamente que você a use corretamente. Aqui estão algumas dicas rápidas para o uso eficiente e correto das coleções padrão em geral. Se você estiver interessado em como usar uma coleção específica em particular, consulte sua documentação para discussão detalhada e exemplos de código.

### Gerenciamento de capacidade 
 
Muitas coleções fornecem vários construtores e métodos que se referem à “capacidade”. Essas coleções são geralmente construídas em cima de um array. O ideal seria que esse array tivesse exatamente o tamanho certo para caber apenas os elementos armazenados na coleção, mas para a coleção fazer isso seria muito ineficiente. Se o array de apoio tivesse exatamente o tamanho certo o tempo todo, então toda vez que um elemento fosse inserido, a coleção teria que aumentar o array para caber nele. Devido à maneira como a memória é alocada e gerenciada na maioria dos computadores, isso quase certamente exigiria alocar um array inteiramente novo e copiar cada elemento do antigo para o novo. Espero que você possa ver que isso não seria muito eficiente para fazer em todas as operações.

Portanto, a maioria das coleções usa uma estratégia de alocação amortizada. Elas geralmente se deixam ter uma quantidade razoável de espaço desocupado para que só precisem crescer ocasionalmente. Quando crescem, alocam um array substancialmente maior para mover os elementos, de modo que levará um tempo para que outro crescimento seja necessário. Embora essa estratégia seja ótima em geral, seria ainda melhor se a coleção nunca tivesse que redimensionar seu array de apoio. Infelizmente, a coleção em si não tem informações suficientes para fazer isso sozinha. Portanto, cabe a nós, programadores, dar dicas.

Qualquer construtor `with_capacity` instruirá a coleção a alocar espaço suficiente para o número especificado de elementos. O ideal é que isso seja para exatamente essa quantidade de elementos, mas alguns detalhes de implementação podem impedir isso. Consulte a documentação específica da coleção para obter detalhes. Em geral, use `with_capacity` quando souber exatamente quantos elementos serão inseridos ou, pelo menos, tiver um limite superior razoável para esse número.

Ao antecipar um grande influxo de elementos, a família de métodos `reserve` pode ser usada para sugerir à coleção quanto espaço ela deve deixar para os próximos itens. Assim como com `with_capacity`, o comportamento preciso desses métodos será específico para a coleção de interesse.

Para desempenho ideal, as coleções geralmente evitarão encolher a si mesmas. Se você acredita que uma coleção não conterá mais elementos em breve, ou apenas realmente precisa da memória, o método `shrink_to_fit` solicita que a coleção encolha o array de apoio para o tamanho mínimo capaz de conter seus elementos.

Finalmente, se você estiver interessado em qual é a capacidade real da coleção, a maioria das coleções fornece um método `capacity` para consultar essas informações sob demanda. Isso pode ser útil para fins de depuração ou para uso com os métodos `reserve`.

### Iteradores

Iteradores são um mecanismo poderoso e robusto usado em todas as bibliotecas padrão do Rust. Iteradores fornecem uma sequência de valores de forma genérica, segura, eficiente e conveniente. O conteúdo de um iterador geralmente é avaliado preguiçosamente, de modo que apenas os valores realmente necessários são realmente produzidos, e nenhuma alocação precisa ser feita para armazená-los temporariamente. Iteradores são consumidos principalmente usando um loop `for`, embora muitas funções também usem iteradores onde uma coleção ou sequência de valores é desejada.

Todas as coleções padrão fornecem vários iteradores para executar manipulação em massa de seus conteúdos. Os três iteradores primários que quase todas as coleções devem fornecer são `iter`, `iter_mut` e `into_iter`. Alguns deles não são fornecidos em coleções onde seria incorreto ou irracional fornecê-los.

`iter` fornece um iterador de referências imutáveis ​​para todos os conteúdos de uma coleção na ordem mais "natural". Para coleções de sequências como `Vec`, isso significa que os itens serão produzidos em ordem crescente de índice começando em 0. Para coleções ordenadas como `BTreeMap`, isso significa que os itens serão produzidos em ordem classificada. Para coleções não ordenadas como `HashMap`, os itens serão produzidos em qualquer ordem que a representação interna tornar mais conveniente. Isso é ótimo para ler todo o conteúdo da coleção.

```rust
let vec = vec![1, 2, 3, 4];
for x in vec.iter() {
   println!("vec contained {x:?}");
}
```

`iter_mut` fornece um iterador de referências mutáveis na mesma ordem que `iter`. Isso é ótimo para alterar todo o conteúdo da coleção.

```rust
let mut vec = vec![1, 2, 3, 4];
for x in vec.iter_mut() {
   *x += 1;
}
```

`into_iter` transforma a coleção real em um iterador sobre seu conteúdo por valor. Isso é ótimo quando a coleção em si não é mais necessária, e os valores são necessários em outro lugar. Usar `extend` com `into_iter` é a principal maneira de mover o conteúdo de uma coleção para outra. `extend` chama `into_iter` automaticamente, e pega qualquer `T: IntoIterator`. Chamar `collect` em um iterador em si também é uma ótima maneira de converter uma coleção em outra. Ambos os métodos devem usar internamente as ferramentas de gerenciamento de capacidade discutidas na seção anterior para fazer isso da forma mais eficiente possível.

```rust
let mut vec1 = vec![1, 2, 3, 4];
let vec2 = vec![10, 20, 30, 40];
vec1.extend(vec2);
```

```rust
use std::collections::VecDeque;

let vec = [1, 2, 3, 4];
let buf: VecDeque<_> = vec.into_iter().collect();
```

Iteradores também fornecem uma série de métodos adaptadores para executar threads comuns em sequências. Entre os adaptadores estão favoritos funcionais como `map`, `fold`, `skip` e `take`. De particular interesse para coleções é o adaptador `rev`, que reverte qualquer iterador que suporte essa operação. A maioria das coleções fornece iteradores reversíveis como a maneira de iterar sobre eles em ordem inversa.

```rust
let vec = vec![1, 2, 3, 4];
for x in vec.iter().rev() {
   println!("vec contained {x:?}");
}
```

Vários outros métodos de coleção também retornam iteradores para gerar uma sequência de resultados, mas evitam alocar uma coleção inteira para armazenar o resultado. Isso fornece flexibilidade máxima, pois `collect` ou `extend` podem ser chamados para "canalizar" a sequência para qualquer coleção, se desejado. Caso contrário, a sequência pode ser percorrida com um loop `for`. O iterador também pode ser descartado após o uso parcial, impedindo o cálculo dos itens não utilizados.

### API `Entry`

A API `entry` tem como objetivo fornecer um mecanismo eficiente para manipular o conteúdo de um mapa condicionalmente na presença de uma chave ou não. O principal caso de uso motivador para isso é fornecer mapas acumuladores eficientes. Por exemplo, se alguém deseja manter uma contagem do número de vezes que cada chave foi vista, ele terá que executar alguma lógica condicional sobre se esta é a primeira vez que a chave foi vista ou não. Normalmente, isso exigiria uma busca seguida por uma inserção, duplicando efetivamente o esforço de busca em cada inserção.

Quando um usuário chama `map.entry(key)`, o mapa buscará a chave e então produzirá uma variante do `enum Entry`.

Se um `Vacant(entry)` for produzido, então a chave não foi encontrada. Neste caso, a única operação válida é inserir um valor na entrada. Quando isso é feito, a entrada vaga é consumida e convertida em uma referência mutável para o valor que foi inserido. Isso permite uma manipulação posterior do valor além do tempo de vida da própria busca. Isso é útil se lógica complexa precisa ser executada no valor, independentemente de o valor ter sido inserido.

Se um `Occupied(entry)` for produzido, então a chave foi encontrada. Nesse caso, o usuário tem várias opções: ele pode obter, inserir ou remover o valor da entrada ocupada. Além disso, ele pode converter a entrada ocupada em uma referência mutável para seu valor, fornecendo simetria ao caso de inserção vago.

### Exemplos

Aqui estão as duas maneiras principais pelas quais a entrada é usada. Primeiro, um exemplo simples onde a lógica realizada nos valores é trivial. 
 
### Contando o número de vezes que cada caractere ocorre em uma string

```rust
use std::collections::btree_map::BTreeMap;

fn main(){
	let mut count = BTreeMap::new();
	let message = "she sells sea shells by the sea shore";

	for c in message.chars() {
		*count.entry(c).or_insert(0) += 1;
	}

	assert_eq!(count.get(&'s'), Some(&8));

	println!("Número de ocorrências de cada caractere");
	for (char, count) in &count {
		println!("{char}: {count}");
	}
}
```

Quando a lógica a ser executada no valor é mais complexa, podemos simplesmente usar a API `entry` para garantir que o valor seja inicializado e executar a lógica depois. 

### Rastreando a embriaguez dos clientes em um bar

```rust
use std::collections::btree_map::BTreeMap;

// Nível de álcool de uma pessoa.
struct Person { blood_alcohol: f32 }

fn main(){
	// Todos os pedidos do bar, por ID do cliente
	let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

	let mut blood_alcohol = BTreeMap::new();

	for id in orders {
		//Se esta for a primeira vez que vemos este cliente, inicialize-o
		// sem álcool no sangue. Caso contrário, apenas recupere-o.
		let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });

		// Reduza o nível de álcool no sangue. Leva tempo para pedir e beber uma cerveja!
		person.blood_alcohol *= 0.9;

		// Verifique se eles estão sóbrios o suficiente para tomar outra cerveja.
		if person.blood_alcohol > 0.3 {
			// Bêbado demais... por enquanto.
			println!("Desculpe {id}, tenho que interromper você");
		} else {
			println!("Tome outro drink {id}");
			person.blood_alcohol += 0.1;
		}
	}
}
```

---

## `insert` e chaves complexas

Se tivermos uma chave mais complexa, as chamadas a `insert()` não atualizarão o valor da chave. Por exemplo:

```rust
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

// 'Foo' será comparado apenas pelo seu valor 'a'.
impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool { self.a == other.a }
}

impl Eq for Foo {}

// hash de 'Foo' é feito apenas pelo seu valor 'a'
impl Hash for Foo {
    fn hash<H: Hasher>(&self, h: &mut H) { self.a.hash(h); }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       self.a.partial_cmp(&other.a)
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering { self.a.cmp(&other.a) }
}

let mut map = BTreeMap::new();
map.insert(Foo { a: 1, b: "baz" }, 99);

// Já temos um 'Foo' com o valor de 1, então este valor será atualizado.
map.insert(Foo { a: 1, b: "xyz" }, 100);

// Valor foi atualizado...
assert_eq!(map.values().next().unwrap(), &100);

// ... mas a chave não foi alterada. 'b' ainda vale "baz" e não "xyz".
assert_eq!(map.keys().next().unwrap().b, "baz");
```

---

## Referências

[Module collections](https://doc.rust-lang.org/std/collections/index.html)

---

arataca89@gmail.com

Última atualização: 20250212