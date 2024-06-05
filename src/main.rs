use rand::Rng;
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SAVE_FILE: &str = "counter.txt";

fn main() {
    let start_number = read_last_number(SAVE_FILE).unwrap_or(0);

    println!("Number of threads:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let num_threads: usize = input.trim().parse().expect("Error parsing number");

    let mut rng = rand::thread_rng();
    let max_number = rng.gen_range(start_number + 10..start_number + 50);

    println!("Counting up to {}", max_number);
    let counter = Arc::new(Mutex::new(start_number));
    let mut handles = vec![];

    for i in 0..num_threads {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let sleep_duration = rng.gen_range(100..1000);
                thread::sleep(Duration::from_millis(sleep_duration));

                let mut num = counter.lock().unwrap();
                if *num >= max_number {
                    break;
                }
                *num += 1;
                println!("Thread {} incremented the counter to: {}", i, *num);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_number = *counter.lock().unwrap();
    write_last_number(SAVE_FILE, final_number).expect("Error writing to file");

    println!("Finished counting up to {}", final_number);
}

fn read_last_number(filename: &str) -> io::Result<u32> {
    if let Ok(contents) = fs::read_to_string(filename) {
        if let Ok(number) = contents.trim().parse() {
            return Ok(number);
        }
    }
    Ok(0)
}

fn write_last_number(filename: &str, number: u32) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    writeln!(file, "{}", number)?;
    Ok(())
}
