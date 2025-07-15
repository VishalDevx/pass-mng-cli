use std::collections::HashMap;
use std::fs;
use std::io::Write;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn load_data() -> HashMap<String, String> {
    let data: String = fs::read_to_string("db.json").unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_data(data: &HashMap<String, String>) -> std::io::Result<()> {
    let save = serde_json::to_string(&data).unwrap();
    fs::write("db.json", save)?; // you had "/db.json", which writes to root!
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut passwords = String::new();
    let mut db: HashMap<String, String>;

    if args.len() == 1 {
        print!("Give password: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut passwords).unwrap();

        db = load_data();
        if db.is_empty() {
            println!("No database found for passwords, consider running passman init first");
            std::process::exit(-1);
        }

        let mcrypt = new_magic_crypt!(passwords.clone(), 256);

        match db.get(&mcrypt.encrypt_str_to_base64("password")) {
            Some(e) => {
                let check = mcrypt.decrypt_base64_to_string(e).unwrap_or_else(|_| "".to_string());
                if check != "test" {
                    println!("Incorrect password, exiting");
                    std::process::exit(-1);
                }
                println!("Welcome!");
            }
            None => {
                println!("Incorrect password, exiting...");
                std::process::exit(-1);
            }
        }
    } else if args.len() == 2 && args[1] == "init" {
        print!("Give password: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut passwords).unwrap();

        let mcrypt = new_magic_crypt!(passwords.clone(), 256);
        let key = mcrypt.encrypt_str_to_base64("password");
        let val = mcrypt.encrypt_str_to_base64("test");

        let mut temp = HashMap::new();
        temp.insert(key, val);

        match save_data(&temp) {
            Err(e) => {
                println!("Unable to create file: {}", e);
                std::process::exit(-1);
            }
            _ => std::process::exit(0),
        }
    } else {
        println!("Incorrect usage.\nSyntax : passman [init]");
        std::process::exit(-1);
    }
}
