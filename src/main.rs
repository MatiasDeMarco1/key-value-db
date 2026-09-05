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
use command::{Command, parse};
use db::execute;
use std::net::TcpStream;
fn main() {
    let db = Arc::new(Mutex::new(load_db_from_log("db.log")));
    let file = Arc::new(Mutex::new(open_log_file("db.log")));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let db_clone = Arc::clone(&db);
        let file_clone = Arc::clone(&file);
        thread::spawn(move || {
            handle_client(stream, db_clone, file_clone);
        });
    }
}
fn handle_client(mut stream: TcpStream, db: Arc<Mutex<HashMap<String, String>>>, file: Arc<Mutex<File>>) {
    let cloned = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut reader = BufReader::new(cloned);
    loop {
        let mut input = String::new();
        match reader.read_line(&mut input) {
            Ok(0) => break, 
            Ok(_) => {},    
            Err(_) => break,
        }
        let input = input.trim();
        let cmd = parse(input);
        let mut db_locked = db.lock().unwrap();
        let mut file_locked = file.lock().unwrap();
        match execute(cmd, &mut db_locked, &mut file_locked) {
            ControlFlow::Continue(msg) => {
                if let Err(_) = writeln!(stream, "{}", msg){
                    break;
                };
            },
            ControlFlow::Break(msg) => {
                if let Err(_) = writeln!(stream, "{}", msg){
                    break;
                };
                break;
            }
        }        
    }
}


fn open_log_file(path: &str) -> File {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap()
}

fn load_db_from_log(path: &str) -> HashMap<String, String> {
    let mut db = HashMap::new();
    if let Ok(read_file) = File::open(path) {
        let reader = BufReader::new(read_file);
        for line in reader.lines() {
            let line = line.unwrap();
            let cmd = parse(&line);
            match cmd {
                Command::Set(key, value) => { db.insert(key, value); },
                Command::Delete(key) => { db.remove(&key); },
                _ => {},
            }
        }
    }
    db
}