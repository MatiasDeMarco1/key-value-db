use std::ops::ControlFlow;
use std::io::Write;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::net::TcpListener;
use std::thread;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
mod command;
mod db;

fn main() {
    use command::{Command, parse};
    use db::execute;
    let db = Arc::new(Mutex::new(HashMap::new()));
    if let Ok(read_file) = File::open("db.log") {
        let reader = BufReader::new(read_file);
        let mut db_guard = db.lock().unwrap();
        for line in reader.lines() {
            let line = line.unwrap();
            let cmd = parse(&line);
            match cmd {
                Command::Set(key,value ) => {db_guard.insert(key, value);},
                Command::Delete(key) => {db_guard.remove(&key);},
                _ => {},
            }
        }
    }
    let file = Arc::new(Mutex::new(OpenOptions::new().create(true).append(true).open("db.log").unwrap()));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let db_clone = Arc::clone(&db);
        let file_clone = Arc::clone(&file);
        let mut stream = stream.unwrap();
        let mut reader =BufReader::new(stream.try_clone().unwrap());
        thread::spawn(move || {
            loop {
                let mut input = String::new();
                reader.read_line(&mut input).unwrap();
                let input = input.trim();
                let cmd = parse(input);
                let mut db_locked = db_clone.lock().unwrap();
                let mut file_locked = file_clone.lock().unwrap();
                match execute(cmd, &mut db_locked, &mut file_locked) {
                    ControlFlow::Continue(msg) => {
                        writeln!(stream, "{}", msg).unwrap();
                    },
                    ControlFlow::Break(msg) => {
                        writeln!(stream, "{}", msg).unwrap();
                        break;
                    }
                }
            }
        });
    }
}


