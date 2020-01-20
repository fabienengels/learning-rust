fn evenly_divisible(year: u64, number: u64) -> bool {
    year % number == 0
}

pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_4 = evenly_divisible(year, 4);
    let divisible_by_100 = evenly_divisible(year, 100);
    let divisible_by_400 = evenly_divisible(year, 400);

    divisible_by_4 && (!divisible_by_100 || divisible_by_400)
}
