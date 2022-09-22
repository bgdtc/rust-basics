// PAS DE MISE EN FORME

// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//     println!("il y à {:?} mois dans une année", 12);
//     println!("{1:?} {0:?} is the {actor:?} name","bon","jean",actor="actors's");

//     println!("Maintenant {:?} va s'afficher", Structure(3));
//     println!("et la {:?} va s'afficher", Deep(Structure(7)));
// }

// MISE EN FORME

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Jean";
    let age = 25;
    let jean = Person { name, age };
    println!("{:#?}", jean);
}