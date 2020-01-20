fn get_factors(n: u32) -> Vec<u32> {
    (1..n + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<u32>>()
}

pub fn raindrops(n: u32) -> String {
    let result = get_factors(n)
        .into_iter()
        .filter_map(|factor| match factor {
            3 => Some("Pling"),
            5 => Some("Plang"),
            7 => Some("Plong"),
            _ => None,
        })
        .collect::<Vec<&str>>()
        .join("");

    if result == "" {
        format!("{}", n)
    } else {
        String::from(result)
    }
}
