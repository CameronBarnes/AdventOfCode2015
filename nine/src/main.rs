use std::{collections::HashMap, sync::atomic::AtomicUsize};

const MAX_SIZE: usize = 8;

struct Location {
    index: usize,
    list: [usize; MAX_SIZE],
}

#[derive(Clone, Copy)]
enum Target {
    HIGH,
    LOW,
}

impl Location {
    fn new(index: usize) -> Self {
        let list: [usize; MAX_SIZE] = [0; MAX_SIZE];
        Location { index, list }
    }

    fn put_for_index(&mut self, index: usize, distance: usize) {
        if index == self.index {
            panic!("Location Given its own index");
        } else {
            *self
                .list
                .get_mut(index)
                .unwrap_or_else(|| panic!("Invalid index {index}")) = distance;
        }
    }

    fn find(
        &self,
        mut unused: [bool; MAX_SIZE],
        list: &[Location],
        ord: Target,
    ) -> (Vec<usize>, usize) {
        unused[self.index] = false;
        let remaining = unused.iter().filter(|i| **i).count();

        let mut best: Option<(Vec<usize>, usize)> = None;
        for i in 0..MAX_SIZE {
            if unused[i] {
                let mut result = list[i].find(unused, list, ord);
                let distance = result.1 + self.list[i];

                if best.is_none() {
                    best = Some((result.0, distance));
                } else if let Some((_, b)) = best {
                    if distance < b {
                        result.0.push(self.index);
                        println!("{}, {}", result.0.len(), remaining);
                        best = Some((result.0, distance));
                    }
                }
            }
        }

        best.unwrap_or((vec![self.index], 0))
    }
}

fn add_distance(a: usize, b: usize, list: &mut [Location], distance: usize) {
    let location = list.get_mut(a).unwrap();
    location.put_for_index(b, distance);
    let location = list.get_mut(b).unwrap();
    location.put_for_index(a, distance);
}

fn parse_str(str: &str) -> (String, String, usize) {
    let mut iter = str.split(' ');
    let a = iter.next().unwrap();
    iter.next();
    let b = iter.next().unwrap();
    iter.next();
    let distance: usize = iter.next().unwrap().parse().unwrap();

    (a.to_owned(), b.to_owned(), distance)
}

fn load_data_from_line(str: &str, map: &mut HashMap<String, usize>, list: &mut Vec<Location>) {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    let (a, b, distance) = parse_str(str);
    let a = if let Some(index) = map.get(&a) {
        *index
    } else {
        let index = COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let location = Location::new(index);
        list.push(location);
        map.insert(a, index);
        index
    };
    let b = if let Some(index) = map.get(&b) {
        *index
    } else {
        let index = COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let location = Location::new(index);
        list.push(location);
        map.insert(b, index);
        index
    };

    add_distance(a, b, list, distance);
}

fn main() {
    let mut map = HashMap::new();
    let mut list = Vec::with_capacity(MAX_SIZE);
    let input = include_str!("./input.txt");
    input
        .lines()
        .for_each(|line| load_data_from_line(line, &mut map, &mut list));

    // println!("{:?}", map);

    let result = list[0].find([true; MAX_SIZE], &list, Target::LOW);

    println!(
        "Shortest possible route: {:?} has distance {:?}",
        result.0, result.1
    );
}
