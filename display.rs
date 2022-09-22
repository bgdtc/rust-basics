use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64); // Structure de 2 nombres

impl fmt::Display for MinMax {
    // Implémentation de l'affichage
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1) // self.0 et 1 fait référence data de la structure
    }
}

#[derive(Debug)]
struct Point2D {
    // Structure de deux valeurs nommables
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    // Implémentation de l'affichage
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    // main function
    let minmax = MinMax(0, 14);

    println!("Comparaison des deux structures: ");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "Grand écart avec debug: {big:?} et petit écart: {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 7.7, y: 12.12 };

    println!("Point2D en Display: {} ", point);
    println!("Point2D en Debug: {:?} ", point);
}
