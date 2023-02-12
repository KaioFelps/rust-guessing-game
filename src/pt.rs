use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn jogo_de_advinhar() {
    println!("Advinhe qual o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Chute um número:");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
    
        let guess: u32 = match guess.trim().parse() {
            // parse returns a Result type, it returns Ok or Err, we can use this return to handle the error instead of blocking the application with an error, this is what expect would do.
            // Err is a STRING that contains informations about the error, it won't match the type of number that parse should generate.
            // This is why this match will work.
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número");
                continue
            }
        };
    
        println!("Você chutou o número: {}", &guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito abaixo!\r\n\r\n"),
            Ordering::Greater => println!("Muito acima!\r\n\r\n"),
            Ordering::Equal => {
                println!("Ganhou!\r\n");
                break;
            }
        }
    }
}