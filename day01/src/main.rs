use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel += calc_fuel(mass);
    }
    writeln!(io::stdout(), "{}", fuel)?;
    Ok(())
}

fn calc_fuel(mass: i32) -> i32 {
    let f = mass / 3 - 2;
    if f > 0 { f } else { 0 }
}

fn part2(input: &str) -> Result<()> {
    let mut fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel += calc_fuel2(mass);
    }
    writeln!(io::stdout(), "{}", fuel)?;
    Ok(())
}

fn calc_fuel2(mass: i32) -> i32 {
    let mut fuel = calc_fuel(mass);
    if fuel > 0 {
        fuel += calc_fuel2(fuel);
    }
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc_fuel2(14), 2);
        assert_eq!(calc_fuel2(1969), 966);
        assert_eq!(calc_fuel2(100756), 50346);
    }
}
