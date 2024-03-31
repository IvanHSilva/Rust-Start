use std::process::exit;

use utils::terminal::{clear_screen, show_menu};

mod essentials;
mod utils;

fn main() {
    loop {
        let items = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
        let selected = show_menu("Principal", &items, true);

        clear_screen();
        match selected {
            1 => essentials::execute(),
            2 => println!("Tipos"),
            3 => println!("Controle"),
            4 => println!("Funções"),
            5 => println!("Ownership"),
            _ => exit(0),
        }
    }
}
