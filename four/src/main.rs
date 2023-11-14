fn main() {
	
    let mut number: u64 = 0;
    let key = "bgvyzdsv";

    loop {
        if format!("{:x}", md5::compute(format!("{}{}", key, number))).starts_with("000000") {
            break;
        }
        number += 1;

        if number % 1000 == 0 {
            println!("Reached {}", number);
        }
    }

    println!("Lowest number that results in a MD5 hash with the desired prefix is {}", number);

}
