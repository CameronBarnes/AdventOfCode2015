struct Lights {
    lights: [[bool; 1000]; 1000]
}

type Pos = (usize, usize);

impl Lights {
    fn new() -> Self {
        Lights{lights: [[false; 1000]; 1000]}
    }

    fn count(&self) -> usize {
        self.lights.iter()
            .map(|arr| 
                 arr.iter()
                    .filter(|item| **item).count()
                 ).sum()
    }

    fn set(&mut self, start: Pos, end: Pos, val: bool) {

        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                self.lights[x][y] = val;
            }
        }

    }

    fn toggle(&mut self, start: Pos, end: Pos) {
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                self.lights[x][y] = !self.lights[x][y];
            }
        }
    }

}

fn parse_pos_pair(str: &str) -> (Pos, Pos) {

    let mut iter = str.split(" through ");
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    let mut iter = first.split(',');
    let first = (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap());
    let mut iter = second.split(',');
    let second = (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap());

    (first, second)

}

fn main() {
	
    let mut lights = Lights::new();

    let input = include_str!("./input.txt");
    input.lines().for_each(|line| {
        if let Some(str) = line.strip_prefix("turn on ") {
            let (start, end) = parse_pos_pair(str);
            lights.set(start, end, true);
        } else if let Some(str) = line.strip_prefix("turn off ") {
            let (start, end) = parse_pos_pair(str);
            lights.set(start, end, false);
        } else if let Some(str) = line.strip_prefix("toggle ") {
            let (start, end) = parse_pos_pair(str);
            lights.toggle(start, end);
        }


    });

    println!("After all instructions {} lights are turned on", lights.count());

}
