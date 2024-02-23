// A Base58 and Hexadecimal Encoder From Decimal for small integers
// TODO: finish the decode function for base58 and hex!


use std::io::stdin;
fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Failed to parse");



    println!("The Hex encoding function returns {}", encode_hex(number));
    println!("The Base58 encoding function returns: {} ", encode_base58(number));
}

fn encode_hex(mut number: u32) -> String {
    let mut result = String::new();
    let hexabet: Vec<char> = "0123456789ABCDEF".chars().collect();
    loop {
        let remainder = (number % 16) as usize;
        let digit = hexabet[remainder];
        result.insert(0,digit);

        number /=16;

        if number == 0 {
            break;
        }
    }
    result
}

fn encode_base58(mut number: u32) -> String {
    let mut result = String::new();
    let alphabet: Vec<char> = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".chars().collect();

    loop {
        // Take each remainder and match to alphabet digit
        let remainder = (number % 58) as usize;
        let digit = alphabet[remainder];

        // inserting the matched digit to base 58 in reverse order.
        result.insert(0,digit);

        // remove that digit
        number /=58;

        //exit
        if number == 0 {
            break;
        }
    }
    result
}

fn decode_hex(hex_string: String) -> u32 {
    let mut result = 0;
    todo!();

    result
}

fn decode_base58(base58: String) -> u32 {
    let mut result = 0;
    todo!();

    result
}
