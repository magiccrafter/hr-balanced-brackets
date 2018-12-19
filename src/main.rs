use std::collections::HashMap;

fn main() {

    let b1 = "{[()]}";
    let b2 = "{[(])}";
    let b3 = "{{[[(())]]}}";

    println!("{}", is_balanced(b1));
    println!("{}", is_balanced(b2));
    println!("{}", is_balanced(b3));
}

fn is_balanced(bracket: &str) -> &str {
    // can be improved using macro
    let mut closing_to_opening_brackets = HashMap::new();
    closing_to_opening_brackets.insert(')', '(');
    closing_to_opening_brackets.insert(']', '[');
    closing_to_opening_brackets.insert('}', '{');

    // check if the brackets string length is even number, else return "NO"
    if bracket.len() % 2 == 0 {
        // find the index of the first closing bracket
        for (idx, ch) in bracket.char_indices() {
            if closing_to_opening_brackets.contains_key(&ch) {
                // get tuple of substrings for both opening & closing brackets
                let (opening_part, closing_part) = bracket.split_at(idx);
                let closing_part_rev = closing_part.chars()
                    .rev()
                    .map(|ch| closing_to_opening_brackets.get(&ch).unwrap())
                    .collect::<String>();
                // compare the opening brackets substring with the closing brackets substring (reversed)
                if opening_part == closing_part_rev {
                    return "YES"
                } else {
                    return "NO"
                }
            }
        }
    }

    "NO"
}
