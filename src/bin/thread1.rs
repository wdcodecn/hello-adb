use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file = File::open("1.md").unwrap();
    let reader = BufReader::new(file);

    let shared_reader = Arc::new(Mutex::new(reader));

    let mut handles = vec![];
    for i in 0..4 {
        let shared_reader = shared_reader.clone();
        let handle = thread::spawn(move || {
            let mut reader = shared_reader.lock().unwrap();
            let mut line = String::new();
            reader.read_to_string(&mut line).unwrap();
            println!("Thread {} read: {}", i, line.trim());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
