fn square_digits(num: u64) -> u64 {
  num.to_string().chars().map(|c| (c.to_digit(10).unwrap() as u64).pow(2)).collect::<Vec<u64>>().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<u64>().unwrap()
}

//Best answer
fn best_square_digits(num: u64) -> u64 {
    num
      .to_string()
      .chars()
      .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
      .collect::<String>()
      .parse()
      .expect("result isnt u64 parsable")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}