use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let b1 = "{[()]}";
    let b2 = "{[(])}";
    let b3 = "{{[[(())]]}}";

    println!("{}", is_balanced(b1));
    println!("{}", is_balanced(b2));
    println!("{}", is_balanced(b3));

    let mut expected_results: Vec<String> = Vec::new();
    let tc_out_filename = "src/tc17.out";
    // Open the file in read-only mode (ignoring errors).
    let tc_out_file = File::open(tc_out_filename).unwrap();
    let out_reader = BufReader::new(tc_out_file);
    for line in out_reader.lines() {
        expected_results.push(line.unwrap());
    }

    let tc_in_filename = "src/tc17.in";
    // Open the file in read-only mode (ignoring errors).
    let tc_in_file = File::open(tc_in_filename).unwrap();
    let in_reader = BufReader::new(tc_in_file);

    let mut n: i32 = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (idx, line) in in_reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if idx == 0 {
            n = line.parse().unwrap(); // first line is the size of the test data input
        } else {
            let res = is_balanced(&line);
            if &res != (expected_results.get(idx - 1).unwrap()) {
                println!("Test case failed: line: {}, string: {}", idx, &line);
            }
        }
    }
}

fn is_balanced(bracket: &str) -> &str {
    // check if the brackets string length is even number, else return "NO"
    if bracket.len() % 2 == 0 {
        let mut opened_brackets: Vec<char> = Vec::new();
        for (idx, ch) in bracket.char_indices() {
            match ch {
                '(' => opened_brackets.push(')'),
                '[' => opened_brackets.push(']'),
                '{' => opened_brackets.push('}'),
                _ => {
                    if opened_brackets.is_empty() || opened_brackets.pop().unwrap() != ch {
                        return "NO"
                    }
                }
            }
        }
        return if opened_brackets.is_empty() { "YES" } else { "NO" };
    }

    "NO"
}
