use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::io;

#[derive(Clone, Eq)]
struct Point {
    x: i32,
    y: i32,
    step: i32,
}

impl Point {
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn read_wires(input: &str) -> (Vec<String>, Vec<String>) {
    let lines = input.lines().collect::<Vec<_>>();
    (
        lines[0]
            .split(',')
            .map(|segment| segment.to_string())
            .collect(),
        lines[1]
            .split(',')
            .map(|segment| segment.to_string())
            .collect(),
    )
}

fn trace(segments: Vec<String>) -> HashSet<Point> {
    let mut x = 0;
    let mut y = 0;
    let mut grid = HashSet::new();
    let mut step = 0;
    for segment in segments {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        let (direction, length) = segment.split_at(1);
        match direction {
            "U" => dy = 1,
            "D" => dy = -1,
            "L" => dx = -1,
            "R" => dx = 1,
            d => panic!("Unknown direction: {}", d),
        }
        for _ in 0..length.parse::<i32>().unwrap() {
            x += dx;
            y += dy;
            step += 1;
            grid.insert(Point { x, y, step });
        }
    }
    grid
}

fn part1(input: &str) -> i32 {
    let (wire1, wire2) = read_wires(input);
    let w1 = trace(wire1);
    let w2 = trace(wire2);

    w1.intersection(&w2).map(|p| p.manhattan()).min().unwrap()
}

fn part2(input: &str) -> i32 {
    let (wire1, wire2) = read_wires(input);
    let w1 = trace(wire1);
    let w2 = trace(wire2);

    w1.intersection(&w2)
        .collect::<Vec<_>>()
        .iter()
        .cloned()
        .map(|p| {
            let p1 = w1.get(p).unwrap();
            let p2 = w2.get(p).unwrap();
            p1.step + p2.step
        })
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part1(input), 159);
    }

    #[test]
    fn test1_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part1(input), 135);
    }

    #[test]
    fn test2_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part2(input), 610);
    }

    #[test]
    fn test2_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part2(input), 410);
    }
}
