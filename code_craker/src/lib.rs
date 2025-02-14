pub struct CodeCracker;

impl CodeCracker {
    pub fn encrypt(&self, value: &str) -> String {
        let decrypted = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let encrypted = [
            '!', ')', '"', '(', '£', '*', '%', '&', '>', '<', '@', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        ];
        let mut result = String::new();
        for char in value.chars() {
            let position = decrypted.into_iter().position(|x| x == char);
            match position {
                Some(position) => result.push(encrypted[position]),
                None => result.push(char),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypts_one_char() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.encrypt("a");

        assert_eq!("!", result);
    }

    #[test]
    fn encrypts_two_chars() {
        let code_cracker = CodeCracker {};

        let result = code_cracker.encrypt("ab");

        assert_eq!("!)", result);
    }
}
