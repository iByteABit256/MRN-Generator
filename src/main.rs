mod parser;

use std::error::Error;

use chrono::{Datelike, Utc};
use clap::Parser;
use mrn_generator::*;
use parser::Args;
use rand::{distributions::Alphanumeric, prelude::Distribution};

/// Returns a valid MRN given a country code
fn generate_random_mrn(country_code: &str, procedure: Option<Procedure>, declaration_office: Option<&str>) -> String {
    let curr_year: String = Utc::now().year().to_string().chars().skip(2).collect();

    let random_str_len = 14 - declaration_office.map_or(0, |decoffice| decoffice.len());

    let random_str: String = Alphanumeric
        .sample_iter(&mut rand::thread_rng())
        .take(random_str_len)
        .map(|c| c.to_ascii_uppercase() as char)
        .collect();

    if country_code.len() != 2 {
        panic!("Country code should be 2 characters")
    }

    let mut mrn = format!("{}{}{}{}", curr_year, capitalize(country_code), declaration_office.unwrap_or(""), random_str);

    if let Some(procedure) = procedure {
        let proctgr_char = procecure_category_to_char(procedure).to_string();

        // Replace n-1 char with regime char
        mrn.replace_range(16..17, &proctgr_char);
    }

    // Check MRN, and replace last character if invalid
    let last_digit = is_mrn_valid(&mrn);

    if let Some(last_digit) = last_digit {
        replace_last_char(&mrn, last_digit)
    } else {
        mrn
    }
}

/// Returns None if MRN is valid, and correct last character if it's invalid
fn is_mrn_valid(mrn: &str) -> Option<char> {
    let mut mrn_iter = mrn.chars();
    let last_digit = mrn_iter.next_back().unwrap();

    let mrn_temp: String = mrn_iter.collect();

    // Multiply each char value with it's power of 2 and sum them
    let multiplied_sum: u32 = mrn_temp
        .chars()
        .zip(0..mrn_temp.len())
        .map(|(c, m)| (check_character_value(c) as u32) << m)
        .sum();

    let check_digit: u8 = (multiplied_sum % 11).try_into().unwrap();
    check_remainder_value(check_digit, last_digit)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let country_code = args.country_code;
    let iterations = args.number_of_mrns;
    let declaration_office = args.declaration_office.as_deref();

    let combined = args.combined;
    let procedure = args.procedure_category.map(|proctg| match_procedure(&proctg, combined.as_deref()));

    for _ in 0..iterations {
        let mrn: &str = &generate_random_mrn(&country_code, procedure, declaration_office);
        println!("{mrn}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_random_mrn_test() {
        let mrn = generate_random_mrn("DK", Some(Procedure::ExportOnly), None);

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        let procedure_char: char = mrn.chars().nth(16).unwrap();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!('A', procedure_char);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!(None, is_mrn_valid(&mrn));
    }

    #[test]
    fn generate_random_mrn_test_without_procedure() {
        let mrn = generate_random_mrn("DK", None, None);

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!(None, is_mrn_valid(&mrn));
    }

    #[test]
    fn generate_random_mrn_test_with_declaration_office() {
        let mrn = generate_random_mrn("DK", None, Some("004700"));

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let declaration_office: String = mrn.chars().skip(4).take(6).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!("004700".to_string(), declaration_office);
        assert_eq!(None, is_mrn_valid(&mrn));
    }

    #[test]
    fn is_mrn_valid_test() {
        assert_eq!(None, is_mrn_valid("22ITZXBZYUTJFLJXK6"));
        assert_eq!(Some('1'), is_mrn_valid("22DK1V0QQK2S6J7TU2"));
    }
}
