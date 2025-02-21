pub struct CodeCracker;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ENCRYPTED_ALPHABET: [char; 26] = [
    '!', ')', '"', '(', '£', '*', '%', '&', '>', '<', '@', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o',
];

impl CodeCracker {
    pub fn encode(&self, message: &str) -> String {
        let mut encoded_message = String::new();
        for char in message.chars() {
            let index = ALPHABET.into_iter().position(|x| x == char);
            match index {
                Some(position) => encoded_message.push(ENCRYPTED_ALPHABET[position]),
                None => encoded_message.push(char),
            }
        }
        encoded_message
    }

    pub fn decode(&self, message: &str) -> String {
        let mut decoded_message = String::new();
        for char in message.chars() {
            let index = ENCRYPTED_ALPHABET.into_iter().position(|x| x == char);
            match index {
                Some(position) => decoded_message.push(ALPHABET[position]),
                None => decoded_message.push(char),
            }
        }
        decoded_message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_all_chars() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.encode("a b c d e f g h i j k l m n o p q r s t u v w x y z");

        assert_eq!(
            "! ) \" ( £ * % & > < @ a b c d e f g h i j k l m n o",
            result
        );
    }

    #[test]
    fn encodes_a_message() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.encode("Matteo");

        assert_eq!("M!ii£d", result);
    }

    #[test]
    fn decodes_all_chars() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.decode("! ) \" ( £ * % & > < @ a b c d e f g h i j k l m n o");

        assert_eq!(
            "a b c d e f g h i j k l m n o p q r s t u v w x y z",
            result
        );
    }

    #[test]
    fn decodes_a_message() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.decode("M!ii£d@Pg>b! 1234");

        assert_eq!("MatteokPrima 1234", result);
    }
}
