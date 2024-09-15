// Define a trait 'FiguraPlana'
pub trait FiguraPlana{
    fn print(&self);
    fn perimetro(&self) -> f64;
    fn area(&self) -> f64;

    // Implementação padrão de um método da trait
    fn relatorio(&self){
        self.print();
        println!("Perimetro: {}", self.perimetro());
        println!("Area     : {}", self.area());
    }
}

pub struct Retangulo{
    base: f64,
    altura: f64,
}

// Métodos específicos do tipo 'Retangulo'
impl Retangulo {
    pub fn new(base: f64, altura: f64) -> Retangulo{
        Retangulo{base, altura}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Retangulo'
impl FiguraPlana for Retangulo{
    fn print(&self){
        println!("Retangulo: {{Base:{}, Altura:{}}}",
        self.base, self.altura);
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.base + self.altura)
    }

    fn area(&self) -> f64 {
        self.base * self.altura
    }
}

pub struct Triangulo{
    lado_a: f64,
    lado_b: f64,
    lado_c: f64,
}

// Métodos específicos do tipo 'Triangulo'
impl Triangulo {
    pub fn new(lado_a: f64, lado_b: f64, lado_c: f64) -> Triangulo{
        Triangulo{lado_a, lado_b, lado_c}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Triangulo'
impl FiguraPlana for Triangulo{
    fn print(&self){
        println!("Triangulo: {{Lado A:{}, Lado B:{}, Lado C:{}}}",
        self.lado_a,self.lado_b, self.lado_c);
    }

    fn perimetro(&self) -> f64 {
        self.lado_a + self.lado_b + self.lado_c
    }

    // Calcula a area de um triangulo segundo a formula de Heron
    // https://mundoeducacao.uol.com.br/matematica/formula-heron.htm
    fn area(&self) -> f64 {
        let p = (self.lado_a + self.lado_b + self.lado_c) / 2.0;
        let q =
        p * (p-self.lado_a) * (p-self.lado_b) * (p-self.lado_c);
        q.sqrt()
    }
}

pub struct Circulo{
    raio: f64,
}

// Métodos específicos do tipo 'Circulo'
impl Circulo {
    pub fn new(raio: f64) -> Circulo{
        Circulo{raio}
    }    
}

// Implementa a trait 'FiguraPlana' para o tipo 'Circulo'
impl FiguraPlana for Circulo{
    fn print(&self){
        println!("Circulo: {{Raio:{}}}", self.raio);
    }

    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.raio
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }

    // Substitui a implementação padrão definida na criação da trait
    fn relatorio(&self){
        println!("Circulo com raio {}", self.raio);
        println!("Perimetro: {}", self.perimetro());
        println!("Area     : {}", self.area());
    }
}

// Função que recebe um objeto que implementa a trait
// 'FiguraPlana'
pub fn relatorio2(figura: &impl FiguraPlana){
    println!("Perimetro: {}; Area: {}", figura.perimetro(),
    figura.area());
}

// Função que recebe um objeto que implementa a trait
// 'FiguraPlana'
pub fn relatorio3<T: FiguraPlana>(figura: &T){
    println!("Area: {}; Perimetro: {}", figura.area(),
    figura.perimetro());
}


// Função que recebe um 'Vec' de objetos que implementam a trait
// 'FiguraPlana' como parâmetro
//pub fn totalizar_area(figuras: Vec<&dyn FiguraPlana>) -> f64 {
pub fn totalizar_area(figuras: Vec<&dyn FiguraPlana>) -> f64 {
    let mut area_total = 0.0;
    for f in figuras{
        area_total += f.area();
    }
    area_total
}


// Função que retorna um tipo que implementa a trait 'FiguraPlana
pub fn retorna_figura() -> impl FiguraPlana {
    Retangulo{base: 2.5, altura: 3.5}
}


// Função que retorna tipos que implementam a trait 'FiguraPlana
// ERRO: a função abaixo não compila
// pub fn retorna_figura2(b: bool) -> impl FiguraPlana {
//     if b {
//         Retangulo{base: 2.5, altura: 3.5}
//     } else {
//         Circulo{raio: 4.5}
//     }
// }

// Função que retorna tipos que implementam a trait 'FiguraPlana
pub fn retorna_figura2(b: bool) -> Box<dyn FiguraPlana> {
    if b {
        Box::new(Retangulo{base: 2.5, altura: 3.5})
    } else {
        Box::new(Circulo{raio: 4.5})
    }
}


