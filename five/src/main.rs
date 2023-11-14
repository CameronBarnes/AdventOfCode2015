fn check_vowels(str: &str) -> bool {

    let mut count = 0;
    count += str.match_indices('a').count();
    count += str.match_indices('e').count();
    count += str.match_indices('i').count();
    count += str.match_indices('o').count();
    count += str.match_indices('u').count();

    count >= 3

}

fn check_double_letters(str: &str) -> bool {

    let mut iter = str.chars();
    let mut last = iter.next().expect("String was empty");

    for next in iter {

        if last == next {
            return true;
        } else {
            last = next;
        }

    }

    false

}

fn check_banned_strs(str: &str) -> bool {

    let banned_strs = vec!("ab", "cd", "pq", "xy");
    for banned in banned_strs {
        if str.contains(banned) {
            return false;
        }
    }

    true

}

fn is_nice_str(str: &str) -> bool {

    check_double_letters(str) && check_vowels(str) && check_banned_strs(str)

}

fn main() {

    let nice_words = include_str!("./input.txt").lines().filter(|item| is_nice_str(item)).count();
    println!("{} words on the list are 'nice'", nice_words);
	
}
