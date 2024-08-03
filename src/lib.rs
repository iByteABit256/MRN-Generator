use chrono::{Datelike, Utc};
use rand::{distributions::Alphanumeric, prelude::Distribution};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MrnGeneratorError {
    #[error("{0} is not a valid country code, it should be exactly two characters (e.g. 'IT')")]
    CountryCodeLength(String),
    #[error("{0} is not a valid procedure category")]
    InvalidProcedureCategory(String),
    #[error("{procedure_category}-{combination} is not a valid procedure category combination")]
    InvalidProcedureCategoryCombination {
        procedure_category: String,
        combination: String,
    },
    #[error("{0} is not an alphanumeric")]
    NotAlphanumeric(char),
}

/// Returns a valid MRN given a country code
pub fn generate_random_mrn(
    country_code: &str,
    procedure: Option<Procedure>,
    declaration_office: Option<&str>,
) -> Result<String, MrnGeneratorError> {
    use MrnGeneratorError::*;

    let curr_year: String = Utc::now().year().to_string().chars().skip(2).collect();

    let random_str_len = 14 - declaration_office.map_or(0, |decoffice| decoffice.len());

    let random_str: String = Alphanumeric
        .sample_iter(&mut rand::thread_rng())
        .take(random_str_len)
        .map(|c| c.to_ascii_uppercase() as char)
        .collect();

    if country_code.len() != 2 {
        return Err(CountryCodeLength(country_code.to_string()));
    }

    let mut mrn = format!(
        "{}{}{}{}",
        curr_year,
        capitalize(country_code),
        declaration_office.unwrap_or(""),
        random_str
    );

    if let Some(procedure) = procedure {
        let proctgr_char = procecure_category_to_char(procedure).to_string();

        // Replace n-1 char with regime char
        mrn.replace_range(16..17, &proctgr_char);
    }

    // Check MRN, and replace last character if invalid
    let last_digit = is_mrn_valid(&mrn)?;

    if let Some(last_digit) = last_digit {
        Ok(replace_last_char(&mrn, last_digit))
    } else {
        Ok(mrn)
    }
}

/// Returns None if MRN is valid, and correct last character if it's invalid
pub fn is_mrn_valid(mrn: &str) -> Result<Option<char>, MrnGeneratorError> {
    let mut mrn_iter = mrn.chars();
    let last_digit = mrn_iter.next_back().unwrap();

    let mrn_temp: String = mrn_iter.collect();

    // Multiply each char value with it's power of 2 and sum them
    let multiplied_sum: u32 = mrn_temp
        .chars()
        .zip(0..mrn_temp.len())
        .map(|(c, m)| (check_character_value(c).map(|value| (value as u32) << m)))
        .collect::<Result<Vec<u32>, MrnGeneratorError>>()?
        .iter()
        .sum();

    let check_digit: u8 = (multiplied_sum % 11).try_into().unwrap();
    Ok(check_remainder_value(check_digit, last_digit))
}

/// Procedure types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Procedure {
    ExportOnly,
    ExportAndExitSummaryDeclaration,
    ExitSummaryDeclarationOnly,
    ReExportNotification,
    DispatchOfGoodsInRelationWithSpecialFiscalTerritories,
    TransitDeclarationOnly,
    TransitDeclarationAndExitSummaryDeclaration,
    TransitDeclarationAndEntrySummaryDeclaration,
    ProofOfTheCustomsStatusOfUnionGoods,
    ImportDeclarationOnly,
    ImportDeclarationAndEntrySummaryDeclaration,
    EntrySummaryDeclarationOnly,
    TemporaryStorageDeclaration,
    IntroductionOfGoodsInRelationWithSpecialFiscalTerritories,
    TemporaryStorageDeclarationAndEntrySummaryDeclaration,
}

