/// Capitalizes string
pub fn capitalize(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// Replaces last character of string with new character
pub fn replace_last_char(s: &str, c: char) -> String {
    let mut new_str = s.to_string().clone();
    new_str.pop();
    new_str.push(c);
    new_str
}

/// Remainder values according to tables in ISO 6346
pub fn check_remainder_value(check_digit: u8, last_digit: char) -> Option<char> {
    if check_digit % 10 != last_digit as u8 - 48 { Some(char::from_digit((check_digit % 10) as u32, 10)).unwrap() } else { None }
}

/// Character values according to tables in ISO 6346
pub fn check_character_value(c: char) -> u8 {
    if c.is_ascii_digit() { return c as u8 - 48 }
    if c.is_alphabetic() {
        if c == 'A' { return  10 }
        else if 'B' <= c && c <= 'K' { return c as u8 - 54 }
        else if 'L' <= c && c <= 'U' { return c as u8 - 53 }
        else { return c as u8 - 52 }
    }
    return 0
}

#[cfg(test)]
mod tests {

    use super::*;

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