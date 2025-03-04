#### arataca89

# Linguagem Rust - Vec

std::Vec

```
pub struct Vec<T, A = Global>
where
    A: Allocator,
{ /* private fields */ }
```

Um tipo de array dinâmico, escrito como ```Vec<T>```, abreviação para "vetor".


* [Exemplos](#exemplos)
* [Indexação](#indexação)
* [Slicing](#slicing)
* [Capacidade e realocação](#capacidade-e-realocação)
* [Garantias](#garantias)
* [Métodos](#métodos)
	- [append()](#append) - Move os elementos de um vetor para outro.
	- [as_mut_slice()](#as_mut_slice) - Extrai uma slice mutável de todo o vetor.
	- [as_ptr()](#as_ptr) - Retorna um ponteiro bruto para o buffer do vetor.
	- [as_slice()](#as_slice) - Extrai uma slice contendo o vetor inteiro.
	- [capacity()](#capacity) - Retorna o número total de elementos que o vetor pode conter sem realocar.
	- [clear()](#clear) - Limpa o vetor, removendo todos os valores.
	- [dedub_by()](#dedup_by) - Remove todos, exceto o primeiro, dos elementos consecutivos no vetor que satisfazem uma determinada relação de igualdade.
	- [dedup()](#dedup) - Remove elementos repetidos consecutivos.
	- [dedup_by_key()](#dedup_by_key) - Remove todos, exceto o primeiro, dos elementos consecutivos no vetor que resolvem para a mesma chave.
	- [drain()](#drain) - Remove o intervalo especificado do vetor, retornando todos os elementos removidos como um iterador.
	- [extend_from_slice()](#extend_from_slice) - Clona e anexa todos os elementos de uma slice ao ```Vec```.
	- [extend_from_within()](#extend_from_within) - Copia elementos de dentro do vetor para o final.
	- [from_raw_parts()](#from_raw_parts) - Cria um ```Vec<T>``` diretamente de um ponteiro, um comprimento e uma capacidade.
	- [insert()](#insert) - Insere um elemento numa determinada posição , deslocando todos os elementos após ele para a direita.
	- [into_boxed_slice()](#into_boxed_slice) - Converte o vetor em ```Box<[T]>```.
	- [into_flattened()](#into_flattened) - Recebe um ```Vec<[T; N]>``` e o achata em um ```Vec<T>```.
	- [is_empty()](#is_empty) - Retorna ```true``` se o vetor não contém elementos.
	- [len()](#len) - Retorna o número de elementos no vetor.
	- [new()](#new) - Constrói um novo ```Vec<T>``` vazio.
	- [pop()](#pop) -  Remove o último elemento do vetor e o retorna, ou ```None``` se estiver vazio. <font color="green">**Complexidade O(1)**</font>. <img src="images/ok.png" width="30" alt="OK">
	- [push()](#push) - Insere um elemento no final do vetor.
	- [remove()](#remove) - Remove e retorna elemento em determinada posição, deslocando todos os elementos após ele para a esquerda.
	- [reserve()](#reserve) - Reserva capacidade para mais elementos.
	- [reserve_exact()](#reserve_exact) - Reserva capacidade mínima para mais elementos.	
	- [resize()](#resize) - Redimensiona o vetor.
	- [resize_with()](#resize_with) - Redimensiona o vetor.
	- [retain()](#retain) - Mantém apenas os elementos especificados pelo predicado (closure passada como argumento).
	- [set_len()](#set_len) - Ajusta o comprimento do vetor, mas de forma insegura. (<font color="red">unsafe</font>)
	- [shrink_to()](#shrink_to) - Reduz a capacidade do vetor com um limite inferior.
	- [shrink_to_fit()](#shrink_to_fit) - Reduz a capacidade do vetor o máximo possível.
	- [spare_capacity_mut()](#spare_capacity_mut) - Retorna a capacidade de reserva restante do vetor.
	- [splice()](#splice) - Cria um iterador que substitui o intervalo especificado no vetor por outro iterador e produz os itens removidos.
	- [split_off()](#split_off) - Divide o vetor em duas partes no índice fornecido.
	- [swap_remove()](#swap_remove()) - Remove e retorna um elemento substituindo-o no vetor pelo último elemento. <font color="green">**Complexidade O(1)**</font>. <img src="images/ok.png" width="30" alt="OK">
	- [truncate()](#truncate) - Reduz o tamanho do vetor, descartando elementos.
	- [try_reserve()](#try_reserve) - Tenta reservar capacidade para mais elementos.
	- [try_reserve_exact()](#try_reserve_exact)- Tenta reservar capacidade mínima para mais elementos.
	- [with_capacity()](#with_capacity) - Constrói um novo ```Vec<T>``` vazio com pelo menos a capacidade especificada.
	
---

## Exemplos

```
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3]);

for x in &vec {
    println!("{x}");
}
assert_eq!(vec, [7, 1, 2, 3]);
```

O macro ```vec!``` é fornecida para facilitar a inicialização:

```
let mut vec1 = vec![1, 2, 3];
vec1.push(4);
let vec2 = Vec::from([1, 2, 3, 4]);
assert_eq!(vec1, vec2);
```

Pode-se também inicializar cada elemento de um ```Vec<T>``` com um valor fornecido. Isso pode ser mais eficiente do que realizar alocação e inicialização em etapas separadas, especialmente ao inicializar um vetor com valores "zeros":

```
let vec = vec![0; 5];
assert_eq!(vec, [0, 0, 0, 0, 0]);

// equivalente, mas potencialmente mais lento:
let mut vec = Vec::with_capacity(5);
vec.resize(5, 0);
assert_eq!(vec, [0, 0, 0, 0, 0]);

let vec1 = vec!["asd"; 5];
assert_eq!(vec1, ["asd", "asd", "asd", "asd", "asd"]);

let vec2 = vec![0.0; 3];
assert_eq!(vec2, [0.0, 0.0, 0.0]);
```

Para mais informações, consulte [Capacidade e Realocação](#capacidade-e-realocação). 


Use um ```Vec<T>``` como uma pilha:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    // imprime na tela: 3, 2, 1,
    print!("{top}, ");
}
``` 

## Indexação

O tipo ```Vec``` permite acesso a valores por índice, pois implementa a trait ```Index```. 

```
let v = vec![13, 14, 15, 16];
assert_eq!(v[0], 13);
assert_eq!(v[1], 14);
assert_eq!(v[2], 15);
assert_eq!(v[3], 16);
```

No entanto, tenha cuidado: se você tentar acessar um índice que não está no ```Vec```, seu software entrará em pânico!

<table><tr>
<td><img src="images/error.png" width="48" alt="ERROR"></td>
<td>
<pre>
let v = vec![13, 14, 15, 16];
assert_eq!(v[4], 17); // panic!
</pre>
</td>
</tr></table>


Use ```get()``` e ```get_mut()``` se você quiser verificar se o índice é válido.


## Slicing 

Um ```Vec``` pode ser mutável. Por outro lado, slices(fatias) são objetos somente de leitura. Para obter uma slice, use ```&```.

```
fn read_slice(slice: &[usize]) {
    for i in slice {
        print!("{i}, ");
    }
    println!("\n");
}

fn main() {
    let v = vec![13,14,15,16];
    read_slice(&v);
    
    // você também pode fazer assim:
    let u: &[usize] = &v;
    read_slice(u); 
    
    // ou assims:
    let u: &[_] = &v;
    read_slice(u);
}
```

Saída:

```
13, 14, 15, 16, 

13, 14, 15, 16, 

13, 14, 15, 16, 
```

Em Rust, é mais comum passar slices como argumentos, em vez de vetores, quando você só quiser fornecer acesso de leitura. O mesmo vale para ```String``` e ```&str```.

## Capacidade e realocação 

A **capacidade** de um vetor é a quantidade de espaço alocada para quaisquer elementos futuros que serão adicionados ao vetor. Isso não deve ser confundido com o **comprimento** de um vetor, que especifica o número de elementos reais dentro do vetor. Se o comprimento de um vetor exceder sua capacidade, sua capacidade será aumentada automaticamente, mas seus elementos precisarão ser realocados.

Por exemplo, um vetor com capacidade 10 e comprimento 0 seria um vetor vazio com espaço para mais 10 elementos. Inserir 10 ou menos elementos no vetor não mudará sua capacidade nem causará a realocação. No entanto, se o comprimento do vetor for aumentado para 11, ele terá que realocar, o que pode ser lento. Por esse motivo, é recomendado usar ```Vec::with_capacity()``` sempre que possível para especificar o tamanho esperado do vetor.

## Garantias

Devido à sua natureza incrivelmente fundamental, o ```Vec``` traz muitas garantias sobre seu design. Isso garante que ele seja o mais leve possível no caso geral e possa ser manipulado corretamente de maneiras primitivas por código inseguro. Observe que essas garantias se referem a um ```Vec<T>``` não qualificado. Se parâmetros de tipo adicionais forem adicionados (por exemplo, para suportar alocadores personalizados), substituir seus valores padrão pode alterar o comportamento.

Fundamentalmente, ```Vec``` é e sempre será um trio (ponteiro, capacidade, comprimento). Nem mais, nem menos. A ordem desses campos é completamente não especificada, e você deve usar os métodos apropriados para modificá-los. O ponteiro nunca será nulo, então este tipo é otimizado para ponteiro nulo.

No entanto, o ponteiro pode não apontar realmente para a memória alocada. Em particular, se você construir um ```Vec``` com capacidade 0 via ```Vec::new()```, ```vec![]```, ```Vec::with_capacity(0)```, ou chamando ```shrink_to_fit()``` em um ```Vec``` vazio, ele não alocará memória. Da mesma forma, se você armazenar tipos de tamanho zero dentro de um ```Vec```, ele não alocará espaço para eles. Observe que, neste caso, o ```Vec``` pode não relatar uma capacidade de 0. O ```Vec``` alocará se e somente se ```mem::size_of::<T>() * capacity() > 0```. Em geral, os detalhes de alocação do ```Vec``` são muito sutis — se você pretende alocar memória usando um ```Vec``` e usá-la para outra coisa (para passar para um código não seguro ou para construir sua própria coleção com suporte de memória), certifique-se de desalocar essa memória usando ```from_raw_parts()``` para recuperar o ```Vec``` e, em seguida, descartá-lo. 

Se um ```Vec``` tiver alocado memória, então a memória para a qual ele aponta está na **heap** (conforme definido pelo alocador que o Rust está configurado para usar por padrão), e seu ponteiro aponta para ```len()``` elementos inicializados, contíguos em ordem (o que você veria se o convertesse para uma slice), seguido por ```capacity() - len()``` elementos logicamente não inicializados, contíguos. 

Um vetor contendo os elementos 'a' e 'b' com capacidade 4 pode ser visualizado abaixo. A parte superior é a estrutura ```Vec```, ela contém um ponteiro para o início da alocação na heap, comprimento e capacidade. A parte inferior é a alocação na heap, um bloco de memória contíguo.

```
            ptr      len  capacity
       +--------+--------+--------+
       | 0x0123 |      2 |      4 |
       +--------+--------+--------+
            |
            v
Heap   +--------+--------+--------+--------+
       |    'a' |    'b' | uninit | uninit |
       +--------+--------+--------+--------+
```

* **uninit** representa memória que não foi inicializada, veja **MaybeUninit**.

* Nota: o **ABI** não é estável e o **Vec** não garante nada sobre seu layout de memória (incluindo a ordem dos campos). 
 
O ```Vec``` nunca realizará uma "pequena otimização" onde os elementos são realmente armazenados na pilha por dois motivos:

* Isso tornaria mais difícil para código inseguro manipular um ```Vec``` corretamente. O conteúdo de um ```Vec``` não teria um endereço estável se ele fosse apenas movido, e seria mais difícil determinar se um ```Vec``` realmente alocou memória. 
* Isso penalizaria o caso geral, incorrendo em um ramo adicional a cada acesso. 

Um ```Vec``` nunca irá encolher automaticamente, mesmo que esteja completamente vazio. Isso garante que não haja alocações ou desalocações desnecessárias. Esvaziar um ```Vec``` e depois preenchê-lo novamente para o mesmo comprimento não deve resultar em nenhuma chamada para o alocador. Se você deseja liberar memória não utilizada, use ```shrink_to_fit()``` ou ```shrink_to()```.

```push()``` e ```insert()``` nunca (re)alocam se a capacidade relatada for suficiente. ```push()``` e ```insert()``` (re)alocam se ```len() == capacity()```. Ou seja, a capacidade relatada é completamente precisa e pode ser confiável. Pode até ser usada para liberar manualmente a memória alocada por um ```Vec```, se desejado. Métodos de inserção em massa podem realocar, mesmo quando não necessário.

```Vec``` não garante nenhuma estratégia de crescimento específica ao realocar quando cheio, nem quando ```reserve()``` é chamado. A estratégia atual é básica e pode ser desejável usar um fator de crescimento não constante. Qualquer estratégia que seja usada, é claro, garantirá O(1) ```push()``` amortizado. 
 
```vec![x; n]```, ```vec![a, b, c, d]``` e ```Vec::with_capacity(n)``` produzirão um ```Vec``` com pelo menos a capacidade solicitada. Se ```len() == capacity()``` (como é o caso da macro ```vec!```), então um ```Vec<T>``` pode ser convertido para e de um ```Box<[T]>``` sem realocar ou mover os elementos. 

O ```Vec``` não sobrescreverá especificamente nenhum dado removido dele, mas também não o preservará especificamente. Sua memória não inicializada é um espaço de rascunho que ele pode usar como quiser. Ele geralmente fará o que for mais eficiente ou fácil de implementar. Não confie em dados removidos para serem apagados por motivos de segurança. Mesmo se você descartar um ```Vec```(drop), seu buffer pode simplesmente ser reutilizado por outra alocação. Mesmo se você zerar a memória de um ```Vec``` primeiro, isso pode não acontecer de fato porque o otimizador não considera isso um efeito colateral que deve ser preservado. Há um caso que não iremos quebrar, no entanto: usar código inseguro para gravar na capacidade excedente e, em seguida, aumentar o comprimento para corresponder, é sempre válido.

Atualmente, ```Vec``` não garante a ordem em que os elementos são descartados. A ordem mudou no passado e pode mudar novamente. 

## Métodos 

## new()

```
new() -> Vec<T>
```

Constrói um novo ```Vec<T>``` vazio.

O vetor não alocará memória até que elementos sejam inseridos nele.

```
let mut vec: Vec<i32> = Vec::new();
```

## with_capacity()

```
with_capacity(capacity: usize) -> Vec<T>
```

Constrói um novo ```Vec<T>``` vazio com pelo menos a capacidade especificada.

O vetor será capaz de conter pelo menos ```capacity``` elementos sem realocar. Este método tem permissão para alocar mais elementos do que ```capacity```. Se ```capacity``` for 0, o vetor não alocará.

É importante observar que, embora o vetor retornado tenha a capacidade mínima especificada, o vetor terá um comprimento zero. Para uma explicação da diferença entre comprimento e capacidade, consulte [Capacidade e realocação](#capacidade-e-realocação).

Se for importante saber a capacidade alocada exata de um ```Vec```, sempre use o método ```capacity()``` após a construção.

Para ```Vec<T>``` onde ```T``` é um tipo de tamanho zero, não haverá alocação e a capacidade sempre será ```usize::MAX```.

### Exemplos

```
let mut vec = Vec::with_capacity(10);

// O vetor não contém itens, embora tenha capacidade para mais
assert_eq!(vec.len(), 0);
assert!(vec.capacity() >= 10);

// Tudo isso é feito sem realocação...
for i in 0..10 {
    vec.push(i);
}
assert_eq!(vec.len(), 10);
assert!(vec.capacity() >= 10);

// ...mas isso pode fazer com que o vetor seja realocado
vec.push(11);
assert_eq!(vec.len(), 11);
assert!(vec.capacity() >= 11);

// Um ​​vetor de um tipo de tamanho zero sempre será superalocado, 
// pois nenhuma alocação é necessária
let vec_units = Vec::<()>::with_capacity(10);
assert_eq!(vec_units.capacity(), usize::MAX);
```

## from_raw_parts()

```
from_raw_parts(
    ptr: *mut T,
    length: usize,
    capacity: usize,
) -> Vec<T>
```

Cria um ```Vec<T>``` diretamente de um ponteiro, um comprimento e uma capacidade. 

### Segurança

Isso é altamente inseguro, devido ao número de invariantes que não são verificados:

* **ptr** deve ter sido alocado usando o alocador global, como por meio da função ```alloc::alloc()```.
* **T** precisa ter o mesmo alinhamento com o qual **ptr** foi alocado. (**T** ter um alinhamento menos estrito não é suficiente, o alinhamento realmente precisa ser igual para satisfazer o requisito de ```dealloc()``` de que a memória deve ser alocada e desalocada com o mesmo layout.)
* O tamanho de **T** vezes **capacity** (ou seja, o tamanho alocado em bytes) precisa ser do mesmo tamanho com o qual o ponteiro foi alocado. (Porque semelhante ao alinhamento, ```dealloc()``` deve ser chamado com o mesmo tamanho de layout.)
* **length** precisa ser menor ou igual à **capacity**.
* Os primeiros valores de **length**  devem ser valores inicializados corretamente do tipo **T**.
* **capacity** precisa ser a capacidade com a qual o ponteiro foi alocado.
* O tamanho alocado em bytes não deve ser maior que ```isize::MAX```. Veja a documentação de segurança de ```pointer::offset```.

Esses requisitos são sempre mantidos por qualquer **ptr** que tenha sido alocado via ```Vec<T>```. Outras fontes de alocação são permitidas se os invariantes forem mantidos.

Violar isso pode causar problemas como corromper as estruturas de dados internas do alocador. Por exemplo, normalmente **<font color="red">não é seguro</font>** construir um ```Vec<u8>``` de um ponteiro para um array C ```char``` com tamanho ```size_t```, fazer isso só é seguro se o array foi inicialmente alocado por um ```Vec``` ou ```String```. Também não é seguro construir um a partir de um ```Vec<u16>``` e seu comprimento, porque o alocador se importa com o alinhamento, e esses dois tipos têm alinhamentos diferentes. O buffer foi alocado com alinhamento 2 (para ```u16```), mas depois de transformá-lo em um ```Vec<u8>``` ele será desalocado com alinhamento 1. Para evitar esses problemas, geralmente é preferível fazer casting/transmutação usando ```slice::from_raw_parts()```.

A propriedade de **ptr** é efetivamente transferida para o ```Vec<T>```, que pode então desalocar, realocar ou alterar o conteúdo da memória apontada pelo ponteiro à vontade. Certifique-se de que nada mais use o ponteiro após chamar esta função.

### Exemplos

```
use std::ptr;
use std::mem;

let v = vec![1, 2, 3];

// Impede a execução do destrutor de `v`
// para que tenhamos controle total da alocação.
let mut v = mem::ManuallyDrop::new(v);

// Extrai várias informações importantes sobre `v`
let p = v.as_mut_ptr();
let len = v.len();
let cap = v.capacity();

unsafe {
    // Sobrescreve a memória com 4, 5, 6
    for i in 0..len {
        ptr::write(p.add(i), 4 + i);
    }

    // Coloca tudo de volta em um Vec
    let rebuilt = Vec::from_raw_parts(p, len, cap);
    assert_eq!(rebuilt, [4, 5, 6]);
}
```

Usando memória que foi alocada em outro lugar:

```
use std::alloc::{alloc, Layout};

fn main() {
    let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

    let vec = unsafe {
        let mem = alloc(layout).cast::<u32>();
        if mem.is_null() {
            return;
        }

        mem.write(1_000_000);

        Vec::from_raw_parts(mem, 1, 16)
    };

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);
}
```

## capacity()

```
capacity(&self) -> usize
```

Retorna o número total de elementos que o vetor pode conter sem realocar.

```
let mut vec: Vec<i32> = Vec::with_capacity(10);
vec.push(42);
assert!(vec.capacity() >= 10);
```

## reserve()

```
reserve(&mut self, additional: usize)
```

Reserva capacidade para pelo menos mais **additional** elementos a serem inseridos no ```Vec<T>```. Mais espaço pode ser reservado para evitar alocações frequentes. Após chamar ```reserve()```, a capacidade será maior ou igual a ```self.len() + additional```. Não faz nada se a capacidade já for suficiente.

### Pânico

Entra em pânico se a nova capacidade exceder ```isize::MAX``` bytes.

```
let mut vec = vec![1];
vec.reserve(10);
assert!(vec.capacity() >= 11);
```

## reserve_exact()

```
reserve_exact(&mut self, additional: usize)
```

Reserva a capacidade mínima para pelo menos mais **additional** elementos a serem inseridos no ```Vec<T>```. Ao contrário de ```reserve()```, isso não alocará mais memória deliberadamente para evitar alocações frequentes. Após chamar ```reserve_exact()```, capacity será maior ou igual a ```self.len() + additional```. Não faz nada se a capacidade já for suficiente.

Observe que o alocador pode dar mais espaço que o solicitado. Portanto, não se pode confiar que a capacidade seja precisamente a mínima. Prefira ```reserve()``` se inserções futuras forem esperadas.

### Pânico

Entra em pânico se a nova capacidade exceder ```isize::MAX``` bytes.

```
let mut vec = vec![1];
vec.reserve_exact(10);
assert!(vec.capacity() >= 11);
```

## try_reserve()


```
try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
```
 
Tenta reservar capacidade para pelo menos mais **additional** elementos  a serem inseridos no ```Vec<T>```. Mais espaço pode ser reservado para evitar alocações frequentes. Após chamar ```try_reserve()```, a capacidade será maior ou igual a ```self.len() + additional``` se retornar ```Ok(())```. Não faz nada se a capacidade já for suficiente. Este método preserva o conteúdo mesmo que ocorra um erro.

### Erro

Se a capacidade transbordar(overflow) ou o alocador relatar uma falha, um erro será retornado. 

```
use std::collections::TryReserveError;

fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    let mut output = Vec::new();

    // Pré-reserva a memória, saindo se não conseguir
    output.try_reserve(data.len())?;

// memória reservada com sucesso, o trabalho continua...
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // muito complicado
    }));

    Ok(output)
}
```

## try_reserve_exact()

```
try_reserve_exact(
    &mut self,
    additional: usize,
) -> Result<(), TryReserveError>
```

Tenta reservar a capacidade mínima para que pelo menos ```additional``` elementos sejam inseridos no ```Vec<T>```. Ao contrário de ```try_reserve()```, não alocará memória a mais para evitar alocações frequentes. Após chamar ```try_reserve_exact()```, a capacidade será maior ou igual a ```self.len() + additional``` se retornar ```Ok(())```. Não faz nada se a capacidade já for suficiente.

Observe que o alocador pode fornecerar mais espaço que o solicitado. Portanto, não se pode confiar que a capacidade seja precisamente mínima. Prefira ```try_reserve()``` se inserções futuras forem esperadas.

### Erro

Se a capacidade transbordar(overflow) ou o alocador relatar uma falha, um erro será retornado.

```
use std::collections::TryReserveError;

fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    let mut output = Vec::new();

    // Pré-reserva a memória, saindo se não conseguir
    output.try_reserve_exact(data.len())?;

// memória reservada com sucesso, o trabalho continua...
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // muito complicado
    }));

    Ok(output)
}
```

## shrink_to_fit()

```
shrink_to_fit(&mut self)
```

Reduz a capacidade do vetor o máximo possível.

O comportamento deste método depende do alocador, que pode reduzir o vetor no local ou realocar. O vetor resultante pode ainda ter alguma capacidade excedente, tal como acontece com ```with_capacity()```. Consulte ```Allocator::shrink``` para mais detalhes.

```
let mut vec = Vec::with_capacity(10);
vec.extend([1, 2, 3]);
assert!(vec.capacity() >= 10);
vec.shrink_to_fit();
assert!(vec.capacity() >= 3);
```

## shrink_to()

```
shrink_to(&mut self, min_capacity: usize)
``` 

Reduz a capacidade do vetor com um limite inferior.

A capacidade permanecerá pelo menos tão grande quanto o comprimento e o valor fornecido.

Se a capacidade atual for menor que o limite inferior, esta é uma operação sem efeito.

```
let mut vec = Vec::with_capacity(10);
vec.extend([1, 2, 3]);
assert!(vec.capacity() >= 10);
vec.shrink_to(4);
assert!(vec.capacity() >= 4);
vec.shrink_to(0);
assert!(vec.capacity() >= 3);
```

## into_boxed_slice()

```
into_boxed_slice(self) -> Box<[T], A>
```

Converte o vetor em ```Box<[T]>```.

Antes de fazer a conversão, este método descarta o excesso de capacidade como ```shrink_to_fit()```.

### Exemplos

```
let v = vec![1, 2, 3];

let slice = v.into_boxed_slice();
```

Qualquer capacidade excedente é removida:

```
let mut vec = Vec::with_capacity(10);
vec.extend([1, 2, 3]);

assert!(vec.capacity() >= 10);
let slice = vec.into_boxed_slice();
assert_eq!(slice.into_vec().capacity(), 3);
```

## truncate()

```
truncate(&mut self, len: usize)
```
 
Reduz o tamanho do vetor, mantendo os primeiros ```len``` elementos e descartando o restante.

Se ```len``` for maior ou igual ao comprimento atual do vetor, este método não terá efeito.

O método ```drain()``` pode emular ```truncate()```, mas faz com que os elementos em excesso sejam retornados em vez de descartados.

Observe que este método não tem efeito na capacidade alocada do vetor.

### Exemplos

Reduz um vetor de cinco elementos para dois elementos:""" 

```
let mut vec = vec![1, 2, 3, 4, 5];
vec.truncate(2);
assert_eq!(vec, [1, 2]);
```

Nenhuma truncagem ocorre quando ```len``` é maior que o comprimento atual do vetor:

```
let mut vec = vec![1, 2, 3];
vec.truncate(8);
assert_eq!(vec, [1, 2, 3]);
```

Truncar quando ```len == 0``` é equivalente a chamar o método ```clear()```.

```
let mut vec = vec![1, 2, 3];
vec.truncate(0);
assert_eq!(vec, []);
```

## as_slice()

```
as_slice(&self) -> &[T]
``` 

Extrai uma slice contendo o vetor inteiro.

Equivalente a ```&s[..]```.

```
use std::io::{self, Write};
let buffer = vec![1, 2, 3, 5, 8];
io::sink().write(buffer.as_slice()).unwrap();
```

## as_mut_slice()

```
as_mut_slice(&mut self) -> &mut [T]
``` 

Extrai uma slice mutável de todo o vetor.

Equivalente a ```&mut s[..]```.

```
use std::io::{self, Read};
let mut buffer = vec![0; 3];
io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();
```

## as_ptr()

```
as_ptr(&self) -> *const T
``` 

Retorna um ponteiro bruto para o buffer do vetor, ou um ponteiro bruto pendente válido para leituras de tamanho zero se o vetor não for alocado.

O chamador deve garantir que o vetor sobreviva ao ponteiro que esta função retorna, ou então ele acabará pendurado. Modificar o vetor pode fazer com que seu buffer seja realocado, o que também tornaria quaisquer ponteiros para ele inválidos.

O chamador também deve garantir que a memória para a qual o ponteiro (não transitivamente) aponta nunca seja gravada (exceto dentro de uma ```UnsafeCell```) usando este ponteiro ou qualquer ponteiro derivado dele. Se você precisar alterar o conteúdo do slice, use ```as_mut_ptr()```.

Este método garante que, para o propósito do modelo de aliasing, este método não materialize uma referência ao slice subjacente e, portanto, o ponteiro retornado permanecerá válido quando misturado com outras chamadas para ```as_ptr()```, ```as_mut_ptr()``` e ```as_non_null()```. Note que chamar outros métodos que materializam referências mutáveis ​​para a slice, ou referências mutáveis ​​para elementos específicos que você está planejando acessar por meio deste ponteiro, bem como escrever para esses elementos, ainda pode invalidar este ponteiro. Veja o segundo exemplo abaixo para saber como esta garantia pode ser usada.

### Exemplos

```
let x = vec![1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(*x_ptr.add(i), 1 << i);
    }
}
```

Devido à garantia de aliasing, o código a seguir é legal:

```
unsafe {
    let mut v = vec![0, 1, 2];
    let ptr1 = v.as_ptr();
    let _ = ptr1.read();
    let ptr2 = v.as_mut_ptr().offset(2);
    ptr2.write(2);
    // Notavelmente, a gravação em `ptr2` *não* invalidou `ptr1`
    // porque ele alterou um elemento diferente:
    let _ = ptr1.read();
}
``` 

## as_mut_ptr()

```
as_mut_ptr(&mut self) -> *mut T
```

Retorna um ponteiro mutável bruto para o buffer do vetor, ou um ponteiro bruto pendente válido para leituras de tamanho zero se o vetor não for alocado.

O chamador deve garantir que o vetor sobreviva ao ponteiro que esta função retorna, ou então ele acabará pendurado. Modificar o vetor pode fazer com que seu buffer seja realocado, o que também tornaria quaisquer ponteiros para ele inválidos.

Este método garante que, para o propósito do modelo de aliasing, este método não materializa uma referência a slice subjacente e, portanto, o ponteiro retornado permanecerá válido quando misturado com outras chamadas para ```as_ptr()```, ```as_mut_ptr()``` e ```as_non_null()```. Observe que chamar outros métodos que materializam referências a slice, ou referências a elementos específicos que você está planejando acessar por meio deste ponteiro, ainda pode invalidar este ponteiro. Veja o segundo exemplo abaixo para saber como esta garantia pode ser usada.

### Exemplos

```
// Aloque um vetor grande o suficiente para 4 elementos.
let size = 4;
let mut x: Vec<i32> = Vec::with_capacity(size);
let x_ptr = x.as_mut_ptr();

// Inicializa elementos por meio de gravações de
// ponteiros brutos e, em seguida, define o comprimento.
unsafe {
    for i in 0..size {
        *x_ptr.add(i) = i as i32;
    }
    x.set_len(size);
}
assert_eq!(&*x, &[0, 1, 2, 3]);
```

Devido à garantia de aliasing, o código a seguir é legal:

```
unsafe {
    let mut v = vec![0];
    let ptr1 = v.as_mut_ptr();
    ptr1.write(1);
    let ptr2 = v.as_mut_ptr();
    ptr2.write(2);
    // Notavelmente, a gravação em `ptr2` *não* invalidou `ptr1`:
    ptr1.write(3);
}
```


## set_len()

<img src="images/warning_unsafe.png" width="100" alt="UNSAFE">

```
set_len(&mut self, new_len: usize)
```

Força o comprimento do vetor para **new_len**.

Esta é uma operação de baixo nível que não mantém nenhuma das invariantes normais do tipo. Normalmente, a alteração do comprimento de um vetor é feita usando uma das operações seguras, como ```truncate()```, ```resize()```, ```extend()```, ou ```clear()```.

### Segurança

* **new_len** deve ser menor ou igual a ```capacity()```;
* Os elementos em **old_len..new_len** devem ser inicializados.

### Exemplos

Este método pode ser útil para situações em que o vetor está servindo como um buffer para outro código, particularmente sobre FFI: 

```
pub fn get_dictionary(&self) -> Option<Vec<u8>> {
    // De acordo com a documentação do método FFI, "32.768 bytes são sempre suficientes".
    let mut dict = Vec::with_capacity(32_768);
    let mut dict_length = 0;
	// SEGURANÇA: Quando `deflateGetDictionary` retorna `Z_OK`, ele garante que:
	// 1. `dict_length` elementos foram inicializados.
	// 2. `dict_length` <= a capacidade (32_768)
	// o que torna `set_len` seguro para chamar.
    unsafe {
	// Faz a chamada FFI...
        let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
        if r == Z_OK {
	// ...e atualiza o comprimento para o que foi inicializado.
            dict.set_len(dict_length);
            Some(dict)
        } else {
            None
        }
    }
}
``` 

Embora o exemplo a seguir seja válido, há um vazamento de memória, pois os vetores internos não foram liberados antes da chamada a ```set_len()```:

```
let mut vec = vec![vec![1, 0, 0],
                   vec![0, 1, 0],
                   vec![0, 0, 1]];
// SEGURANÇA:
// 1. `old_len..0` está vazio, então nenhum elemento precisa ser inicializado.
// 2. `0 <= capacity` sempre contém qualquer `capacity` que seja.
unsafe {
    vec.set_len(0);
}
```

Normalmente, aqui, ```clear()``` deveria ter sido usado para descartar corretamente o conteúdo e, portanto, não vazar memória.

## swap_remove()

```
swap_remove(&mut self, index: usize) -> T
```

Remove um elemento do vetor e o retorna.

O elemento removido é substituído pelo último elemento do vetor.

Isso não preserva a ordem dos elementos restantes, mas é O(1). Se você precisar preservar a ordem dos elementos, use ```remove()```.

### Pânico

Entra em pânico se o índice estiver fora dos limites.

```
let mut v = vec!["foo", "bar", "baz", "qux"];

assert_eq!(v.swap_remove(1), "bar");
assert_eq!(v, ["foo", "qux", "baz"]);

assert_eq!(v.swap_remove(0), "foo");
assert_eq!(v, ["baz", "qux"]);
```

## insert()

```
insert(&mut self, index: usize, element: T)
``` 
 
Insere um elemento na posição **index**, deslocando todos os elementos após ele para a direita.

### Pânico

Entra em pânico se **index** é maior que ```len()```.

### Exemplos

```
let mut vec = vec![1, 2, 3];
vec.insert(1, 4);
assert_eq!(vec, [1, 4, 2, 3]);
vec.insert(4, 5);
assert_eq!(vec, [1, 4, 2, 3, 5]);
```

### Complexidade

**O(Vec::len)**.

Todos os itens após o índice de inserção devem ser deslocados para a direita. No pior caso, todos os elementos são deslocados quando o índice de inserção é 0. 

## remove()

```
remove(&mut self, index: usize) -> T
```

Remove e retorna o elemento na posição **index**, deslocando todos os elementos após ele para a esquerda.

Observação: Como este método desloca os elementos restantes, ele tem um desempenho de pior caso de **O(n)**. Se você não precisar que a ordem dos elementos seja preservada, use ```swap_remove()```. Se você quiser remover elementos do início, considere usar ```VecDeque::pop_front()```.

### Pânico

Entra em pânico se **index** estiver fora dos limites.

```
let mut v = vec![1, 2, 3];
assert_eq!(v.remove(1), 2);
assert_eq!(v, [1, 3]);
```

## retain()

```
retain<F>(&mut self, f: F)
where
    F: FnMut(&T) -> bool,
``` 
 
Mantém apenas os elementos especificados pelo predicado (closure passada como argumento).

Em outras palavras, remove todos os elementos **e** para os quais **f(&e)** retorna **false**. Este método opera no local, visitando cada elemento exatamente uma vez na ordem original, e preserva a ordem dos elementos retidos.

### Exemplos

```
let mut vec = vec![1, 2, 3, 4];
vec.retain(|&x| x % 2 == 0);
assert_eq!(vec, [2, 4]);
```

Como os elementos são visitados exatamente uma vez na ordem original, o estado externo pode ser usado para decidir quais elementos manter. 

```
let mut vec = vec![1, 2, 3, 4, 5];
let keep = [false, true, true, false, true];
let mut iter = keep.iter();
vec.retain(|_| *iter.next().unwrap());
assert_eq!(vec, [2, 3, 5]);
```

## retain_mut()

```
retain_mut<F>(&mut self, f: F)
where
    F: FnMut(&mut T) -> bool,
```

Mantém apenas os elementos especificados pelo predicado(closure passada como argumento), passando uma referência mutável para o elemento.

Em outras palavras, remove todos os elementos e tais que **f(&mut e)** retorna **false**. Este método opera no local, visitando cada elemento exatamente uma vez na ordem original, e preserva a ordem dos elementos retidos.

```
let mut vec = vec![1, 2, 3, 4];
vec.retain_mut(|x| if *x <= 3 {
    *x += 1;
    true
} else {
    false
});
assert_eq!(vec, [2, 3, 4]);
```

## dedup_by_key()

```
dedup_by_key<F, K>(&mut self, key: F)
where
    F: FnMut(&mut T) -> K,
    K: PartialEq,
```

Remove todos, exceto o primeiro, dos elementos consecutivos no vetor que resolvem para a mesma chave.


```
let mut vec = vec![10, 20, 21, 30, 20];
    
vec.dedup_by_key(|i| *i / 10);
    
assert_eq!(vec, [10, 20, 30, 20]);
```

Se o vetor estiver ordenado, isso remove todas as duplicatas:

```   
let mut vec2 = vec![10, 20, 20, 30, 20, 40, 40, 50];
    
vec2.dedup_by_key(|i| *i / 10);
    
assert_eq!(vec2, [10, 20, 30, 20, 40, 50]);
```

## dedup_by()

```
dedup_by<F>(&mut self, same_bucket: F)
where
    F: FnMut(&mut T, &mut T) -> bool,
```

Remove todos, exceto o primeiro, dos elementos consecutivos no vetor que satisfazem uma determinada relação de igualdade.

A função ```same_bucket()``` recebe referências a dois elementos do vetor e deve determinar se os elementos são iguais. Os elementos são passados na ordem oposta à sua ordem na fatia, então se ```same_bucket(a, b)``` retornar **true**, **a** é removido.

Se o vetor estiver ordenado, isso remove todas as duplicatas.

```
let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];

vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
```

## push()

```
push(&mut self, value: T)
```

Insere um elemento no final do vetor.

### Pânico

Entra em pânico se a nova capacidade exceder ```isize::MAX bytes```.

### Exemplo

```
let mut vec = vec![1, 2];
vec.push(3);
assert_eq!(vec, [1, 2, 3]);
```

### Complexidade

Leva tempo amortizado **O(1)**. Se o comprimento do vetor exceder sua capacidade após ```push()```, **O(capacidade)** de tempo é gasto para copiar os elementos do vetor para uma alocação maior. Essa operação cara é compensada pelas inserções **O(1)** dentro da capacidade que ela permite. 

## pop()

```
pop(&mut self) -> Option<T>
```
 
Remove o último elemento do vetor e o retorna, ou ```None``` se estiver vazio.

Se você quiser remover o primeiro elemento, considere usar ```VecDeque::pop_front()``` em vez disso.

### Exemplos

```
let mut vec = vec![1, 2, 3];
assert_eq!(vec.pop(), Some(3));
assert_eq!(vec, [1, 2]);
```

### Complexidade

<font color="green">**Complexidade O(1)**</font>  

## append()

```
append(&mut self, other: &mut Vec<T, A>)
```

Move todos os elementos de **other** para **self,** deixando **other** vazio.

### Pânico

Entra em pânico se a nova capacidade exceder ```isize::MAX bytes``` ou se os tipos dos elementos dos vetores forem diferentes.

```
let mut vec = vec![1, 2, 3];
let mut vec2 = vec![4, 5, 6];
vec.append(&mut vec2);
assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
assert_eq!(vec2, []);
```

## drain()

```
drain<R>(&mut self, range: R) -> Drain<'_, T, A>
where
    R: RangeBounds<usize>,
```

Remove o intervalo especificado do vetor, retornando todos os elementos removidos como um iterador. Se o iterador for descartado antes de ser totalmente consumido, ele descarta os elementos removidos restantes.

O iterador retornado mantém um empréstimo mutável no vetor para otimizar sua implementação.

### Pânico

Entra em pânico se o ponto de partida for maior que o ponto final ou se o ponto final for maior que o comprimento do vetor. 

### Vazamento

Se o iterador retornado sair do escopo sem ser descartado (devido a ```mem::forget()```, por exemplo), o vetor pode ter perdido e vazado elementos arbitrariamente, incluindo elementos fora do intervalo.

```
let mut v = vec![1, 2, 3];
let u: Vec<_> = v.drain(1..).collect();
assert_eq!(v, &[1]);
assert_eq!(u, &[2, 3]);

// Um ​​intervalo completo limpa o vetor, como `clear()` faz
v.drain(..);
assert_eq!(v, &[]);
```

## clear()

```
clear(&mut self)
```

Limpa o vetor, removendo todos os valores.

Observe que este método não tem efeito na capacidade alocada do vetor. 

```
let mut v = vec![1, 2, 3];

v.clear();

assert!(v.is_empty());
```

## len()
```
len(&self) -> usize
```

Retorna o número de elementos no vetor, também conhecido como seu 'comprimento'. 

```
let a = vec![1, 2, 3];
assert_eq!(a.len(), 3);
```

## is_empty()

```
is_empty(&self) -> bool
```
 

Retorna ```true``` se o vetor não contém elementos.

```
let mut v = Vec::new();
assert!(v.is_empty());

v.push(1);
assert!(!v.is_empty());
```

## split_off()

```
split_off(&mut self, at: usize) -> Vec<T, A>
where
    A: Clone,
```

Divide o vetor em duas partes no índice fornecido.

Retorna um novo vetor alocado contendo os elementos no intervalo **[at, len)**. Após a chamada, o vetor original será deixado contendo os elementos **[0, at)** com sua capacidade anterior inalterada.

* Se você deseja assumir a propriedade de todo o conteúdo e capacidade do vetor, consulte ```mem::take()``` ou ```mem::replace()```.
* Se você não precisa do vetor retornado, consulte ```Vec::truncate()```.
* Se você deseja assumir a propriedade de uma subslice arbitrária, ou não precisa necessariamente armazenar os itens removidos em um vetor, consulte ```Vec::drain()```. 

### Pânico

Entra em pânico se **at > len**.

```
let mut vec = vec![1, 2, 3];
let vec2 = vec.split_off(1);
assert_eq!(vec, [1]);
assert_eq!(vec2, [2, 3]);
```

## resize_with()

```
resize_with<F>(&mut self, new_len: usize, f: F)
where
    F: FnMut() -> T,
``` 

Redimensiona o vetor para que **len** seja igual a **new_len**.

Se **new_len** for maior que **len**, o ```Vec``` é estendido pela diferença, com cada slot adicional preenchido com o resultado da chamada da closure **f**. Os valores de retorno de **f** acabarão no ```Vec``` na ordem em que foram gerados.

Se **new_len** for menor que **len**, o ```Vec``` é simplesmente truncado.

Este método usa uma closure para criar novos valores em cada **push**. Se você preferir clonar um valor fornecido, use ```Vec::resize()```. Se quiser usar a traço ```Default``` para gerar valores, você pode passar ```Default::default``` como o segundo argumento.

```
let mut vec = vec![1, 2, 3];
vec.resize_with(5, Default::default);
assert_eq!(vec, [1, 2, 3, 0, 0]);

let mut vec = vec![];
let mut p = 1;
vec.resize_with(4, || { p *= 2; p });
assert_eq!(vec, [2, 4, 8, 16]);
```

## leak()

```
leak<'a>(self) -> &'a mut [T]
where
    A: 'a,
```

Consome e vaza o ```Vec```, retornando uma referência mutável ao conteúdo, ```&'a mut [T]```.

Observe que o tipo **T** deve sobreviver ao tempo de vida escolhido **'a**. Se o tipo tiver apenas referências estáticas, ou nenhuma, então isso pode ser escolhido para ser **'static**.

A partir do Rust 1.57, esse método não realoca ou reduz o ```Vec```, então a alocação vazada pode incluir capacidade não utilizada que não faz parte da slice retornada.

Essa função é útil principalmente para dados que vivem pelo restante da vida do programa.

<img src="images/warning.png" width="100" alt="WARNING"> Descartar a referência retornada causará um vazamento de memória. 

```
let x = vec![1, 2, 3];
let static_ref: &'static mut [usize] = x.leak();
static_ref[0] += 1;
assert_eq!(static_ref, &[2, 2, 3]);
```

## spare_capacity_mut()

```
spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>]
```

Retorna a capacidade de reserva restante do vetor como uma slice de ```MaybeUninit<T>```.

A slice retornada pode ser usada para preencher o vetor com dados (por exemplo, lendo de um arquivo) antes de marcar os dados como inicializados usando o método ```set_len()```.

```
// Aloca um vetor grande o suficiente para 10 elementos.
let mut v = Vec::with_capacity(10);

// Preencha os 3 primeiros elementos.
let uninit = v.spare_capacity_mut();
uninit[0].write(0);
uninit[1].write(1);
uninit[2].write(2);

// Marca os 3 primeiros elementos do vetor como inicializados.
unsafe {
    v.set_len(3);
}

assert_eq!(&v, &[0, 1, 2]);
```

## resize()

```
resize(&mut self, new_len: usize, value: T)
```

Redimensiona o ```Vec``` para que **len** seja igual a **new_len**.

Se **new_len** for maior que **len**, o ```Vec``` é estendido pela diferença, com cada slot adicional preenchido com **value**. Se **new_len** for menor que **len**, o ```Vec``` é simplesmente truncado.

Este método requer que **T** implemente ```Clone```, para poder clonar o valor passado. Se você precisar de mais flexibilidade (ou quiser depender de ```Default``` em vez de ```Clone```), use ```Vec::resize_with()```. Se você precisar apenas redimensionar para um tamanho menor, use ```Vec::truncate()```.

```
let mut vec = vec!["hello"];
vec.resize(3, "world");
assert_eq!(vec, ["hello", "world", "world"]);

let mut vec = vec![1, 2, 3, 4];
vec.resize(2, 0);
assert_eq!(vec, [1, 2]);
```

## extend_from_slice()

```
extend_from_slice(&mut self, other: &[T])
``` 

Clona e anexa todos os elementos de uma slice ao ```Vec```.

Itera sobre a slice **other**, clona cada elemento e, em seguida, o anexa a este ```Vec```. A slice **other** é percorrida em ordem.

Observe que esta função é a mesma que ```extend()```, exceto que é especializada para funcionar com slices. Se e quando o Rust obtiver especialização, esta função provavelmente será depreciada (mas ainda estará disponível).

```
let mut vec = vec![1];
vec.extend_from_slice(&[2, 3, 4]);
assert_eq!(vec, [1, 2, 3, 4]);
```

## extend_from_within()

```
extend_from_within<R>(&mut self, src: R)
```

Copia elementos do intervalo **src** para o final do vetor.

### Pânico 

Entra em pânico se o ponto de partida for maior que o ponto final ou se o ponto final for maior que o comprimento do vetor. 

```
let mut vec = vec![0, 1, 2, 3, 4];

vec.extend_from_within(2..);
assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4]);

vec.extend_from_within(..2);
assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);

vec.extend_from_within(4..8);
assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);
```

## into_flattened()

```
into_flattened(self) -> Vec<T, A>
```

Recebe um ```Vec<[T; N]>``` e o achata em um ```Vec<T>```.

### Pânico

Entra em pânico se o comprimento do vetor resultante exceder o tamanho máximo do tipo ```usize```.

Isso só é possível ao achatar um vetor de matrizes de tipos de tamanho zero, e, portanto, tende a ser irrelevante na prática. Se ```size_of::<T>() > 0```, isso nunca causará pânico. 
 

### Exemplos

```
let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
let mut flattened = vec.into_flattened();
assert_eq!(flattened, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
```

```
let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
assert_eq!(vec.pop(), Some([7, 8, 9]));
assert_eq!(vec, [[1, 2, 3], [4, 5, 6]]);

let mut flattened = vec.into_flattened();
assert_eq!(flattened, [1, 2, 3, 4, 5, 6]);
assert_eq!(flattened.pop(), Some(6));
```

## dedup()

```
dedup(&mut self)
```

Remove elementos repetidos consecutivos no vetor de acordo com a implementação da trait ```PartialEq```.

Se o vetor estiver ordenado, isso remove todas as duplicatas.

```
let mut vec = vec![1, 2, 2, 3, 2];

vec.dedup();

assert_eq!(vec, [1, 2, 3, 2]);
```

## splice()

```
splice<R, I>(
    &mut self,
    range: R,
    replace_with: I,
) -> Splice<'_, <I as IntoIterator>::IntoIter, A> ⓘ
where
    R: RangeBounds<usize>,
    I: IntoIterator<Item = T>,
```

Cria um iterador que substitui o intervalo especificado no vetor pelo iterador **replace_with** fornecido e produz os itens removidos. **replace_with** não precisa ter o mesmo comprimento que o intervalo.

O intervalo é removido mesmo que o iterador não seja consumido até o final.

Não é especificado quantos elementos são removidos do vetor se o valor ```Splice``` for vazado.

O iterador de entrada **replace_with** só é consumido quando o valor ```Splice``` é descartado.

Isso é ótimo se: 

* A cauda (elementos no vetor após o intervalo) está vazia,
* ou **replace_with** gera menos ou igual elementos do que o comprimento do intervalo,
* ou o limite inferior do seu ```size_hint()``` é exato. 

Caso contrário, um vetor temporário é alocado e a cauda é movida duas vezes. 

### Pânico

Entra em pânico se o ponto de partida for maior que o ponto final ou se o ponto final for maior que o comprimento do vetor. 

```
let mut v = vec![1, 2, 3, 4];
let new = [7, 8, 9];
let u: Vec<_> = v.splice(1..3, new).collect();
assert_eq!(v, &[1, 7, 8, 9, 4]);
assert_eq!(u, &[2, 3]);
```

---

## Referências

[std::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

---

arataca89@gmail.com

Última atualização: 20241220
