use crate::command::Command;
use std::ops::ControlFlow;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;

pub fn execute(cmd: Command, db: &mut HashMap<String, String>,file:  &mut File) -> ControlFlow<String, String>{
    match cmd {
        Command::Set(key ,value) => {db.insert(key.clone(), value.clone());let msg = match writeln!(file, "SET {} {}", key, value)  {
            Ok(_) => "Se inserto correctamente".to_string(),
            Err(_) =>  "Se inserto, pero fallo al guardad el log".to_string(),
            
        }; ControlFlow::Continue(msg)},   
        Command::Get(key) => {let msg = match db.get(&key) {
            Some(value) => format!("Get: {}", value),
            None => "Key not found...".to_string(),
        }; ControlFlow::Continue(msg)},
        Command::Delete(key) => {let msg = match db.remove(&key) {
            Some(value) => {
                let msg = match writeln!(file, "DELETE {}", key) {
                    Ok(_) => format!("Deleted {}", value),
                    Err(_) => format!("Deleted {}, pero no se guardo en el Log", value),
                };
                msg
            },
            None => "Key not found".to_string(),
        }; ControlFlow::Continue(msg)},
        Command::Exit =>  ControlFlow::Break("Exit...".to_string()),
        Command::Unknown => ControlFlow::Continue("Err...".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_execute_set() {
        let mut db = HashMap::new();
        let file = NamedTempFile::new().unwrap();
        let mut file_handle = file.reopen().unwrap();

        let result = execute(Command::Set("nombre".to_string(), "juan".to_string()), &mut db, &mut file_handle);

        assert_eq!(db.get("nombre"), Some(&"juan".to_string()));
        assert!(matches!(result, ControlFlow::Continue(_)));
    }
    #[test]
    fn none_key(){
        let mut db = HashMap::new();
        let file = NamedTempFile::new().unwrap();
        let mut file_handle = file.reopen().unwrap();
        let result = execute(Command::Get("animal".to_string()), &mut db, &mut file_handle);
        if let ControlFlow::Continue(msg) = result {
            assert_eq!(msg, "Key not found...");
        }else {
            panic!("esperaba ControlFlow::Continue")
        }
    }
    #[test]
    fn test_delete_existing_key() {
        let mut db = HashMap::new();
        db.insert("animal".to_string(), "perro".to_string());
        let file = NamedTempFile::new().unwrap();
        let mut file_handle = file.reopen().unwrap();

        let result = execute(Command::Delete("animal".to_string()), &mut db, &mut file_handle);

        assert_eq!(db.get("animal"), None);
        if let ControlFlow::Continue(msg) = result {
            assert_eq!(msg, "Deleted perro");
        } else {
            panic!("esperaba ControlFlow::Continue");
        }
    }
    #[test]
    fn test_exit_breaks() {
        let mut db = HashMap::new();
        let file = NamedTempFile::new().unwrap();
        let mut file_handle = file.reopen().unwrap();

        let result = execute(Command::Exit, &mut db, &mut file_handle);

        assert!(matches!(result, ControlFlow::Break(_)));
    }
}