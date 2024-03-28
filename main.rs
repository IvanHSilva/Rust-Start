use utils::terminal::show_menu;

mod utils;


fn main() {
    let items = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
    show_menu("Principal", &items, true);
}
