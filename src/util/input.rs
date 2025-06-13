use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::{any::Any, fmt::Debug, io::{self, Read, Write}, os::fd::AsRawFd, str::FromStr};


/**
 * Reads one character input from stdin, and calls the callback function with
 * the read character.
 * If the callback returns true, it will read another character.
 * If the callback returns false, the function returns.
 * Taken from https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
 */
pub fn read_one_char() -> char {
    // get stdin file descriptor
    let stdin = io::stdin().as_raw_fd();

    // get current terminal 'settings' to restore to later
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    
    // set the flags to *just* canonical and echo mode
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    // get stdout and stdin
    let stdout = io::stdout();
    let mut reader = io::stdin();

    // buffer to read 1 byte
    let mut buffer = [0;1];

    // flush stdout and read exactly 1 character from stdin
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();

    // put the settings back, return the read character
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
    buffer[0] as char
}

/**
 * Get an input, validated under multiple criteria.
 * -> character_validator: valid character conditional.
 * -> input_validator: valid input conditional.
 * Highlights valid inputs green and invalid outputs red.
 * Returns the input once valid and enter is pressed
 */
pub fn validated_input<ValidChar, ValidInput, Ret: FromStr + 'static>
        (valid_character: ValidChar, valid_input: ValidInput) -> Ret 
        where <Ret as FromStr>::Err:Debug,
        ValidChar: Fn(char) -> bool,
        ValidInput: Fn(Ret) -> bool {

    /**
     * Parses an input, then validates it.
     * If the input cannot be parsed, returns false.
     * If the input can be parsed, returns the result of the validator on the 
     * input
     */
    fn validate_parse_or_false<V, T: Any + FromStr>(validate: &V, input: &String) -> bool where V: Fn(T) -> bool {
        match input.parse() {
            Ok(inp) => validate(inp),
            Err(_)    => false,
        }
    }

    // start with empty input
    let mut input = String::new();
    
    // keep taking input until input is valid
    loop {
        // keep taking input until backspace is pressed
        loop {
            // read one character, and check what was recieved
            match read_one_char() as char {
                // if a valid character
                c if valid_character(c) => {
                    // add it to the input.
                    input.push(c);

                    // move left enough times to properly print the input later
                    let len = input.len();
                    if len > 1 {
                        print!("[{}D", len-1);
                    }
                } 
                // if the backspace character
                '\x7f' => {
                    // remove last char from the input
                    let p = input.pop();

                    // move left enough times to properly print the input later
                    let len = input.len();
                    if p.is_some() {
                        print!("[{}D", len+1);
                    }
                },
                // if pressed enter, check the input
                '\n' => break,
                // ignore all other input
                _ => continue,
            }

            // color according to validity
            if validate_parse_or_false(&valid_input, &input) {
                print!("[38;5;40m{input}[K[0m");
            } else {
                print!("[38;5;196m{input}[K[0m");
            }
        }

        if validate_parse_or_false(&valid_input, &input) {
            break;
        }
    }

    // return the parsed input
    input.parse().unwrap()
}
