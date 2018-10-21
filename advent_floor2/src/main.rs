use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut input = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("File read failed");

    part1(&contents);
    part2(&contents);
}

// Split the input into number vectors, keep track of largest and smallest, and subtract them to add to a final sum.
fn part1(input: &str) {
    let mut sum = 0;

    for mut line in input.lines() {
        let mut numbers: Vec<_> = line.split("\t").map(|number| number.parse::<usize>().unwrap()).collect();

        let mut smallest = numbers[0];
        let mut largest = 0;
        for num in numbers {
            if smallest > num {
                smallest = num;
            }
            if largest < num {
                largest = num;
            }
        }
        sum += largest - smallest;
    }
    println!("{:?}", sum);
}

// Go through the lines, sort the vector by descending (because a smaller number can't cleanly mod a larger number), and find the two evenly divisable numbers.
fn part2(input: &str) {
    let mut sum = 0;

    for mut line in input.lines() {
        let mut numbers: Vec<_> = line.split("\t").map(|number| number.parse::<usize>().unwrap()).collect();

        numbers.sort_by(|a, b| b.cmp(a));

        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if numbers[i] != numbers[j] && numbers[i] % numbers[j] == 0 {
                    sum += numbers[i] / numbers[j];
                }
            }
        }
    }
    println!("{:?}", sum);
}
