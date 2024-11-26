# Linguagem Rust - estruturas

Uma ```struct```, ou estrutura, é um tipo de dado criado pelo programador que permite encapsular vários valores relacionados de modo que o tipo de dado criado melhore o significado do seu uso e a compreensão das diversas partes do programa. Se você estiver familiarizado com uma linguagem orientada a objetos, uma ```struct``` é como os atributos de dados de um objeto. 

* [Definindo e instanciando estruturas](#definindo-e-instanciando-estruturas)

* [Usando a sintaxe abreviada para inicializar os campos da struct](#usando-a-sintaxe-abreviada-para-inicializar-os-campos-da-struct)

* [Criando instâncias a partir de outras instâncias](#criando-instâncias-a-partir-de-outras-instâncias)

---

## Definindo e instanciando estruturas

Uma estrutura pode possuir vários campos nomeados que podem ser de tipos diferentes. 


Para definir uma ```struct``` usa-se a seguinte sintaxe:

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

Para definir uma estrutura usamos a palavra-chave ```struct```, seguida pelo nome da estrutura, neste caso ```User```, seguida dos campos da estrutura entre chaves.

Cada campo da estrutura tem um nome, seguido pelo caractere de dois pontos, seguido pelo tipo de dados do campo, seguido pelo caractere vírgula.

Neste exemplo, observe que a estrutura encapsula todos os dados referentes a um usuário de um sistema: se ele está ou não ativo, seu nome de usuário, seu email e quantas vezes ele logou no sistema.

Para usar uma ```struct``` depois de defini-la, criamos uma instância dessa ```struct``` especificando valores concretos para cada um dos campos. Criamos uma instância declarando o nome da ```struct``` e então adicionamos chaves contendo pares chave:valor, onde as chaves são os nomes dos campos e os valores são os dados que queremos armazenar nesses campos. Não precisamos especificar os campos na mesma ordem em que os declaramos na ```struct```. Em outras palavras, a definição da ```struct``` é como um modelo geral para o tipo, e as instâncias preenchem esse modelo com dados específicos para criar valores do tipo. Por exemplo, podemos declarar um usuário específico conforme mostrado abaixo.

```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

Para obter um valor específico de uma ```struct```, usamos a notação de ponto. Por exemplo, para acessar o endereço de e-mail deste usuário, usamos ```user1.email```. Se a instância for mutável, podemos alterar um valor usando a notação de ponto e atribuindo novo valor a um campo específico. Abaixo mostramos como alterar o valor no campo de e-mail de uma instância ```User``` mutável.

```
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Observe que toda a instância deve ser mutável; Rust não permite que marquemos apenas certos campos como mutáveis. 

Como em qualquer expressão, podemos construir uma nova instância da estrutura como a última expressão no corpo da função para retornar implicitamente essa nova instância. O código abaixo mostra uma função ```build_user()``` que retorna uma instância de ```User``` com o email e nome de usuário fornecidos. O campo ```active``` recebe o valor ```true``` e o ```sign_in_count``` recebe o valor 1.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

Faz sentido nomear os parâmetros da função com o mesmo nome dos campos da estrutura, mas ter que repetir os nomes dos campos e variáveis de email e nome de usuário é um pouco tedioso. Se a estrutura tivesse mais campos, repetir cada nome ficaria ainda mais irritante. Felizmente, existe um atalho conveniente! 

## Usando a sintaxe abreviada para inicializar os campos da struct

Como os nomes dos parâmetros e os nomes dos campos da estrutura são exatamente iguais no código acima, podemos usar a sintaxe abreviada de inicialização de campo para reescrever ```build_user()``` para que se comporte exatamente da mesma forma, mas não tenha a repetição de ```username``` e ```email```.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Aqui, estamos criando uma nova instância da estrutura ```User```, que possui um campo chamado ```email```. Queremos definir o valor do campo ```email``` para o valor no parâmetro de mesmo nome, ```email``` da função ```build_user()```. Como o campo ```email``` e o parâmetro ```email``` têm o mesmo nome, não precisamos escrever ```email: email``` , podemos escrever apenas ```email``` e o Rust vai entender que queremos usar o valor passado para a função no parâmetro de mesmo nome.

## Criando instâncias a partir de outras instâncias 

asd 
 
 

 

 



---

## Referências
[Capítulo 5 do Livro](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

arataca89@gmail.com

Última atualização: 20241126