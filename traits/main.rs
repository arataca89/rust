
use trait2::Retangulo;
use trait2::Triangulo;
use trait2::Circulo;
use trait2::FiguraPlana;


// outra maneira de trazer os tipos e a trait para este crate
// use trait2::{Retangulo, Triangulo, Circulo, FiguraPlana};

fn main() {
    let r1 = Retangulo::new(5.0, 3.0);
    r1.relatorio();

    println!("");

    let t1 = Triangulo::new(14.0, 9.0, 7.0);
    t1.relatorio();

    println!("");

    let c1 = Circulo::new(5.0);
    c1.relatorio();

    println!("");

    trait2::relatorio2(&r1);
    trait2::relatorio2(&t1);
    trait2::relatorio2(&c1);

    println!("");

    trait2::relatorio3(&r1);
    trait2::relatorio3(&t1);
    trait2::relatorio3(&c1);

    println!("");
    
    let mut figuras: Vec<&dyn FiguraPlana> = Vec::new();


    figuras.push(&r1);
    figuras.push(&t1);
    figuras.push(&c1);


    println!("Total das areas somadas: {}", trait2::totalizar_area(figuras));

    println!("");
     
    let r2 = trait2::retorna_figura();
    r2.relatorio();

    println!("");
     
    let f1 = trait2::retorna_figura2(true);
    f1.relatorio();
    println!("");
    let f2 = trait2::retorna_figura2(false);
    f2.relatorio();
    
}
