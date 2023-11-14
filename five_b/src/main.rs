use regex::Regex;

fn main() {
	let nice_words = include_str!("./input.txt").lines().filter(|item| is_nice_str(item)).count();
    println!("{} words on the list are 'nice'", nice_words);
}

fn is_nice_str(item: &str) -> bool {
    check_separated_repeat(item) && check_non_overlapping_letter_pair(item)
}

fn check_non_overlapping_letter_pair(item: &str) -> bool {

    for i in 0..item.len() {
        if i == item.len() - 2 {
            return false;
        }

        let pair = &item[i..=i+1];

        let mut index = i + 2;
        while index < item.len() - 1 {
            if pair.eq(&item[index..=index+1]) {
                return true;
            }
            index += 1;
        }

    }

    false

}

fn check_separated_repeat(item: &str) -> bool {
    
    for c in 'a'..='z' {
        let regex = Regex::new(&format!("{c}[^{c}]{c}")).unwrap();
        if regex.is_match(item) {
            return true;
        }
    }

    false

}
