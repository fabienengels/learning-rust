fn is_prime_number(number: u32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut number: u32 = 1;
    let mut index: u32 = 0;

    loop {
        number += 1;

        if is_prime_number(number) {
            if index == n {
                return number;
            }

            index += 1;
        }
    }
}
