use std::ops::ControlFlow;
use std::io::Write;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::net::TcpListener;
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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader =BufReader::new(stream.try_clone().unwrap());
        loop {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let input = input.trim();
            let cmd = parse(input);
            match execute(cmd, &mut db, &mut file) {
                ControlFlow::Continue(msg) => {
                    writeln!(stream, "{}", msg).unwrap();
                },
                ControlFlow::Break(msg) => {
                    writeln!(stream, "{}", msg).unwrap();
                    break;
                }
            }
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
fn execute(cmd: Command, db: &mut HashMap<String, String>,file:  &mut File) -> ControlFlow<String, String>{
    match cmd {
        Command::Set(key ,value) => {db.insert(key.clone(), value.clone()); writeln!(file, "SET {} {}", key, value).unwrap(); ControlFlow::Continue("Se inserto correctamente".to_string())},   
        Command::Get(key) => {let msg = match db.get(&key) {
            Some(value) => format!("Get: {}", value),
            None => "Key not found...".to_string(),
        }; ControlFlow::Continue(msg)},
        Command::Delete(key) => {let msg = match db.remove(&key) {
            Some(value) => {
                writeln!(file, "DELETE {}", key).unwrap();
                format!("Deleted {}", value)
            },
            None => "Key not found".to_string(),
        }; ControlFlow::Continue(msg)},
        Command::Exit =>  ControlFlow::Break("Exit...".to_string()),
        Command::Unknown => ControlFlow::Continue("Err...".to_string())
    }
}