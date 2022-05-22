use rpassword;
use std::env;
use std::process;

use kdf::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: `kdf <iterations>`");
        return;
    }

    let secret = rpassword::prompt_password("Secret: ").unwrap();

    let config = Config::new(secret, args[1].clone()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let seed_phrase = kdf::derive(config);
    match seed_phrase {
        Err(e) => {
            println!("Application error: {}", e);

            process::exit(1);
        }
        Ok(seed_phrase) => {
            println!("BIP39: {}", seed_phrase);
        }
    }
}
