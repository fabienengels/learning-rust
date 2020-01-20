pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut queue: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '[' => queue.push(c),
            '{' => queue.push(c),
            '(' => queue.push(c),
            ']' => match queue.pop() {
                Some(opening_char) => {
                    if opening_char != '[' {
                        return false;
                    }
                }
                None => return false,
            },
            '}' => match queue.pop() {
                Some(opening_char) => {
                    if opening_char != '{' {
                        return false;
                    }
                }
                None => return false,
            },
            ')' => match queue.pop() {
                Some(opening_char) => {
                    if opening_char != '(' {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {}
        }
    }

    queue.is_empty()
}
