mod en;
mod pt;
use std::io;

fn main() {
    let mut choosen_language = String::new();
    
    loop {
        println!("Select your language | Escolha seu idioma");
        println!("pt-br | en\r\n");
        io::stdin().read_line(&mut choosen_language).expect("You cannot skip this step | Você não pode pular esta etapa");

        match choosen_language.trim() {
            "pt-br" => {
                break pt::jogo_de_advinhar();
            },
            "en" => {
                break en::guess_game();
            },
            _ => println!("Type a valid language | Digite um idioma válido")
        }
    }
}