/// Maps procedure category to a corresponding character
pub fn procecure_category_to_char(procedure: Procedure) -> char {
    match procedure {
        Procedure::ExportOnly => 'A',
        Procedure::ExportAndExitSummaryDeclaration => 'B',
        Procedure::ExitSummaryDeclarationOnly => 'C',
        Procedure::ReExportNotification => 'D',
        Procedure::DispatchOfGoodsInRelationWithSpecialFiscalTerritories => 'E',
        Procedure::TransitDeclarationOnly => 'J',
        Procedure::TransitDeclarationAndExitSummaryDeclaration => 'K',
        Procedure::TransitDeclarationAndEntrySummaryDeclaration => 'L',
        Procedure::ProofOfTheCustomsStatusOfUnionGoods => 'M',
        Procedure::ImportDeclarationOnly => 'R',
        Procedure::ImportDeclarationAndEntrySummaryDeclaration => 'S',
        Procedure::EntrySummaryDeclarationOnly => 'T',
        Procedure::TemporaryStorageDeclaration => 'U',
        Procedure::IntroductionOfGoodsInRelationWithSpecialFiscalTerritories => 'V',
        Procedure::TemporaryStorageDeclarationAndEntrySummaryDeclaration => 'W',
    }
}

/// Matches a procedure category code (optionally combined with another one) and returns
/// the corresponding customs procedure
pub fn match_procedure(
    proctgr: &str,
    combined: Option<&str>,
) -> Result<Procedure, MrnGeneratorError> {
    use MrnGeneratorError::*;

    let exit_combined = ["A"];
    let entry_combined = ["F"];
    match proctgr {
        "B1" | "B2" | "B3" | "C1" if combined.is_none() => Ok(Procedure::ExportOnly),
        "B1" | "B2" | "B3" | "C1" if combined.is_some_and(|c| exit_combined.contains(&c)) => {
            Ok(Procedure::ExportAndExitSummaryDeclaration)
        }
        "A1" | "A2" => Ok(Procedure::ExitSummaryDeclarationOnly),
        "A3" => Ok(Procedure::ReExportNotification),
        "B4" => Ok(Procedure::DispatchOfGoodsInRelationWithSpecialFiscalTerritories),
        "D1" | "D2" | "D3" if combined.is_none() => Ok(Procedure::TransitDeclarationOnly),
        "D1" | "D2" | "D3" if combined.is_some_and(|c| exit_combined.contains(&c)) => {
            Ok(Procedure::TransitDeclarationAndExitSummaryDeclaration)
        }
        "D1" | "D2" | "D3" if combined.is_some_and(|c| entry_combined.contains(&c)) => {
            Ok(Procedure::TransitDeclarationAndEntrySummaryDeclaration)
        }
        "E1" | "E2" => Ok(Procedure::ProofOfTheCustomsStatusOfUnionGoods),
        "H1" | "H2" | "H3" | "H4" | "H6" | "I1" if combined.is_none() => {
            Ok(Procedure::ImportDeclarationOnly)
        }
        "H1" | "H2" | "H3" | "H4" | "H6" | "I1"
            if combined.is_some_and(|c| entry_combined.contains(&c)) =>
        {
            Ok(Procedure::ImportDeclarationAndEntrySummaryDeclaration)
        }
        "F1a" | "F1b" | "F1c" | "F1d" | "F2a" | "F2b" | "F2c" | "F2d" | "F3a" | "F3b" | "F4a"
        | "F4b" | "F4c" | "F5" => Ok(Procedure::EntrySummaryDeclarationOnly),
        "H5" => Ok(Procedure::IntroductionOfGoodsInRelationWithSpecialFiscalTerritories),
        "G4" if combined.is_none() => Ok(Procedure::TemporaryStorageDeclaration),
        "G4" if combined.is_some_and(|c| entry_combined.contains(&c)) => {
            Ok(Procedure::TemporaryStorageDeclarationAndEntrySummaryDeclaration)
        }
        _ => {
            if let Some(c) = combined {
                Err(InvalidProcedureCategoryCombination {
                    procedure_category: proctgr.to_string(),
                    combination: c.to_string(),
                })
            } else {
                Err(InvalidProcedureCategory(proctgr.to_string()))
            }
        }
    }
}

