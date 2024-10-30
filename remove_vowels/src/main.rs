fn disemvowel(s: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars().filter(|c| !vowels.contains(&c)).collect()
}

// Best anwer
fn rmvowels(s: &str) -> String {
    s.chars()
        .filter(|c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    use super::rmvowels;

    #[test]
    fn disemvowel_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }

    #[test]
    fn rmvowels_test() {
        assert_eq!(
            rmvowels("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        )
    }
}

fn main() {
    todo!();
}
