use std::env;
use chrono::{Datelike,Utc};
use rand::{distributions::{Alphanumeric}, prelude::Distribution};
use mrn_generator::*;

/// Returns a valid MRN given a country code
fn generate_random_mrn(country_code: &str) -> String {
    let curr_year: String = Utc::now().year().to_string().chars().skip(2).collect();

    let random_str: String = Alphanumeric.sample_iter(&mut rand::thread_rng()).take(14)
                            .map(|c| c.to_ascii_uppercase() as char).collect();

    if country_code.len() != 2 { panic!("Country code should be 2 characters") }
    let mrn = format!("{}{}{}", curr_year, capitalize(country_code), random_str);

    // Check MRN, and replace last character if invalid
    let last_digit = is_mrn_valid(&mrn);
    if last_digit.is_none() { mrn } else { replace_last_char(&mrn, last_digit.unwrap()) }
}

/// Returns None if MRN is valid, and correct last character if it's invalid
fn is_mrn_valid(mrn: &str) -> Option<char> {
    let mut mrn_iter = mrn.chars();
    let last_digit = mrn_iter.next_back().unwrap();

    let mrn_temp: String = mrn_iter.collect();

    // Powers of 2
    let multipliers: Vec<u32> = (0..mrn_temp.len()).map(|n| 1 << n).collect();

    let multiplied_sum: u32 = mrn_temp.chars().zip(multipliers).map(|(c,m)| check_character_value(c) as u32 * m).sum();

    let check_digit: u8 = (multiplied_sum % 11).try_into().unwrap();
    if check_remainder_value(check_digit, last_digit) { None } else { Some(check_digit as char) }
}

fn main() {
    let country_code: &str = &env::args().skip(1).next().expect("Please provide a country code as argument. (e.g. DK)");

    let mrn: &str = &generate_random_mrn(country_code);
    println!("Here is your MRN: {}", mrn);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_random_mrn_test() {
        let mrn = generate_random_mrn("DK");

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!("DK".to_string(), country_code);
    }

    #[test]
    fn is_mrn_valid_test() {
        assert_eq!(None, is_mrn_valid("22ITZXBZYUTJFLJXK6"));
        assert_eq!(Some('\u{1}'), is_mrn_valid("22DK1V0QQK2S6J7TU2"));
    }

}