/// Capitalizes string
pub fn capitalize(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// Replaces last character of string with new character
pub fn replace_last_char(s: &str, c: char) -> String {
    let mut new_str = s.to_string();
    new_str.pop();
    new_str.push(c);
    new_str
}

/// Remainder values according to tables in ISO 6346
pub fn check_remainder_value(check_digit: u8, last_digit: char) -> Option<char> {
    if check_digit % 10 != last_digit as u8 - 48 {
        char::from_digit((check_digit % 10) as u32, 10)
    } else {
        None
    }
}

/// Character values according to tables in ISO 6346
pub fn check_character_value(c: char) -> Result<u8, MrnGeneratorError> {
    if c.is_ascii_digit() {
        return Ok(c as u8 - 48);
    }
    if c.is_alphabetic() {
        if c == 'A' {
            return Ok(10);
        } else if ('B'..='K').contains(&c) {
            return Ok(c as u8 - 54);
        } else if ('L'..='U').contains(&c) {
            return Ok(c as u8 - 53);
        } else {
            return Ok(c as u8 - 52);
        }
    }

    // Default as fallback, change to an error sometime
    Err(MrnGeneratorError::NotAlphanumeric(c))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_random_mrn_test() {
        let mrn = generate_random_mrn("DK", Some(Procedure::ExportOnly), None).unwrap();

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        let procedure_char: char = mrn.chars().nth(16).unwrap();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!('A', procedure_char);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!(None, is_mrn_valid(&mrn).unwrap());
    }

    #[test]
    fn generate_random_mrn_test_without_procedure() {
        let mrn = generate_random_mrn("DK", None, None).unwrap();

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!(None, is_mrn_valid(&mrn).unwrap());
    }

    #[test]
    fn generate_random_mrn_test_with_declaration_office() {
        let mrn = generate_random_mrn("DK", None, Some("004700")).unwrap();

        let country_code: String = mrn.chars().skip(2).take(2).collect();
        let actual_year: String = mrn.chars().take(2).collect();
        let declaration_office: String = mrn.chars().skip(4).take(6).collect();
        let expected_year: String = Utc::now().year().to_string().chars().skip(2).collect();
        assert_eq!(18, mrn.len());
        assert_eq!(expected_year, actual_year);
        assert_eq!("DK".to_string(), country_code);
        assert_eq!("004700".to_string(), declaration_office);
        assert_eq!(None, is_mrn_valid(&mrn).unwrap());
    }

    #[test]
    fn is_mrn_valid_test() {
        assert_eq!(None, is_mrn_valid("22ITZXBZYUTJFLJXK6").unwrap());
        assert_eq!(Some('1'), is_mrn_valid("22DK1V0QQK2S6J7TU2").unwrap());
    }

    #[test]
    fn procedure_matched_test() {
        assert_eq!(Procedure::ExportOnly, match_procedure("B1", None).unwrap());
        assert_eq!(
            Procedure::ExportAndExitSummaryDeclaration,
            match_procedure("B2", Some("A")).unwrap()
        );
    }

    #[test]
    fn procedure_not_matched_test() {
        use MrnGeneratorError::*;

        assert_eq!(
            Err(InvalidProcedureCategoryCombination {
                procedure_category: "B2".to_string(),
                combination: "B".to_string()
            }),
            match_procedure("B2", Some("B"))
        );

        let invalid_procedure_category = "not a valid procedure 🤡";

        assert_eq!(
            Err(InvalidProcedureCategory(
                invalid_procedure_category.to_string()
            )),
            match_procedure(invalid_procedure_category, None)
        );
        assert_eq!(
            Err(InvalidProcedureCategoryCombination {
                procedure_category: invalid_procedure_category.to_string(),
                combination: "F".to_string()
            }),
            match_procedure(invalid_procedure_category, Some("F"))
        );
    }

    #[test]
    fn capitalize_test() {
        assert_eq!("BAT", capitalize("bat"))
    }

    #[test]
    fn replace_last_char_test() {
        assert_eq!("bar", replace_last_char("bat", 'r'))
    }

    #[test]
    fn check_remainder_value_test() {
        assert_eq!(None, check_remainder_value(3, '3'));
        assert_eq!(None, check_remainder_value(10, '0'));
        assert_eq!(Some('3'), check_remainder_value(3, '5'));
        assert_eq!(Some('0'), check_remainder_value(10, '9'));
    }

    #[test]
    fn check_character_value_test() {
        assert_eq!(3, check_character_value('3').unwrap());
        assert_eq!(10, check_character_value('A').unwrap());
        assert_eq!(13, check_character_value('C').unwrap());
        assert_eq!(35, check_character_value('W').unwrap());
        assert_eq!(
            Err(MrnGeneratorError::NotAlphanumeric('🤡')),
            check_character_value('🤡')
        );
    }
}
