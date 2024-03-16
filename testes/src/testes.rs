//--------------------------------------------------------------------
// package: testes
// module : testes.rs
//--------------------------------------------------------------------

#[cfg(test)]
mod testes{
	use crate::matematica::aritmetica::*;
		
	#[test]
	fn somar_test(){
		assert_eq!(somar(2.0,3.0), 5.0);
	}

	#[test]
	fn subtrair_test(){
		assert_eq!(subtrair(5.0,3.0), 2.0);
	}
	
	#[test]
	fn multiplicar_test(){
		assert_eq!(multiplicar(5.0,3.0), 15.0);
	}	

	#[test]
	fn dividir_test(){
		assert_eq!(dividir(7.0,2.0), Ok(3.5));
		assert_eq!(dividir(7.0,0.0), Err(String::from("Tentativa de divisão por zero.")));
	}	
}	



