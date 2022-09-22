fn main () {
    // ARGUMENTS
    println!("{} chewing gum's", 20); // {} remplace auto par n'importe quel argument.
    println!("{0} est {1}. Il à un grand {2}","julien","juif","nez"); // {n} prends des arguments positionnels;
    println!("{planete}, {pays}, {ville}",planete="Terre", pays="France",ville="Le Mans"); // {name} prends des arguments nommés

    // FORMAT
    println!("base 10: {}", 12345); //{:b,o,x,X}
    println!("bin 2: {:b}", 12345); // formatage
    println!("octal 8: {:o}", 12345); // en 
    println!("hexa 16: {:x}", 12345); // différentes
    println!("hexa 16: {:X}", 12345); // bases

    // STYLE
    println!("{n:>5} ':>n' insère un nombre d'éspaces égal à n avant la réponse", n=7);
    println!("{n:0>5} ':0>n' insère des 0 à la place des espaces ", n=7);
    println!("{n:0>width$} ':0>name$' pareil avec une variable  ", n=7, width=8);

    // EN PRATIQUE
    let number: f64 = 7.7;
    let width: usize = 7;
    println!("Mise en pratique {number:>width$}");

}