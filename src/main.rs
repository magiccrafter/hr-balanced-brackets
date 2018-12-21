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

fn is_balanced(bracket: &str) -> String {
    // can be improved using macro
    let mut closing_to_opening_brackets = HashMap::new();
    closing_to_opening_brackets.insert(')', '(');
    closing_to_opening_brackets.insert(']', '[');
    closing_to_opening_brackets.insert('}', '{');

    // check if the brackets string length is even number, else return "NO"
    if bracket.len() % 2 == 0 {
        let mut start_string_chars: Vec<char> = Vec::new();
        // find the index of the first closing bracket
        for (idx, ch) in bracket.char_indices() {
            if idx > 0 && closing_to_opening_brackets.contains_key(&ch) {
                let opening_bracket = closing_to_opening_brackets.get(&ch);
                if start_string_chars.last().unwrap() == opening_bracket.unwrap() {
                    start_string_chars.pop();
                    let reviewed_bracket = start_string_chars.iter().collect::<String>() + bracket.split_at(idx + 1).1;
                    if reviewed_bracket.is_empty() {
                        return String::from("YES");
                    } else {
                        return is_balanced(&reviewed_bracket);
                    }
                } else {
                    return String::from("NO");
                }
            }
            start_string_chars.push(ch);
        }
    }

    String::from("NO")
}
