use crate::utils::terminal::{clear_screen, show_menu, wait_return};

pub fn execute() {
    loop {
        let itens = [
            "Primeiro Exemplo",
            "Variáveis Imutáveis",
            "Variáveis Mutáveis",
            "Variáveis Constantes",
            "Variáveis Shadowing",
            "Operadores Aritméticos",
            "Operadores Relacionais",
            "Operadores Lógicos",
        ];

        let selected = show_menu("Fundamentos", &itens, false);
        clear_screen();
        match selected {
            1 => println!("Fundamentos 1"),
            _ => break,
        }
        wait_return();
    }
}
