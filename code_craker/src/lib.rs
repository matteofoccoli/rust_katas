pub struct CodeCracker;

impl CodeCracker {
    pub fn encrypt(&self, _: &str) -> &str {
        "!"
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
}
