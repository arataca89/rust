//--------------------------------------------------------------------
// package: testes
// module : aritmetica.rs
//--------------------------------------------------------------------

pub mod aritmetica{
	pub fn somar(x: f64, y: f64) -> f64 {
		x + y
	}

	pub fn subtrair(x: f64, y: f64) -> f64 {
		x - y
	}
	
	pub fn multiplicar(x: f64, y: f64) -> f64 {
		x * y
	}
	
	pub fn dividir(x: f64, y: f64) -> Result<f64, String> {
		let s = String::from("Tentativa de divisão por zero.");
		if y == 0.0 {
			return Err(s);
		}
		Ok(x / y)		
	}
}
