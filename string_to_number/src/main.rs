fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::string_to_number;

    #[test]
    fn string_to_number_test() {
        assert_eq!(string_to_number("5678"), 5678);
        assert_eq!(string_to_number("-77"), -77);
    }
}
