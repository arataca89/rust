# basic_calculator

 Este pacote implementa implementa uma calculadora aritmética simples. 
 
## Módulo: pilha_generica

Este módulo implementa a funcionalidade de criar pilhas dos tipos
primitivos: inteiros, de ponto flutuante e char.
 
Métodos implementados:

new() -> Pilha<T>{
	Cria uma pilha vazia, do tipo genérico T.

is_empty(&self) -> bool
	Retorna true se a pilha estiver vazia (sem nenhum item), senão
	retorna false.


push(&mut self, valor: T)
	Insere um valor do tipo genérico T na pilha e
	incrementa o tamanho (size) em uma unidade.

pop(&mut self) -> T 
	Retira e retorna o item no topo da pilha.
	
top(&mut self) -> T 
	Retorna o item no topo da pilha, sem retirar.
	
size(&self) -> usize 
	Retorna a quantidade de itens da pilha.

print(&self)
	Imprime a pilha na tela no formato: colchete, seguido dos itens
	da pilha separados por vírgula, seguido por colchete. Exemplos:
		['3', '+', '5']
		[1970, 1985, 2015]
		[3.14159, 2.65, 1.2345]


O tipo Pilha implementa Debug e Copy.
A trait Debug permite usar println! com o tipo generico T.
Se não implementarmos Debug obteremos o seguinte erro:

println!("{:?}", self.itens);
                 ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`

 
A trait Copy permite que a função top() retorne uma cópia de
um objeto generico do tipo T. Se não implementarmos Copy obteremos
o seguinte erro:

self.itens[self.size-1]        
^^^^^^^^^^^^^^^^^^^^^^^ move occurs because value has type `T`, which does not implement the `Copy` trait


Módulo tools
============
Este módulo implementa funções auxiliares.

Funções implementadas:

check_paren(expr: &String) -> bool
	Verifica os parênteses da expressão. Cada parêntese esquerdo deve
	ter um parêntese direito correspondente. Se todos os parênteses
	conferem retorna true; senão retorna false.
	
is_operator(c: char) -> bool
	Verifica se o caractere passado como argumento é um operador
	aritmético ('+', '-', '*' ou '/'). Se o caractere for um
	operador aritmético retorna true; senao retorna false.

prioridade(c: char) -> i8
	Recebe um caractere que é um operador e retorna sua prioridade
	conforme tabela abaixo.
	
	operador	prioridade
	   '+'		   1
	   '-'		   1
	   '*'		   2
	   '/'		   2


Módulo pfix
============
Este módulo possui funções que tratam expressões posfixas.

Funções implementadas:

to_pfix(expr: &String) -> String 
	Converte uma expressão infixa em posfixa.
	
pfix_value(tokens: Vec<Token>) -> f64 
	Recebe um Vec de Tokens referente aos tokens de uma expressão
	aritmética posfixa e retorna o resultado da expressão.
	
	

Módulo lexer
============
Este módulo implementa tipos e funções para fornecer a funcionalidade
de lexer a expressão aritmética sendo avalida. Bsicamente o tipo 
Token e a função get_token().

O tipo Token é um enum com as variantes:
	Operador(char); e
	Numero(f64)
	
A função get_token() recebe a expressão aritmética na forma
posfixa e retorna um Vec de Tokens.

get_token(expr: String) -> Vec<Token> 	


ALGORITMO PARA CONVERTER EXPRESSAO INFIXA EM POSFIXA
====================================================

 * Criar uma pilha de caracteres.
 * Percorrer a expressao infixa, da esquerda para a direita,
 * analisando cada caractere:
 		* se for um parentese de abertura, insira na pilha;
  		* se for um operando, anexar a expressao posfixa;
  		* se for um operador:
  				* enquanto houver no topo da pilha operador com 
  				  prioridade maior ou igual, retire esse operador
  				  da pilha e anexe a expressao posfixa;
  				* insira o operador recem encontrado na pilha
  		* se for um parentese de fechamento, remover um operador da
  		  pilha e anexar a expressao posfixa, ate que apareca um
  		  parentese de abertura. No final retire esse parentese e
  		  descarte-o.
  		* Depois de percorrer a expressao infixa, retirar todos os
  		  itens restantes da pilha e anexa-los a expressao posfixa.
  
  
ALGORITMO PARA AVALIAR EXPRESSAO POSFIXA
========================================

 * Criar uma pilha do tipo de dados numerico.
 * Percorrer a expressao posfixa, da esquerda para a direita,
 * analisando cada token:
  		* se for um operando, insira na pilha seu valor numerico;
  		* se for um operador, desempilhe dois itens da pilha, aplique
  		  o operador a estes valores e empilhe o resultado;
  		* no final, devolva como resultado o valor existente no topo
  		  da pilha.


=============
FIM DO README
=============


