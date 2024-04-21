// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


enum ticket {
    Atras(f64, String),
    Vip(f64),
    Normal(f64, String)
}

fn main() {
    let tick_variant = vec![
        ticket::Atras(50.0,"Josep".to_owned()),
        ticket::Vip(50.0),
        ticket::Normal(50.0,"Pau".to_owned()),
    ];

    for T in tick_variant {
        match T {
            ticket::Vip(n) => println!("El ticket costa {:?}", n),
            ticket::Normal(n,nom) => println!("El ticket costa {:?} i es de {:?}", n, nom),
            ticket::Atras(n,nom) => println!("El ticket costa {:?} i es de {:?}", n, nom),

        };
    };

}
