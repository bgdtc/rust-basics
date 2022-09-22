use std::fmt; // import du module

struct List(Vec<i32>); // structure vecteur

impl fmt::Display for List {
    // implémentation affichage
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?; // début tableau
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                // boucle for sur les valeurs
                write!(f, ",")?; // du vecteur, "," après chaques entrées si c'est pas la premiere
            }
            write!(f, "{1}: {0}", v, count)?; // insertion valeur
        }

        write!(f, "]") // fin du tableau
    }
}

// call de la structure dans la function main
fn main() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}
