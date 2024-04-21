// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Blau,
    Vermell,
    Verd,
    Groc
}

fn main() {
    let go = Color::Vermell;
    match go {
        Color::Blau => println!("Blau"),
        Color::Vermell => println!("Vermell"),
        Color::Verd => println!("Verd"),
        Color::Groc => println!("Groc"),
    }
}
