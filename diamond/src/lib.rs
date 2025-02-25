pub struct Diamond {
    letter: char,
    alphabet: Vec<char>,
}

impl Diamond {
    pub fn new(letter: char) -> Self {
        Self {
            letter,
            alphabet: ('A'..='Z').collect(),
        }
    }

    pub fn create(&self) -> String {
        let mut letter_index = 0;

        while letter_index < self.alphabet.len()
            && self.letter.to_ascii_uppercase() != self.alphabet[letter_index]
        {
            letter_index += 1;
        }
        println!("Position of {} is {}", self.letter, letter_index);

        let mut rows = letter_index + 1;
        let mut columns = letter_index + 2;

        let mut initial_spaces = 0;
        let mut middle_spaces = 0;
        let mut ending_spaces = 0;

        println!("Diamond width is {columns}");
        for row in 0..rows {
            println!("************************");
            let mut buffer = "".to_string();
            let current_letter = self.alphabet[row];
            println!("Elaborating line {} with {}", row, current_letter);

            initial_spaces = letter_index - row;
            ending_spaces = initial_spaces;
            if row != 0 {
                middle_spaces = columns - 2 - initial_spaces - ending_spaces
            } else {
                middle_spaces = 0;
            }

            println!("Will have {initial_spaces} initial spaces");
            println!("Will have {middle_spaces} middle spaces");
            println!("Will have {ending_spaces} ending spaces");

            for column in 0..columns {}

            println!("Buffer is '{buffer}'");
        }
        "A".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_diamond() {
        let diamond = Diamond::new('a');

        let result = diamond.create();

        assert_eq!("A".to_string(), result);
    }

    #[test]
    fn b_diamond() {
        let diamond = Diamond::new('b');

        let result = diamond.create();

        let expected_result = r#" A 
B B
 A"#;

        assert_eq!(expected_result, result);
    }
}
