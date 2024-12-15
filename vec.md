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
	- [new()](#new) - Constrói um novo ```Vec<T>``` vazio.
	- [with_capacity()](#with_capacity) - Constrói um novo ```Vec<T>``` vazio com pelo menos a capacidade especificada.
	- [from_raw_parts()](#from_raw_parts) - Cria um ```Vec<T>``` diretamente de um ponteiro, um comprimento e uma capacidade.
	- [capacity()](#capacity) - Retorna o número total de elementos que o vetor pode conter sem realocar.
	- [reserve()](#reserve) - Reserva capacidade para mais elementos.
	- [reserve_exact()](#reserve_exact) - Reserva capacidade mínima para mais elementos.	
	- [try_reserve()](#try_reserve) - Tenta reservar capacidade para mais elementos.

	
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

Se a capacidade transbordar ou o alocador relatar uma falha, um erro será retornado. 

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

asd



---

## Referências

[std::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

---

arataca89@gmail.com

Última atualização: 20241215
