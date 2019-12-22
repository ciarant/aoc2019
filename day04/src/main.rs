const MIN: i32 = 136760;
const MAX: i32 = 595730;

fn check_incrementing(digits: &[u8]) -> bool {
    (1..=5).all(|i| digits[i] >= digits[i - 1])
}

fn check_adjacent(digits: &[u8]) -> bool {
    (0..5).any(|i| digits[i] == digits[i + 1])
}

fn check_exactly_two_adjacent(digits: &[u8]) -> bool {
    (0..5).any(|i| match i {
        0 => digits[0] == digits[1] && digits[0] != digits[2],
        4 => digits[4] == digits[5] && digits[4] != digits[3],
        n => {
            digits[n] == digits[n + 1] && (digits[n] != digits[n - 1] && digits[n] != digits[n + 2])
        }
    })
}

fn part1() -> i32 {
    let mut count = 0;

    for i in MIN..=MAX {
        let string = i.to_string(); // FIXME doing this to avoid "temporary value is freed at the end of this statement"
        let digits = string.as_bytes();
        if check_incrementing(digits) && check_adjacent(digits) {
            count += 1;
        }
    }
    count
}

fn part2() -> i32 {
    let mut count = 0;

    for i in MIN..=MAX {
        let string = i.to_string(); // FIXME doing this to avoid "temporary value is freed at the end of this statement"
        let digits = string.as_bytes();
        if check_incrementing(digits) && check_exactly_two_adjacent(digits) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_incrementing() {
        assert_eq!(check_incrementing(&[1, 1, 2, 2, 3, 3]), true);
        assert_eq!(check_incrementing(&[1, 1, 2, 2, 3, 2]), false);
    }

    #[test]
    fn test_check_adjacent() {
        assert_eq!(check_adjacent(&[1, 1, 2, 2, 3, 3]), true);
        assert_eq!(check_adjacent(&[1, 1, 1, 2, 3, 3]), true);
        assert_eq!(check_adjacent(&[1, 2, 3, 4, 5, 6]), false);
    }

    #[test]
    fn test_check_exactly_two_adjacent() {
        assert_eq!(check_exactly_two_adjacent(&[1, 1, 2, 2, 3, 3]), true);
        assert_eq!(check_exactly_two_adjacent(&[1, 1, 1, 2, 3, 4]), false);
        assert_eq!(check_exactly_two_adjacent(&[1, 2, 3, 4, 5, 6]), false);
    }

    #[test]
    fn test_check() {
        assert_eq!(check(112233), true);
        assert_eq!(check(111122), true);
        assert_eq!(check(123444), false);
    }
}
