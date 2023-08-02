//use im::HashMap;

//const MORSE_CODE:HashMap<String, String> = HashMap::new();

/*fn decode_morse(encoded: &str) -> String {
    encoded
        .split("   ")
        .map(
            |word| -> String {
                word
                    .split(' ')
                    .map(|y|
                        MORSE_CODE
                            .get(y)
                            .unwrap()
                            .to_string())
                    .collect::<String>() + " "
            }
        )
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}*/