fn is_a_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    message == message.to_uppercase()
}

fn has_letters(message: &str) -> bool {
    message.chars().map(|c| c.is_alphabetic()).any(|r| r)
}

pub fn reply(mut message: &str) -> &str {
    message = message.trim();

    if message.is_empty() {
        "Fine. Be that way!"
    } else if is_a_question(message) {
        if !has_letters(message) {
            "Sure."
        } else if is_yelling(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if !has_letters(message) {
        "Whatever."
    } else if is_yelling(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
