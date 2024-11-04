# RBE - Módulos

Rust fornece um sistema de módulos poderoso que pode ser usado para dividir hierarquicamente o código em unidades lógicas (módulos) e gerenciar a visibilidade (pública/privada) entre elas.

Um módulo é uma coleção de itens: funções, structs, traits, blocos impl e até mesmo outros módulos. 

* [Visibilidade](#Visibilidade)
* [Visibilidade de struct](#Visibilidade-de-struct)
* [A declaração use](#A-declaração-use)
* [super e self](#super-e-self)
* [Hierarquia de arquivos](#Hierarquia-de-arquivos)

---

## Visibilidade

Por padrão, os itens em um módulo têm visibilidade privada, mas isso pode ser substituído pelo modificador ```pub```. Somente os itens públicos de um módulo podem ser acessados de fora do escopo do módulo.

```
// Um módulo chamado 'my_mod'
mod my_mod {
    // Os itens em um módulo tem visibilidade padrão privada
    fn private_function() {
        println!("'my_mod::private_function()' foi chamada.");
    }

    // Use o modificador 'pub' para tornar um item público.
    pub fn function() {
        println!("'my_mod::function()' foi chamada.");
    }

    // Itens em um módulo podem acessar outros itens do módulo,
    // mesmo que sejam privados.
    pub fn indirect_access() {
        print!("'my_mod::indirect_access()' chamando...\n> ");
        private_function();
    }

    // Módulos podem ser aninhados, um dentro do outro.
    pub mod nested {
        pub fn function() {
            println!("'my_mod::nested::function()' foi chamada.");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("'my_mod::nested::private_function()' foi chamada.");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module

        // Funções declaradas usando a sintaxe 'pub(in path)' são visíveis apenas
        // dentro do 'path' fornecido. 'path' deve ser um módulo pai ou ancestral.
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("'my_mod::nested::public_function_in_my_mod()' chamando...\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private

        // As funções declaradas usando a sintaxe 'pub(self)' são visíveis apenas dentro
        // do módulo atual, o que é o mesmo que deixá-las privadas.
        pub(self) fn public_function_in_nested() {
            println!("'my_mod::nested::public_function_in_nested()' foi chamada.");
        }

        // Funções declaradas usando a sintaxe 'pub(super)' são visíveis somente dentro
        // do módulo pai.
        pub(super) fn public_function_in_super_mod() {
            println!("'my_mod::nested::public_function_in_super_mod()' foi chamada.");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("'my_mod::call_public_function_in_my_mod()' chamando...\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // 'pub(crate)' deixa a função visível apenas dentro do crate atual.
    pub(crate) fn public_function_in_crate() {
        println!("'my_mod::public_function_in_crate()' foi chamada.");
    }

    // Módulos aninhados seguem as mesmas regras de visibilidade.
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("'my_mod::private_nested::function()' foi chamada.");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.

        // Itens privados do pai ainda restringirão a visibilidade de um item filho,
        // mesmo que ele seja declarado como visível dentro de um escopo maior.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("'my_mod::private_nested::restricted_function()' foi chamada.");
        }
    }
}

fn function() {
    println!("'function()' foi chamada.");
}

fn main() {
    // Os módulos permitem a desambiguação entre itens que têm o mesmo nome.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.

    // Itens públicos, incluindo aqueles dentro de módulos aninhados, podem ser
    // acessados de fora do módulo pai.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // Itens 'pub(crate)' podem ser chamados de qualquer lugar no mesmo crate.
    my_mod::public_function_in_crate();

    // Itens 'pub(in path)' só podem ser chamados dentro do módulo especificado.
    // ERRO! A função 'public_function_in_my_mod()' é privada.
    //my_mod::nested::public_function_in_my_mod();

    // Itens privados de um módulo não podem ser acessados diretamente, mesmo se
    // aninhados em um módulo público:

    // ERRO! 'private_function' é privada
    //my_mod::private_function();

    // ERRO! 'private_function' é privada
    //my_mod::nested::private_function();

    // ERRO! 'private_nested' é um módulo privado
    //my_mod::private_nested::function();

    // ERRO! 'private_nested' é um módulo privado
    //my_mod::private_nested::restricted_function();
}
```

---

## Visibilidade de struct

Estruturas (```struct```) possuem um nível extra de visibilidade com seus campos. A visibilidade padrão é privada, e pode ser sobrescrita com o modificador ```pub```. Essa visibilidade só importa quando uma estrutura é acessada de fora do módulo onde ela é definida, e tem o objetivo de esconder informação (encapsulamento). 

```
mod my_module {
    // Ums struct pública com um campo público do tipo genérico 'T'
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Uma struct pública com um campo privado do tipo genérico 'T'
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T: ToString> ClosedBox<T>{
        // Um método construtor público
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }

        // Um método público para acessar um campo privado.
        pub fn get_contents(&self) -> String {
            let s = self.contents.to_string();
            s
        }
    }
}

fn main() {
    // struct pública com campo público pode ser construída como de costume
    let open_box = my_module::OpenBox { contents: "informação pública" };

    // e seus campos podem ser acessados normalmente.
    println!("O objeto 'open_box' contém: {}", open_box.contents);

    // struct pública com campo privado não pode ser construída usando seus nomes de campo.
    // ERRO! 'ClosedBox' tem campos privados
    //let closed_box = my_module::ClosedBox { contents: "informação privada" };

    // Porém, uma struct com campos privados pode ser construída usando
    // um construtor público
    let closed_box = my_module::ClosedBox::new("informação privada");

    // e campos privados de uma struct pública não podem ser acessados diretamente.
    // ERRO! O campo 'contents' é privado.
    //println!("O objeto 'closed_box' contém: {}", closed_box.contents);

    // Porém, podem ser acessados por métodos públicos.
    println!("O objeto 'closed_box' contém: {}", closed_box.get_contents());
}
```

---

## A declaração use

A declaração ```use``` pode ser usada para vincular um caminho completo a um novo nome, para acesso mais fácil. É frequentemente usada desta forma:

```
// Vincula o path 'deeply::nested::function' ao nome 'other_function'.
use deeply::nested::function as other_function;

fn function() {
    println!("'function()' foi chamada");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("'deeply::nested::function()' foi chamada");
        }
    }
}

fn main() {
    // Acesso mais fácil a 'deeply::nested::function'
    other_function();

    println!("Entrando no bloco");
    {
        // Isto é equivalente a 'use deeply::nested::function as function'.
        
        // Esta 'function()' irá sombrear a 'function()' do escopo global
        use crate::deeply::nested::function;

        // 'use' tem um escopo local. Neste caso, o
        // sombreamento de 'function()' está somente neste bloco.
        function();

        println!("Saindo do bloco");
    }

    function();
}
```

## super e self

As palavras-chave ```super``` e ```self``` podem ser usadas no path para remover ambiguidade ao acessar itens e para evitar hardcode desnecessário de caminhos (paths). 

```
fn function() {
    println!("'function()' foi chamada");
}

mod cool {
    pub fn function() {
        println!("'cool::function()' foi chamada");
    }
}

mod my {
    fn function() {
        println!("'my::function()' foi chamada");
    }
    
    mod cool {
        pub fn function() {
            println!("'my::cool::function()' foi chamada");
        }
    }
    
    pub fn indirect_call() {
        // Vamos acessar todas as funções chamadas 'function' a partir deste escopo!
        print!("'my::indirect_call()' foi chamada...\n");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.

        // A palavra-chave 'self' se refere ao escopo do módulo atual - neste caso 'my'.
        // Chamar 'self::function()' e chamar 'function()' diretamente dão
        // o mesmo resultado, porque se referem à mesma função.
        self::function();
        function();
        
        // Podemos usar 'self' para acessar outro módulo dentro do módulo 'my':
        self::cool::function();
        
        // A palavra-chave 'super' refere-se ao escopo pai (fora do módulo 'my').
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.

        // Isso será vinculado a 'cool::function' no escopo *crate*.
        // Neste caso, o escopo crate é o escopo mais externo.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

## Hierarquia de arquivos

Módulos podem ser mapeados para uma hierarquia de arquivos/diretórios. Vamos analisar o exemplo usado na seção visibilidade acima em arquivos:

```
$ tree .
.
├── my_mod
│   ├── inaccessible.rs
│   └── nested.rs
├── my_mod.rs
└── main.rs
```

Código em ```main.rs```:

```
mod my_mod;

fn function() {
    println!("'function()' foi chamada.");
}

fn main() {

    function();

    my_mod::function();

    my_mod::indirect_access();

    my_mod::nested::function();
}
```

Código em ```my_mod.rs```:

```
// Da mesma forma, 'mod inaccessible' e 'mod nested' localizarão
// os arquivos 'inaccessible,rs' e 'nested.rs' e os inserirão
// aqui sob seus respectivos módulos.
mod inaccessible;
pub mod nested;

fn private_function() {
    println!("'my_mod::private_function()' foi chamada.");
}

pub fn function() {
    println!("'my_mod::function()' foi chamada.");
}

pub fn indirect_access() {
    print!("'my_mod::indirect_access()' chamando...\n> ");
    private_function();
}
```

Código em ```my_mod/nested.rs```:

```
pub fn function() {
    println!("'my_mod::nested::function()' foi chamada.");
}

#[allow(dead_code)]
fn private_function() {
    println!("'my_mod::nested::private_function()' foi chamada.");
}
```

Código em ```my_mod/inaccessible.rs```:

```
#[allow(dead_code)]
pub fn public_function() {
    println!("'my_mod::inacessible::public_function()' foi chamada.");
}
```

Saída ao executar  ```cargo run```:

```
'function()' foi chamada.
'my_mod::function()' foi chamada.
'my_mod::indirect_access()' chamando...
> 'my_mod::private_function()' foi chamada.
'my_mod::nested::function()' foi chamada.
```

---

[RBE - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)

---

arataca89@gmail.com

Última atualização: 20241104
