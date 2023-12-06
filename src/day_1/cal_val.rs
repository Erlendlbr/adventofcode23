use std::fs;

pub fn calc_inp() {
    let file1 = "docs/1/star1.txt";
    let file2 = "docs/1/star2.txt";

    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let accum: u32 = contents.split('\n').map(|line| get_line_nr(line)).sum();

    println!("accuum: {:?}", accum);

    let contents = fs::read_to_string(file2).expect("Should have been able to read the file");

    let correctrd_accum: u32 = contents
        .split('\n')
        .map(|line| get_line_nr(tranform_line(line).as_str()))
        .sum();

    println!("Corrected accuum: {:?}", correctrd_accum);
}

fn get_line_nr(line: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut first = ' ';
    let mut last = ' ';
    for c in line.chars() {
        match c.to_digit(RADIX) {
            Some(_) => {
                if first == ' ' {
                    first = c;
                }
                last = c;
            }
            _ => (),
        }
    }
    // println!("First: {first}; Last: {last}");
    (first.to_digit(RADIX).unwrap() * 10) + last.to_digit(RADIX).unwrap()
}

fn tranform_line(line: &str) -> String {
    let ret = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    ret
}
