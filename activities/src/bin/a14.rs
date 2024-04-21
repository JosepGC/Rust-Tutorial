// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Persona {
    nom: String,
    edad: i32,
    color: String
}

fn imprimir(nom: &str, color: &str){
    println!("El nom es: {:?}, El color es: {:?}",nom, color);
}

fn main() {
    let Personas = vec![
        Persona{
            nom: "Josep".to_owned(),
            edad: 23,
            color: "Blau".to_owned(),
        },
        Persona{
            nom: "Gala".to_owned(),
            edad: 23,
            color: "Tronja".to_owned(),
        },
        Persona{
            nom: "Pau".to_owned(),
            edad: 22,
            color: "Vermell".to_owned(),
        }
    ];

    for Pers in Personas {
        if Pers.edad < 25 {
            imprimir(&Pers.nom, &Pers.color);
        }
        else {

        }
    }

}
