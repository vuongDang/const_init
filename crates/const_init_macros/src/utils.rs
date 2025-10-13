/// Convert struct camel case name into screaming snake for  constants
pub(crate) fn camelcase_to_screaming_snake(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.char_indices() {
        if i == 0 {
            result.push(ch.to_ascii_uppercase());
            continue;
        }
        if ch.is_uppercase() {
            // If current letter is upper and next is lower
            if let Some(next_ch) = s.chars().nth(i + 1) {
                if next_ch.is_lowercase() {
                    result.push('_');
                }
            } else if
            // If the last letter is uppercase and the one before is lowercase
            s.chars().nth(i - 1).unwrap().is_lowercase() {
                result.push('_');
            }
            result.push(ch);
        } else {
            result.push(ch.to_ascii_uppercase());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camelcase_to_screaming_snake_is_correct() {
        let testcases = [
            "MyStruct",
            "CAPITALStruct",
            "fooBar",
            "Onewordstruct",
            "EndWithUppeR",
        ];
        let expected = [
            "MY_STRUCT",
            "CAPITAL_STRUCT",
            "FOO_BAR",
            "ONEWORDSTRUCT",
            "END_WITH_UPPE_R",
        ];

        for (test, expect) in testcases.iter().zip(expected) {
            assert_eq!(camelcase_to_screaming_snake(test), expect);
        }
    }
}
