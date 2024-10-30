use core::f32;

fn bmi(weight: u32, height: f32) -> &'static str {
    match (weight as f32) / height.powf(2.0) {
        n if n >= 30.0 => "Obese",
        n if n >= 25.1 => "Overweight",
        n if n > 18.5 => "Normal",
        n if n <= 18.5 => "Underweight",
        _ => "None",
    }
}

fn main() {
    println!("Juan\'s BMI: {}", bmi(74, 1.72));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}
