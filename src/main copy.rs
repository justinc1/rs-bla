/*
extern crate termion;

use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use std::io::stdout;
// use std::time::{Duration, Instant};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};

fn main() {
    println!("Hello, world 1!");
    thread::sleep(Duration::from_millis(100));
    println!("Hello, world 2!");

    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}x", termion::cursor::Goto(3,7)).unwrap();
}
*/

/*

o w
|
x
*/

extern crate termion;
extern crate termios;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::thread;
use std::time::Duration;

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
// use std::sync::mpsc::TryRecvError;
use std::time;

use std::io::Read;
// use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};



// use std::io;
// use std::io::Write;
// use std::thread;
// use std::time;

// use termion;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

// read_ch
fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if 'q' is pressed
                termion::event::Key::Char('q') => break,
                // Else print the pressed key
                _ => {
                    write!(
                        stdout,
                        "{}{}Key pressed: {:?}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        key
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}


fn main3() {
    // let stdin_channel = spawn_stdin_channel();
    // loop {
    //     match stdin_channel.try_recv() {
    //         Ok(key) => println!("Received: {}", key),
    //         Err(TryRecvError::Empty) => println!("Channel empty"),
    //         Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    //     }
    //     sleep(1000);
    // }

    let ts: u64 = 1000;
    // let stdin = stdin();
    let stdin = stdin();
    let stdin_fd = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin_fd, TCSANOW, &mut new_termios).unwrap();
    //let stdout = io::stdout();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    print!("Hit a key! ");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    println!("You have hit: {:?}", buffer);
    tcsetattr(stdin_fd, TCSANOW, & termios).unwrap(); 
    return;



    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    write!(stdout, "{}x", termion::cursor::Goto(3, 7)).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_millis(ts));
    write!(stdout, "{}y", termion::cursor::Goto(3, 5)).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_millis(ts));

    loop {
        // write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
        let ch: String = "?".to_string();
        write!(stdout, "{}", ch).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(ts));
    }


    return;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
