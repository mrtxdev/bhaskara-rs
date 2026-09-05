mod bhaskara;

fn main() {
    let squared = bhaskara::Squared::new(1.0, -3.5, 3.0);

    println!("========================================");
    println!("          RESULTADOS DA EQUAÇÃO         ");
    println!("========================================");

    println!("Equação resolvida: {}", squared);

    println!("Raiz 1 (x1)   : {:.4}", squared.x1);
    println!("Raiz 2 (x2)   : {:.4}", squared.x2);
    println!("Vértice (xv)  : {:.4}", squared.xv);
    println!("Vértice (yv)  : {:.4}", squared.yv);

    println!("========================================");
}
