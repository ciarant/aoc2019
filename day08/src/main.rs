use std::io;
use std::io::Read;

fn part1(input: &str) -> usize{
    let min_zero_layer= input.as_bytes()
        .chunks(25 * 6)
        .min_by_key(|&l| l.iter().filter(|&c| *c == b'0').count())
        .unwrap();
    let num_ones = min_zero_layer
        .iter()
        .filter(|&x| *x == b'1')
        .count();
    let num_twos = min_zero_layer
        .iter()
        .filter(|&x| *x == b'2')
        .count();

    num_ones * num_twos
}

fn part2(input: &str) -> Vec<&str> {
    let layers: Vec<_> = input.as_bytes()
        .chunks(25 * 6)
        .collect();
    let mut image = vec![" "; 25 * 6];
    for layer in layers.iter().rev() {
        for i in 0..layer.len() {
            match layer[i] {
                b'0' => image[i] = " ",
                b'1' => image[i] = "*",
                _ => ()
            }
        }
    }
    image
}

fn print_image(image: Vec<&str>) {
    let mut offset = 0;
    for _ in 0..6 {
        for _ in 0..25 {
            print!("{}", image[offset]);
            offset += 1;
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    print_image(part2(&input));
}
