use std::ops::ControlFlow;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut db: HashMap<String, String> = HashMap::new();
    if let Ok(read_file) = File::open("db.log") {
        let reader = BufReader::new(read_file);
        for line in reader.lines() {
            let line = line.unwrap();
            let cmd = parse(&line);
            match cmd {
                Command::Set(key,value ) => {db.insert(key, value);},
                Command::Delete(key) => {db.remove(&key);},
                _ => {},
            }
        }
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.log")
        .unwrap();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let cmd = parse(input);
        if let ControlFlow::Break(_) = execute(cmd, &mut db, &mut file){
            break;
        }
    }

}
enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    Exit,
    Unknown,
}
fn parse(input: &str) -> Command{
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        [cmd, key, value] if cmd.eq_ignore_ascii_case("SET") => Command::Set(key.to_string(), value.to_string()),
        [cmd, key] if cmd.eq_ignore_ascii_case("GET") => Command::Get(key.to_string()),
        [cmd, key] if cmd.eq_ignore_ascii_case("DELETE") => Command::Delete(key.to_string()),
        [cmd] if cmd.eq_ignore_ascii_case("EXIT") => Command::Exit,
        _ => Command::Unknown
    }
}
fn execute(cmd: Command, db: &mut HashMap<String, String>,file:  &mut File) -> ControlFlow<()>{
    match cmd {
        Command::Set(key ,value) => {db.insert(key.clone(), value.clone()); writeln!(file, "SET {} {}", key, value).unwrap(); println!("Se inserto correctamente");ControlFlow::Continue(())},
        Command::Get(key) => {match db.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("Key not found..."),
        }; ControlFlow::Continue(())},
        Command::Delete(key) => {match db.remove(&key) {
            Some(value) => {println!("Deleted {}", value); writeln!(file, "DELETE {}", key).unwrap();},
            None => println!("Key not found"),
        }; ControlFlow::Continue(())},
        Command::Exit => {println!("Exit."); ControlFlow::Break(())},
        Command::Unknown => {println!("Err.."); ControlFlow::Continue(())}
    }
}