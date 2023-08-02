fn alphabet_position(text: &str) -> String {
    let chars = text.chars();
    let mut out = String::new();
    for c in chars{
        //if c is a letter push it's alphabetical position
        if c.is_alphabetic(){
            //push the Alphabetical position of the char
            out.push_str({
                        //the value of A is 65 so for each letter subtract 64 to get it's position
                        u32::from(c.to_ascii_uppercase())-64
                    }
                    .to_string()
                    .as_str()
                );
            //separate numbers with spaces
            out.push(' ');
        }
    }
    //remove last space and recast to string
    out.trim().to_string()
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}