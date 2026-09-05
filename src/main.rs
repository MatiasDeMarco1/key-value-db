use std::ops::ControlFlow;
use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut db: HashMap<String, String> = HashMap::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let cmd = parse(input);
        if let ControlFlow::Break(_) = execute(cmd, &mut db){
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
fn execute(cmd: Command, db: &mut HashMap<String, String>) -> ControlFlow<()>{
    match cmd {
        Command::Set(key ,value) => {db.insert(key, value); println!("Se inserto correctamente");ControlFlow::Continue(())},
        Command::Get(key) => {match db.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("Key not found..."),
        }; ControlFlow::Continue(())},
        Command::Delete(key) => {match db.remove(&key) {
            Some(value) => println!("Deleted {}", value),
            None => println!("Key not found"),
        }; ControlFlow::Continue(())},
        Command::Exit => {println!("Exit."); ControlFlow::Break(())},
        Command::Unknown => {println!("Err.."); ControlFlow::Continue(())}
    }
}