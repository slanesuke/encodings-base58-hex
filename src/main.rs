// A Base58 and Hexadecimal Encoder From Decimal for small integers
// I also added a base58 decoder to test
use std::io::stdin;

fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Failed to parse");

    let hex = encode_hex(number);
    let base58 = encode_base58(number);

    println!("The Hex encoding function returns {}", hex);
    println!("The Base58 encoding function returns: {} ", base58);

    let decode_base58 = decode_base58(base58.as_str());


    println!("\n\nNow I'm testing decode Base58 fn.");
    match decode_base58 {
        Some(decode) => println!("Decoded Base58 value {}", decode),
        None => println!("Invalid Base58 String"),
    }


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


fn decode_base58(base58: &str) -> Option<u64> {
    const BASE68_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let mut result: u64 = 0;

    for char in base58.chars() {
        // Find the index of the character in the Base58 alphabet!
        // if None is the base58 character is invalid and there will be an error.
        let value = BASE68_ALPHABET.find(char)?;
        result = result * 58 + value as u64;
    }

    Some(result)
}
