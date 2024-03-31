use utils::terminal::{show_menu, wait_return};

mod utils;

fn main() {
    let items = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
    let selected = show_menu("Principal", &items, true);
    wait_return();
}
