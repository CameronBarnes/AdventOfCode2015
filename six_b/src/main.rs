struct Lights {
    lights: [[i32; 1000]; 1000]
}

type Pos = (usize, usize);

impl Lights {
    fn new() -> Self {
        Lights{lights: [[0; 1000]; 1000]}
    }

    fn count(&self) -> i32 {
        self.lights.iter().flat_map(|item| item.iter()).sum()
    }

    fn add(&mut self, start: Pos, end: Pos, val: i32) {

        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                self.lights[x][y] += val;
                if self.lights[x][y] < 0 {
                    self.lights[x][y] = 0;
                }
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
            lights.add(start, end, 1);
        } else if let Some(str) = line.strip_prefix("turn off ") {
            let (start, end) = parse_pos_pair(str);
            lights.add(start, end, -1);
        } else if let Some(str) = line.strip_prefix("toggle ") {
            let (start, end) = parse_pos_pair(str);
            lights.add(start, end, 2);
        }


    });

    println!("Total Brightness is {}", lights.count());

}
