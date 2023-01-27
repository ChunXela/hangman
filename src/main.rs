use std::io;

fn main() {

    loop {

        let mut choice = String::new();
        println!("Welcome to handman in rust");
        println!("1. Start game");
        println!("2. Settings");
        println!("3. Exit game");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

        let choice: u16 = match choice
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        match choice{
            1 => println!("Game has started"),
            // 2 => settings(),
            3 => break,
            _=>println!("Please enter a number between 1 => 3!"),
        };
        break;
    }
}

fn game(){

    let mut topic: Vec<String> = vec![String::new(), 126]; 

    topic ["Anime Characters", "Cartoon Characters", "NTI Teachers", "Custom"];
}
