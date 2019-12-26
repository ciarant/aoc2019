use std::collections::HashMap;
use std::io;
use std::io::Read;

fn orbits(input: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    input
        .lines()
        .map(|l| l.split(")").collect::<Vec<&str>>())
        .for_each(|o| {
            result.insert(o[1].to_string(), o[0].to_string());
        });
    result
}

// Compute the path from a starting point back to the the universal centre of mass
fn path(start: &str, orbits: &HashMap<String, String>) -> Vec<String> {
    let mut current = start;
    let mut path: Vec<String> = Vec::new();
    loop {
        match orbits.get(current) {
            Some(s)=> { current = s; path.push(s.to_string())}
            None => break
        };
    }
    path
}

fn count_all(orbits: &HashMap<String, String>) -> usize {
    orbits
        .keys()
        .fold(0usize, | total, v | total + path(v, orbits).len())
}

// Find the first point where two paths intersect
fn path_intersect(path1: &Vec<String>, path2: &Vec<String>) -> String {
    for s in path1 {
        if path2.contains(s) {
            return s.to_string();
        }
    }
    "".to_string() // FIXME do some more idiomatic. Maybe Result<String, Err>?
}

fn part1(orbits: &HashMap<String, String>) {
    println!("{}", count_all(orbits));
}

fn part2(orbits: &HashMap<String, String>) {
    let path1 = path("YOU", orbits);
    let path2 = path("SAN", orbits);
    let intersect = path_intersect(&path1, &path2);
    let index1 = path1.into_iter().position(|o| o == intersect).unwrap();
    let index2 = path2.into_iter().position(|o| o == intersect).unwrap();
    println!("{}", index1 + index2);
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let orbits = orbits(input.as_str());
    part1(&orbits);
    part2(&orbits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let orbits = orbits(s);
        assert_eq!(path("D", &orbits).len(), 3);
        assert_eq!(path("L", &orbits).len(), 7);
        assert_eq!(count_all(&orbits), 42);
    }

    #[test]
    fn test_intersect() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let orbits = orbits(s);
        let path1 = path("YOU", &orbits);
        let path2 = path("SAN", &orbits);
        assert_eq!(path_intersect(&path1, &path2), "D");
    }
}