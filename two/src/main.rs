use std::str::FromStr;

fn lowest_two(a: i32, b: i32, c: i32) -> (i32,i32) {
    if a == i32::min(a, b) {
        (a, i32::min(b, c))
    } else {
        (b, i32::min(a, c))
    }
}

struct BOX {
    length: i32,
    width: i32,
    height: i32
}

impl FromStr for BOX {
    fn from_str(s: &str) -> std::result::Result<BOX, &'static str> {
        let (l,w,h);
        let mut itr = s.split("x");
        l = itr.next().expect("Too few values");
        w = itr.next().expect("Too few values");
        h = itr.next().expect("Too few values");

        if itr.next().is_some() {
            panic!("Too many values");
        } else {
            Ok(BOX{length: l.parse().unwrap(), width: w.parse().unwrap(), height: h.parse().unwrap()})
        }
    }

    type Err = &'static str;

}

impl BOX {
    fn required_paper(&self) -> i32 {
        let a = 2 * self.length * self.width;
        let b = 2 * self.width * self.height;
        let c = 2 * self.height * self.length;
        let smallest = i32::min(a, i32::min(b, c)) / 2;

        a + b + c + smallest
    }

    fn required_ribbon(&self) -> i32 {

        let (a, b) = lowest_two(self.length, self.width, self.height);

        a + a + b + b + (self.length * self.width * self.height)

    }
}

fn main() {
	
    let input = include_str!("./input.txt");
    let boxes: Vec<BOX> = input.lines()
        .map(|line| BOX::from_str(line).unwrap())
        .collect();

    let paper_needed : i32 = boxes.iter().map(|item| item.required_paper()).sum();
    let ribbon_needed : i32 = boxes.iter().map(|item| item.required_ribbon()).sum();

    println!("{} square feet of wrapping papper are needed", paper_needed);
    println!("{} feet of ribbon are required", ribbon_needed);

}
