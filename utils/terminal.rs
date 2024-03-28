use rpassword::prompt_password;

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn show_items(itens: &[&str]) {
    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn show_menu(title: &str, items: &[&str], exit: bool) -> u32 {
    clear_screen();
    let complete: String = String::from("Rust Menu :: ") + title;
    println!("{}", complete);
    println!("{}", String::from("=").repeat(complete.len()));
    show_items(items);
    print!("");
    return 0;
}

pub fn wait_return() {
    prompt_password("Pressione ENTER para continuar...").unwrap();
}
