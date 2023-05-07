use std::{time::Duration};
use std::thread::{sleep, self};
use std::io;
use inputbot::{KeybdKey::*, MouseButton::*};
use console::Term;

fn define_key(ms: u64) {
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();
            sleep(Duration::from_millis(ms));
        }
    });
}

// read the time from console
fn read_time() -> u64 {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");
    let trimmed = buffer.trim();

    let mut time = 0;
    match trimmed.parse::<u64>() {
        Ok(i) => time = i,
        Err(..) => println!("you did not enter a number"),
    };
    time
}

fn read_string() -> String {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");
    let trimmed = buffer.trim();
    trimmed.to_string()
}

fn main() {
    loop {
        println!("--Autoclicker--");
        println!("Enter delay between clicks (ms): ");
    
        let time: u64 = read_time();
        let term: Term = Term::stdout();

        // binds the time to 'Caps'
        define_key(time);
    
        println!("Press [CapsLock] to toggle the autoclicker.");
    
        // starts listening to inputs
        thread::spawn(|| {
            inputbot::handle_input_events();
        });
        println!("enter [back] to go back or anything else to quit");
        if read_string() == "back" {
            term.clear_screen().expect("couldn't clear the console");
        }
        else {
            break;
        }
    } 
}