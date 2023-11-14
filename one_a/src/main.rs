fn main() {
	let input = include_str!("../input.txt");

    let mut floor = 0;
    // input.chars().for_each(|c| if c == '(' {floor += 1} else {floor -= 1}); Solution first part

    let mut index = 0;
    let mut check_basement = true;
    input.chars().for_each(|c| {
        index += 1;
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if check_basement && floor < 0 {
            check_basement = false;
            println!("First basement instruction is at index {}", index);
        }
    });
    println!("Final floor from directions is {}", floor);
}
