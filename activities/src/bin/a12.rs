// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum col {
    Azul,
    Rojo,
    Verde
}

struct caja {
    altura: i32,
    ancho: i32,
    color: col
}

impl caja {
    fn Nueva_Caja () -> Self{
        Self {
            altura: 15,
            ancho: 20,
            color: col::Azul
        }
    }
    fn impr (&self){
        println!("La caja tiene una altura de {:?}", &self.altura);
        println!("La caja tiene una anchura de {:?}", &self.ancho);

        match &self.color {
            col::Azul => println!("La caja es de color azul"),
            _ => println!("La caja es de otro color"),
        };
    }
}

fn main() {
    let capça = caja::Nueva_Caja();
    capça.impr();
}
