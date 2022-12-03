pub fn init(day: u8) {
    println!("---------- DAY {:02}", day);
}

pub fn output<T, Z>(first_part: T, second_part: Z)
where
    T: std::fmt::Debug,
    Z: std::fmt::Debug,
{
    println!("First part:  {:?}", first_part);
    println!("Second part: {:?}", second_part);
    println!();
}
