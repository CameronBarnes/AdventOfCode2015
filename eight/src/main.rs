use regex::Regex;

fn handle_quotes(str: &str) -> String {
    let str = &str[1..str.len()-1];
    str.replace("\\\"", "\"")
}

fn handle_escaped_bkslsh(str: &str) -> String {
    str.replace("\\\\", "\\")
}

fn handle_hex(str: &str) -> String {
    let rgx = Regex::new("\\\\x[a-f0-9]{2}").unwrap();
    rgx.replace_all(str, "#").to_string()
        // I dont think it actually matters what this char is, as long as
        // there's only one. So we'll use # as a placeholder
}

fn handle_str_input(str: &str) -> String {
    let str = handle_quotes(str);
    let str = handle_escaped_bkslsh(&str);
    handle_hex(&str)
}

const NEWLINE: u8 = 10;
const QUOTE: u8 = 34;
const SLASH: u8 = 92;

fn main() {
	
    let input = include_str!("./input.txt");
    let total_chars: usize = input.lines().map(|line| line.len()).sum();
    let total_escaped_chars: usize = input.lines().map(handle_str_input).map(|str| str.len()).sum();
    //input.lines().map(handle_str_input).for_each(|line| println!("{line}"));
    let answer = total_chars - total_escaped_chars;
    println!("The total number of original chars minus escaped is is {answer}");

    // For the second part we're going to take a simple iterative aproach as we only need to escape
    // quotes and slashes
    let answer: usize = input.bytes().map(|b| {
        match b {
            NEWLINE => 2,
            QUOTE | SLASH => 1,
            _ => 0,
        }
    }).sum();

    println!("The difference between before and after escaping all the quotes and slashes is {answer}");

}
