use std::collections::HashMap;

fn main() {
	
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut santa_pos = (0, 0);
    let mut santa_two_pos = (0, 0);

    map.insert(santa_pos, 2);

    let mut santa = true;
    let input = include_str!("./input.txt");
    input.chars().for_each(|c| {

        let current_pos: &mut (i32, i32) = if santa {
            &mut santa_pos
        } else {
            &mut santa_two_pos
        };

        match c {
            '>' => current_pos.0 += 1,
            '<' => current_pos.0 -= 1,
            '^' => current_pos.1 += 1,
            'v' => current_pos.1 -= 1,
            _ => panic!("Invalid Input"),
        }

        if let std::collections::hash_map::Entry::Vacant(e) = map.entry(*current_pos) {
            e.insert(1);
        } else {
            *map.get_mut(current_pos).unwrap() += 1;
        }

        santa = !santa;

    });

    println!("{} houses got at least one present", map.keys().len());

}
