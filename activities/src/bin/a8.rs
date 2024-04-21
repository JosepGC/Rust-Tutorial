// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Sabors{
    Fresa,
    Vallnilla,
    Chocolate
}

struct Begudes {
    Tipus: Sabors,
    Onza: i32,
}

fn main() {
    let Sab = Sabors::Fresa;
    let Beg = Begudes {
        Tipus: Sab,
        Onza: 12,
    };
    Escollir(Beg);
    let Ber = Begudes {
        Tipus: Sabors::Vallnilla,
        Onza: 9,
    };
    Escollir(Ber);
}

fn Escollir(X: Begudes){
    match X.Tipus {
        Sabors::Fresa => println!("El sabor es Fresa i la Onza: {:?}",X.Onza),
        Sabors::Vallnilla => println!("El sabor es Vallnilla i la Onza: {:?}",X.Onza),
        Sabors::Chocolate => println!("El sabor es Chocolate i la Onza: {:?}",X.Onza),
    };
    
}