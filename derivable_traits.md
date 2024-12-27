#### arataca89

# Traits deriváveis

O atributo **derive** pode ser usado na definição de uma ```struct``` ou ```enum``` para implementar determinada **trait**  no tipo sendo criado. A **trait** criada terá uma implementação padrão do Rust. Se você deseja um comportamento diferente, consulte a [documentação da biblioteca padrão](https://doc.rust-lang.org/std/index.html) para cada **trait** para obter detalhes sobre como implementar a **trait** manualmente.

As **traits** listadas aqui são as únicas definidas pela biblioteca padrão que podem ser implementados em seus tipos usando **derive**. Outras traits definidas na biblioteca padrão não têm comportamento padrão sensato, então cabe a você implementá-las da maneira que fizer sentido para o que você está tentando realizar.

Um exemplo de **trait** que não pode ser derivada é **Display**, que lida com a formatação para usuários finais. Você deve sempre considerar a maneira apropriada de exibir um tipo para um usuário final. Quais partes do tipo um usuário final deve ter permissão para ver? Quais partes eles considerariam relevantes? Qual formato dos dados seria mais relevante para eles? O compilador Rust não tem essa percepção, então ele não pode fornecer o comportamento padrão apropriado para você.

A lista de traits deriváveis fornecida aqui não é completa: bibliotecas podem implementar **derive** para suas traits, tornando a lista de traits onde você pode usar **derive** realmente aberta. Implementar **derive** envolve o uso de uma macro procedural, que é abordada em [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#macros). 
 
## Debug

A trait ```Debug``` permite formatação para depuração que você indica adicionando **:?** dentro de placeholders indicados por **{}**. 

A trait ```Debug``` permite que você imprima instâncias de um tipo para fins de depuração, para que você e outros programadores que usam seu tipo possam inspecionar uma instância em um determinado ponto da execução de um programa.

A trait ```Debug``` é necessária, por exemplo, ao usar a macro ```assert_eq!```. Esta macro imprime os valores das instâncias fornecidas como argumentos se a asserção de igualdade falhar, para que os programadores possam ver por que as duas instâncias não eram iguais.

## PartialEq e Eq

A trait ```PartialEq``` permite que você compare instâncias de um tipo para verificar a igualdade e permite o uso dos operadores **==** e **!=**.

Derivar ```PartialEq``` implementa o método ```eq()```. Quando ```PartialEq``` é derivada em structs, duas instâncias são iguais apenas se todos os campos forem iguais, e as instâncias não são iguais se algum campo não for igual. Quando derivado em enums, cada variante é igual a si mesma e não é igual às outras variantes.

A trait ```PartialEq``` é necessária, por exemplo, com o uso da macro ```assert_eq!```, que precisa ser capaz de comparar duas instâncias de um tipo para igualdade.

A trait ```Eq``` não possui métodos. Seu propósito é sinalizar que para cada valor do tipo anotado, o valor é igual a si mesmo. A trait ```Eq``` só pode ser aplicada a tipos que também implementam ```PartialEq```, embora nem todos os tipos que implementam ```PartialEq``` possam implementar ```Eq```. Um exemplo disso são os tipos de números de ponto flutuante: a implementação de números de ponto flutuante afirma que duas instâncias do valor não-um-número (**NaN**) não são iguais entre si.

Um exemplo de quando ```Eq``` é necessária é para chaves em um ```HashMap<K, V>``` para que o ```HashMap<K, V>``` possa dizer se duas chaves são iguais.

## PartialOrd e Ord 

A trait ```PartialOrd``` permite que você compare instâncias de um tipo para fins de ordenação. Um tipo que implementa ```PartialOrd``` pode ser usado com os operadores **<**, **>**, **<=** e **>=**. Você só pode aplicar a trait ```PartialOrd``` a tipos que também implementam ```PartialEq```.

Derivar ```PartialOrd``` implementa o método ```partial_cmp()```, que retorna um ```Option<Ordering>``` que será ```None``` quando os valores fornecidos não produzirem uma ordenação. Um exemplo de um valor que não produz uma ordenação, embora a maioria dos valores desse tipo possa ser comparada, é o valor de ponto flutuante não-um-número (```NaN```). Chamar ```partial_cmp()``` com qualquer número de ponto flutuante e o valor de ponto flutuante ```NaN``` retornará ```None```. 
 
Quando derivado em structs, ```PartialOrd``` compara duas instâncias comparando o valor em cada campo na ordem em que os campos aparecem na definição da ```struct```. Quando derivado em enums, variantes da ```enum``` declaradas anteriormente na definição da ```enum``` são consideradas menores que as variantes listadas posteriormente.

A trait ```PartialOrd``` é necessária, por exemplo, para o método ```gen_range()``` do crate **rand** que gera um valor aleatório no intervalo especificado por uma expressão de intervalo. 

A trait ```Ord``` permite que você saiba que para quaisquer dois valores do tipo anotado, uma ordenação válida existirá. A trait ```Ord``` implementa o método ```cmp()```, que retorna um ```Ordering``` em vez de um ```Option<Ordering>``` porque uma ordenação válida sempre será possível. Você só pode aplicar a trait ```Ord``` a tipos que também implementam ```PartialOrd``` e ```Eq``` (e ```Eq``` requer ```PartialEq```). Quando derivado em structs e enums, ```cmp()``` se comporta da mesma forma que a implementação derivada para ```partial_cmp()``` faz com ```PartialOrd```. 

Um exemplo de quando ```Ord``` é necessário é ao armazenar valores em um ```BTreeSet<T>```, uma estrutura de dados que armazena dados com base na ordem de classificação dos valores. 

## Clone e Copy

A trait ```Clone``` permite que você crie explicitamente uma cópia profunda de um valor, e o processo de duplicação pode envolver a execução de código arbitrário e a cópia de dados da memória heap. 
 
Derivar ```Clone``` implementa o método ```clone()```, que quando implementado para o tipo todo, chama ```clone()``` em cada uma das partes do tipo. Isso significa que todos os campos ou valores do tipo considerado também devem implementar ```Clone``` para o tipo poder derivar ```Clone```. 

Um exemplo de quando ```Clone``` é necessário é ao chamar o método ```to_vec()``` em uma slice. A slice não possui a propriedade das instâncias de tipo que ela contém, mas o vetor retornado de ```to_vec()``` precisará ter a propriedade destas instâncias, então ```to_vec()``` chama ```clone()``` em cada item. Portanto, o tipo armazenado na slice deve implementar ```Clone```. 

A trait ```Copy``` permite que você duplique um valor copiando apenas bits armazenados na região de memória da pilha(stack); nenhum código arbitrário é necessário.

A trait ```Copy``` não define nenhum método para impedir que programadores sobrecarreguem esses métodos e violem a suposição de que nenhum código arbitrário está sendo executado. Dessa forma, todos os programadores podem assumir que copiar um valor será muito rápido. 

Você pode derivar ```Copy``` em qualquer tipo cujas partes implementem ```Copy```. Um tipo que implementa ```Copy``` também deve implementar ```Clone```, porque um tipo que implementa ```Copy``` tem uma implementação trivial de ```Clone``` que realiza a mesma tarefa que ```Copy```.

A trait ```Copy``` raramente é necessária; tipos que implementam ```Copy``` têm otimizações disponíveis, o que significa que você não precisa chamar ```clone()```, o que torna o código mais conciso.

Tudo o que é possível com ```Copy``` você também pode realizar com ```Clone```, mas o código pode ser mais lento ou ter que usar ```clone()``` em alguns lugares. 

## Hash

A trait Hash permite que você pegue uma instância de um tipo de tamanho arbitrário e mapeie essa instância para um valor de tamanho fixo usando uma função hash. Derivar ```Hash``` implementa o método ```hash()```. A implementação derivada do método ```hash()``` combina o resultado da chamada de ```hash()``` em cada uma das partes do tipo, o que significa que todos os campos ou valores também devem implementar ```Hash``` para derivar ```Hash```.

Um exemplo de quando ```Hash``` é necessário é no armazenamento de chaves em um ```HashMap<K, V>``` para armazenar dados de forma eficiente.

## Default 

A trait ```Default``` permite que você crie um valor padrão para um tipo. Derivar ```Default``` implementa a função ```default()```. A implementação derivada da função ```default()``` chama a função ```default()``` em cada parte do tipo, o que significa que todos os campos ou valores no tipo também devem implementar ```Default``` para derivar ```Default```. 

A função ```Default::default()``` é comumente usada em combinação com a [sintaxe de atualização de struct](structs.md#criando-instâncias-a-partir-de-outras-instâncias). Você pode personalizar alguns campos de uma ```struct``` e, em seguida, definir e usar um valor padrão para o restante dos campos usando ```..Default::default()```.

A trait ```Default``` é necessária quando você usa o método ```unwrap_or_default()``` em instâncias ```Option<T>```, por exemplo. Se o ```Option<T>``` for ```None```, o método ```unwrap_or_default()``` retornará o resultado de ```Default::default()``` para o tipo ```T``` armazenado no ```Option<T>```. 

## Referências

[Apêndice C do Livro](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

---

arataca89@gmail.com

Última atualização: 20241227