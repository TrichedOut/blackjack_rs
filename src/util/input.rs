// single character reading functionality taken from
// https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::io::{self, Read, Write};


/**
 * Reads one character input from stdin, and calls the callback function with
 * the read character.
 * If the callback returns true, it will read another character.
 * If the callback returns false, the function returns
 */
pub fn read_one_char() -> u8 {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();

    tcsetattr(stdin, TCSANOW, & termios).unwrap();
    return buffer[0];
}
