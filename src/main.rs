use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::process;
use std::{thread, time};
use morse::encode;

const MULTIPLIER: u64 = 150;

const DOT_LENGTH: time::Duration = time::Duration::from_millis(1 * MULTIPLIER);
const DASH_LENGTH: time::Duration = time::Duration::from_millis(3 * MULTIPLIER);
const INNER_ELE_GAP: time::Duration = time::Duration::from_millis(1 * MULTIPLIER);
const LETTER_GAP: time::Duration = time::Duration::from_millis(3 * MULTIPLIER);
const WORD_GAP: time::Duration = time::Duration::from_millis(7 * MULTIPLIER);
const LOOP_GAP: time::Duration = time::Duration::from_millis(15 * MULTIPLIER);

fn led(state: bool) -> io::Result<()>{
    let mut f =  OpenOptions::new().write(true).open("/sys/kernel/debug/ec/ec0/io")?;
    f.seek(SeekFrom::Start(12))?;
    if state {
        f.write(&[0x8a])?;
    } else {
        f.write(&[0x0a])?;
    }
    f.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let morse_str;

    match args.len() {
        1 => {
            println!("No string argument provided. Exiting.");
            process::exit(1);
        },
        2 => {
            match encode::encode(&args[1]){
                Ok(morse)   => morse_str = morse,
                Err(e)      => {
                    println!("Failed to convert string to morse equivalent. Exiting. {:?}", e);
                    process::exit(1);
                }
            };
        },
        _ => {
            println!("Too many arguments. Exiting.");
            process::exit(1);
        }
    }
    println!("String in morse: {}", morse_str);
    let char_vec: Vec<char> = morse_str.chars().collect();

    loop {
        for (i, c) in char_vec.iter().enumerate() {
            led(false).unwrap();

            if c == &'.' {
                led(true).unwrap();
                thread::sleep(DOT_LENGTH);
                led(false).unwrap();
                
                if char_vec.len() != i+1 {
                    if char_vec[i+1] != ' ' && char_vec[i+1] != '/'{
                        thread::sleep(INNER_ELE_GAP);
                    }
                }
                
            }
            else if c == &'_' {
                led(true).unwrap();
                thread::sleep(DASH_LENGTH);
                led(false).unwrap();

                if char_vec.len() != i+1 {
                    if char_vec[i+1] != ' ' && char_vec[i+1] != '/'{
                        thread::sleep(INNER_ELE_GAP);
                    }
                }
            }
            else if c == &' ' {
                if char_vec.len() != i+1 {
                    if char_vec[i+1] != ' ' && char_vec[i-1] != '/'{
                        thread::sleep(LETTER_GAP);
                    }
                }
            }
            else if c == &'/' {
                thread::sleep(WORD_GAP);
            }
        }
        thread::sleep(LOOP_GAP);
    }

}
