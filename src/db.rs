use crate::command::Command;
use std::ops::ControlFlow;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;

pub fn execute(cmd: Command, db: &mut HashMap<String, String>,file:  &mut File) -> ControlFlow<String, String>{
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