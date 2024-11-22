use std::{env, fs, path::Path, process::exit};

use simple_user_input::get_input;

mod simple_user_input {
    use std::io::{self, Write};
    pub fn get_input(prompt: &str) -> String{
        eprint!("{}",prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

fn main() {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.config/shortcuts", home);

    let args = env::args().collect::<Vec<String>>();
    let scf_name: &String = match args.get(1) {
        Some(v) => v,
        None => {
            eprintln!("shortcuts [NAME]");
            eprintln!("    NAME: Should be the name of a file in ~/.config/shortcuts/");
            exit(1);
        },
    };

    let path = Path::new(&path);
    let path = path.join(scf_name);

    if !path.exists() {
        eprintln!("The file does not exist");
    }

    let mut elements = fs::read_to_string(path)
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if elements.last().unwrap() == "" {
        elements.pop();
    }
    
    for (idx, e) in elements.iter().enumerate() {
       eprintln!("{}) {}", idx+1, e);
    }
    eprintln!();
    let value = get_input(": ");

    match usize::from_str_radix(&value, 10) {
       Ok(v) => {
           let idx = v-1;
           
           if idx >= elements.len() {
               eprintln!("Not in range.");
               return
           }
           println!("{}", elements[idx])
       },
       Err(_) => {
           eprintln!("Not a valid number")
       },
    };
}
