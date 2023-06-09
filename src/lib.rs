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
pub fn match_procedure(proctgr: &str, combined: Option<&str>) -> Procedure {
    let exit_combined = vec!["A"];
    let entry_combined = vec!["F"];
    match proctgr {
        "B1" | "B2" | "B3" | "C1" if combined.is_none() => Procedure::ExportOnly,
        "B1" | "B2" | "B3" | "C1" if combined.is_some_and(|c| exit_combined.contains(&c)) => {
            Procedure::ExportAndExitSummaryDeclaration
        }
        "A1" | "A2" => Procedure::ExitSummaryDeclarationOnly,
        "A3" => Procedure::ReExportNotification,
        "B4" => Procedure::DispatchOfGoodsInRelationWithSpecialFiscalTerritories,
        "D1" | "D2" | "D3" if combined.is_none() => Procedure::TransitDeclarationOnly,
        "D1" | "D2" | "D3" if combined.is_some_and(|c| exit_combined.contains(&c)) => {
            Procedure::TransitDeclarationAndExitSummaryDeclaration
        }
        "D1" | "D2" | "D3" if combined.is_some_and(|c| entry_combined.contains(&c)) => {
            Procedure::TransitDeclarationAndEntrySummaryDeclaration
        }
        "E1" | "E2" => Procedure::ProofOfTheCustomsStatusOfUnionGoods,
        "H1" | "H2" | "H3" | "H4" | "H6" | "I1" if combined.is_none() => {
            Procedure::ImportDeclarationOnly
        }
        "H1" | "H2" | "H3" | "H4" | "H6" | "I1"
            if combined.is_some_and(|c| entry_combined.contains(&c)) =>
        {
            Procedure::ImportDeclarationAndEntrySummaryDeclaration
        }
        "F1a" | "F1b" | "F1c" | "F1d" | "F2a" | "F2b" | "F2c" | "F2d" | "F3a" | "F3b" | "F4a"
        | "F4b" | "F4c" | "F5" => Procedure::EntrySummaryDeclarationOnly,
        "H5" => Procedure::IntroductionOfGoodsInRelationWithSpecialFiscalTerritories,
        "G4" if combined.is_none() => Procedure::TemporaryStorageDeclaration,
        "G4" if combined.is_some_and(|c| entry_combined.contains(&c)) => {
            Procedure::TemporaryStorageDeclarationAndEntrySummaryDeclaration
        }
        _ => {
            if let Some(c) = combined {
                panic!("{proctgr} combined with {c} is not a valid combined procedure category.")
            } else {
                panic!("{proctgr} is not a valid procedure category.")
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
        Some(char::from_digit((check_digit % 10) as u32, 10)).unwrap()
    } else {
        None
    }
}

/// Character values according to tables in ISO 6346
pub fn check_character_value(c: char) -> u8 {
    if c.is_ascii_digit() {
        return c as u8 - 48;
    }
    if c.is_alphabetic() {
        if c == 'A' {
            return 10;
        } else if ('B'..='K').contains(&c) {
            return c as u8 - 54;
        } else if ('L'..='U').contains(&c) {
            return c as u8 - 53;
        } else {
            return c as u8 - 52;
        }
    }

    // Default as fallback, change to an error sometime
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn procedure_matched_test() {
        assert_eq!(Procedure::ExportOnly, match_procedure("B1", None));
        assert_eq!(
            Procedure::ExportAndExitSummaryDeclaration,
            match_procedure("B2", Some("A"))
        );
    }

    #[test]
    #[should_panic]
    fn procedure_not_matched_test() {
        match_procedure("B2", Some("B"));
        match_procedure("clearly not a valid procedure 🤡", None);
        match_procedure("clearly not a valid procedure 🤡", Some("F"));
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
        assert_eq!(3, check_character_value('3'));
        assert_eq!(10, check_character_value('A'));
        assert_eq!(13, check_character_value('C'));
        assert_eq!(35, check_character_value('W'));
    }
}
